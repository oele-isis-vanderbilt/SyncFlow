/* eslint-disable */
'use client';
import { useState } from 'react';
import { useRouter } from 'next/navigation';
import Select from 'react-select';
import { customClassNames } from '@/app/ui/dashboard/rooms/widgets/utils';
import { videoCodecs, VideoPresets, AudioPresets } from 'livekit-client';
import { Modal } from 'flowbite-react';
import { lusitana } from '../../fonts';
import { set } from 'zod';

export default function SessionJoinForm({
  show,
  onClose,
  projectId,
  sessionId,
  sessionName,
}: {
  show: boolean;
  onClose: () => void;
  projectId: string;
  sessionId: string;
  sessionName: string;
}) {
  const router = useRouter();
  const [identity, setIdentity] = useState('');

  let videoCodecOptions = videoCodecs.map((codec) => {
    return { label: codec.toUpperCase(), value: codec };
  });

  let [selectedVideoCodec, setSelectedVideoCodec] = useState('h264');

  let videoPresetOptions = Object.keys(VideoPresets).map((preset) => {
    return { label: preset.toUpperCase(), value: preset };
  });

  let [selectedVideoPreset, setSelectedVideoPreset] = useState('h1080');

  let audioPresetOptions = Object.keys(AudioPresets).map((preset) => {
    return { label: preset.toUpperCase(), value: preset };
  });

  let [selectedAudioPreset, setSelectedAudioPreset] = useState('speech');

  const onCloseFn = () => {
    setIdentity('');
    setSelectedVideoCodec('h264');
    setSelectedVideoPreset('h1080');
    setSelectedAudioPreset('speech');
    onClose();
  };

  return (
    <Modal size="7xl" show={show} onClose={onCloseFn}>
      <Modal.Header className={`${lusitana.className} text-4xl font-bold`}>
        Please select an identity and data sharing options for session{' '}
        {sessionName}
      </Modal.Header>
      <Modal.Body>
        <div className={'hidden w-full text-center md:block dark:text-white'}>
          <div className={'p-2'}>
            <h2 className={'text-xl font-bold'}>Data Sharing Options</h2>
            <div
              className={
                'flex h-full w-full flex-row items-center justify-center p-2'
              }
            >
              <h3 className={'m-2 w-1/3 text-lg'}>
                Default Video Codec
                <InformationLink
                  href={'https://docs.livekit.io/guides/video-codecs/'}
                />
                :
              </h3>
              <div className={'flex-1'}>
                <SingleSelect
                  options={videoCodecOptions}
                  value={selectedVideoCodec}
                  onChange={setSelectedVideoCodec}
                  placeholder="Select default Video Codec"
                />
              </div>
            </div>

            <div className={'flex flex-row items-center justify-center p-2'}>
              <h3 className={'m-2 w-1/3 text-lg'}>
                Default Video Preset
                <InformationLink
                  href={
                    'https://docs.livekit.io/client-sdk-js/variables/VideoPresets.html'
                  }
                />
                :
              </h3>
              <div className={'flex-1'}>
                <SingleSelect
                  options={videoPresetOptions}
                  value={selectedVideoPreset}
                  onChange={setSelectedVideoPreset}
                  placeholder="Select default Video Preset"
                />
              </div>
            </div>

            <div className={'flex flex-row items-center justify-center p-2'}>
              <h3 className={'m-2 w-1/3 text-lg'}>
                Default Audio Preset
                <InformationLink
                  href={
                    'https://docs.livekit.io/client-sdk-js/modules/AudioPresets.html'
                  }
                />
                :
              </h3>
              <div className={'flex-1'}>
                <SingleSelect
                  options={audioPresetOptions}
                  value={selectedAudioPreset}
                  onChange={setSelectedAudioPreset}
                  placeholder="Select default Video Preset"
                />
              </div>
            </div>
          </div>
          <hr className={'my-5'} />
          <div className={'p-2'}>
            <label htmlFor="identity">Enter the Identity</label>
          </div>
          <div className={'p-2'}>
            <input
              id="identity"
              type="text"
              value={identity}
              onChange={(e) => setIdentity(e.target.value)}
              className={'p-2 text-black'}
            />
          </div>
        </div>
      </Modal.Body>
      <Modal.Footer>
        {identity && (
          <button
            className={'bg-blue-500 p-2 text-white'}
            onClick={() => {
              router.push(
                `/dashboard/projects/${projectId}/sessions/${sessionId}/share?identity=${identity}&videoCodec=${selectedVideoCodec}&videoPreset=${selectedVideoPreset}&audioPreset=${selectedAudioPreset}`,
              );
            }}
          >
            Join Session
          </button>
        )}
        <button
          className={'bg-red-500 p-2 text-white'}
          onClick={() => onClose()}
        >
          Close
        </button>
      </Modal.Footer>
    </Modal>
  );
}

function SingleSelect({
  options,
  value,
  onChange,
  placeholder,
}: {
  options: { value: string; label: string }[];
  value?: string | undefined;
  onChange: (value: string) => void;
  placeholder: string;
}) {
  return (
    <Select
      classNames={customClassNames}
      options={options}
      value={options.find((option) => option.value === value)}
      onChange={(option) => {
        // @ts-ignore
        onChange(option?.value);
      }}
      placeholder={placeholder}
    />
  );
}

function InformationLink({ href }: { href: string }) {
  return (
    <a className={'text-red-900'} href={href} target={'_blank'}>
      {' '}
      &#8505;{' '}
    </a>
  );
}
