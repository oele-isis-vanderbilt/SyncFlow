/* eslint-disable */
'use client';
import { useState } from 'react';
import { useRouter } from 'next/navigation';
import Select from 'react-select';
import { customSelectStyles } from '@/app/utils';
import { videoCodecs, VideoPresets, AudioPresets } from 'livekit-client';

export default function RoomJoinForm({ roomNames }: { roomNames: string[] }) {
  if (roomNames.length === 0) {
    return (
      <div className={'p-2'}>
        <p>
          No rooms available. Refresh this page to see if new rooms are
          available
        </p>
      </div>
    );
  }

  const router = useRouter();
  const [selectedRoom, setSelectedRoom] = useState(roomNames[0]);
  const [identity, setIdentity] = useState('');

  let roomNameOptions = roomNames.map((name) => {
    return { value: name, label: name };
  });

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

  return (
    <>
      <div className={'hidden w-1/2 text-center md:block'}>
        <hr className={'my-5'} />
        <div className={'p-2'}>
          <h2 className={'text-xl font-bold'}>Select room to join</h2>
        </div>
        <div className={'items-center p-2 text-center'}>
          <SingleSelect
            options={roomNameOptions}
            value={selectedRoom}
            onChange={(value) => {
              // @ts-ignore
              setSelectedRoom(value);
            }}
            placeholder="select a room"
          />
        </div>
        <hr className={'my-5'} />

        <div className={'p-2'}>
          <h2 className={'text-xl font-bold'}>Room Options</h2>
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
          <label htmlFor="identity">Enter your identity</label>
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
        {selectedRoom && identity && (
          <button
            className={'bg-blue-500 p-2 text-white'}
            onClick={() => {
              router.push(
                `/room?name=${selectedRoom}&identity=${identity}&videoCodec=${selectedVideoCodec}&videoPreset=${selectedVideoPreset}&audioPreset=${selectedAudioPreset}`,
              );
            }}
          >
            Join Room
          </button>
        )}
      </div>
      <div className={'block text-center italic text-red-500 md:hidden'}>
        This page is not optimized for mobile. Please use a desktop browser.
      </div>
    </>
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
      styles={customSelectStyles}
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
