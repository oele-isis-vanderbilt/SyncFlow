'use client';
import '@livekit/components-styles';
import VideoGallery from '@/app/ui/dashboard/rooms/video-gallery';
import { Modal } from 'flowbite-react';
import { getHelpText } from '@/app/ui/dashboard/rooms/utils';
import {
  LiveKitRoom,
  RoomName,
  ControlBar,
  useRoomInfo,
  useRoomContext,
  useTracks,
} from '@livekit/components-react';

import Select from 'react-select';
import makeAnimated from 'react-select/animated';

import { useState } from 'react';
import { Tooltip } from 'flowbite-react';

import AudioStreams from '@/app/ui/dashboard/rooms/audio-streams';

import { lusitana } from '@/app/ui/fonts';
import { redirectToDashboard } from '@/app/lib/actions';
import { BsRecordBtn } from 'react-icons/bs';
import clsx from 'clsx';
import { Button } from '@/app/ui/button';
import {useFormState, useFormStatus} from "react-dom";

export default function Room({ token }: { token: string }) {
  return (
    <LiveKitRoom
      video={false}
      audio={false}
      serverUrl={process.env.NEXT_PUBLIC_LIVEKIT_SERVER_URL}
      token={token}
      className={`h-full w-full`}
      onDisconnected={() => {
        redirectToDashboard();
      }}
    >
      <div className="flex h-full w-full flex-col bg-black">
        <TopBar />
        <div className="flex h-full w-full flex-row bg-black">
          <div className="h-full w-1/2 flex-1">
            <VideoGallery title={'Videos'} />
          </div>
          <div className="h-full w-1/2">
            <div className="flex h-full w-full flex-col">
              <div className="h-1/2 w-full bg-white">
                <AudioStreams title={'Audio Streams'} />
              </div>
              <div className="h-1/2 w-full">Remaining content</div>
            </div>
          </div>
        </div>
      </div>
    </LiveKitRoom>
  );
}

function TopBar() {
  return (
    <div className="flex h-20 w-full items-center justify-between bg-black py-2 md:py-5">
      <div className="w-1/2">
        <RoomName className={`${lusitana.className} p-2 text-xl md:text-2xl`} />
      </div>
      <div className="flex w-1/2 items-center justify-end gap-4">
        <ControlBar
          controls={{
            microphone: false,
            camera: true,
            chat: false,
            screenShare: true,
            leave: true,
          }}
          className={`flex text-xl`}
          variation={'verbose'}
        />
        <RoomRecorder />
      </div>
    </div>
  );
}

export function RoomRecorder() {
  const roomInfo = useRoomContext();
  const tracks = useTracks();
  const {pending} = useFormStatus();

  function logData(prevState: { selectedOptions: any[] },
  formData: { selectedOptions: any[]}) {
    'use server';
    console.log(formData);
    return {
      text: 'Recording',
    };
  }
  const [errorMessage, dispatch] = useFormState(logData, {selectedOptions: []});

  const [openModal, setOpenModal] = useState(false);

  const trackSelections = tracks.map((track) => {
    return {
      label: getHelpText(track),
      value: track.publication?.trackSid!,
    };
  });


  const animatedComponents = makeAnimated();

  const [selections, setSelections] = useState<
    { label: string; value: string }[]
  >([]);

  const [text, setText] = useState('Start Recording');

  const handleSelectedChange = (option) => {
    setSelections(option);
  };

  const selectAll = () => {
    setSelections(trackSelections);
  };

  const clearAll = () => {
    setSelections([]);
  };

  const isAllSelected = selections.length === trackSelections.length;

  return (
    <Tooltip
      content={roomInfo.isRecording ? 'Stop Recording' : 'Start Recording'}
      className={tracks.length > 0 ? 'block' : 'hidden'}
    >
      <BsRecordBtn
        className={clsx(
          roomInfo.isRecording ? 'text-red-500' : 'text-white',
          'cursor-pointer text-4xl transition-colors duration-300 ease-in-out hover:text-red-500',
        )}
        onClick={() => {
          setOpenModal(true);
        }}
      />
      <Modal
        dismissible
        size="7xl"
        show={openModal}
        onClose={() => setOpenModal(false)}
      >
        <Modal.Header>Record Room ({roomInfo.name})</Modal.Header>
        <Modal.Body>
          <div className="min-h-96 w-full space-y-6">
              <h2 className={`${lusitana.className} text-2xl text-black`}>
                Select Tracks to Record
              </h2>
              <div className={'flex w-full flex-row items-center gap-2 p-5'}>
                <Select
                  options={trackSelections}
                  value={selections}
                  onChange={handleSelectedChange}
                  closeMenuOnSelect={false}
                  components={animatedComponents}
                  isMulti
                  className={'w-full flex-1 text-black'}
                />
                <Button onClick={isAllSelected ? clearAll : selectAll}>
                  {isAllSelected ? 'Clear All' : 'Select All'}
                </Button>
              </div>
              <input type='text' className={'text-black'} />
              <div className={'flex w-full flex-row justify-center gap-2 p-5'}>
              <Button type={"submit"} className={'float-end mt-10'} >
                {text}
              </Button>
              </div>
          </div>
        </Modal.Body>
      </Modal>
    </Tooltip>
  );
}
