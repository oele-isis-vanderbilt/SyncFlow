'use client';
import { BsRecordBtn, BsStopBtnFill } from 'react-icons/bs';
import { useState } from 'react';
import {
  beginTrackEgress,
  beginTracksEgress,
  stopEgress,
  stopTracksEgress,
} from '@/app/lib/actions';
import { EgressStatus } from 'livekit-server-sdk/dist/proto/livekit_egress';
import { Button } from '@/app/ui/button';
import { Modal } from 'flowbite-react';
import { lusitana } from '@/app/ui/fonts';
import Select from 'react-select';
import makeAnimated from 'react-select/animated';

type TrackInfo = {
  participant: string;
  trackId: string;
  kind: string;
  roomName: string;
  egressId: string | undefined;
};

export function TrackRecordButton({ trackInfo }: { trackInfo: TrackInfo }) {
  const { trackId, roomName } = trackInfo;
  const [isBtnDisabled, setIsBtnDisabled] = useState(false);
  const [isRecording, setIsRecording] = useState(!!trackInfo.egressId);
  const [bindedEgressId, setBindedEgressId] = useState(trackInfo.egressId);
  console.log(trackInfo.egressId, '***', bindedEgressId, isRecording);
  return (
    <Button
      className="flex bg-transparent hover:bg-gray-900"
      onClick={async () => {
        setIsBtnDisabled(true);
        if (!isRecording) {
          const egressInfo = await beginTrackEgress(roomName, trackId);
          setIsRecording(
            [EgressStatus.EGRESS_ACTIVE, EgressStatus.EGRESS_STARTING].includes(
              egressInfo.status!,
            ),
          );
          [EgressStatus.EGRESS_ACTIVE, EgressStatus.EGRESS_STARTING].includes(
            egressInfo.status!,
          )
            ? setBindedEgressId(egressInfo.egressId)
            : null;
        } else {
          const egressInfo = await stopEgress(roomName, bindedEgressId!);
          setIsRecording(egressInfo.status === EgressStatus.EGRESS_COMPLETE);
          setBindedEgressId(undefined);
        }
        setIsBtnDisabled(false);
      }}
      disabled={isBtnDisabled}
    >
      {isRecording ? (
        <BsStopBtnFill className="w-6 text-red-500" />
      ) : (
        <BsRecordBtn className="w-6 hover:text-red-500" />
      )}
      <p className="hidden p-1 md:block">{isRecording ? 'Stop' : 'Record'}</p>
    </Button>
  );
}

export function AllTracksRecordButton({ tracks }: { tracks: TrackInfo[] }) {
  const activeRecordingTracks = tracks.filter((tracks) => !!tracks.egressId);
  const canRecordTracks = tracks.filter((tracks) => !tracks.egressId);
  const [openModal, setOpenModal] = useState(false);
  const close = () => setOpenModal(false);
  const open = () => setOpenModal(true);

  const stopActiveRecordings = async () => {
    if (activeRecordingTracks.length > 0) {
      const roomName = activeRecordingTracks[0].roomName;
      await stopTracksEgress(
        activeRecordingTracks.map((t) => t.egressId) as string[],
        roomName,
      );
    }
  };

  return (
    <div className={'flex flex-row justify-end'}>
      <Button
        className={'bg-transparent hover:bg-gray-900'}
        onClick={open}
        aria-disabled={canRecordTracks.length === 0}
        disabled={canRecordTracks.length === 0}
      >
        Multiple Track Recording
      </Button>
      <Button
        className={'bg-transparent hover:bg-gray-900'}
        aria-disabled={activeRecordingTracks.length === 0}
        disabled={activeRecordingTracks.length === 0}
        onClick={stopActiveRecordings}
      >
        Stop All Active Recordings
      </Button>
      <RecorderModal
        toRecord={canRecordTracks}
        activelyRecording={activeRecordingTracks}
        show={openModal}
        onClose={close}
      />
    </div>
  );
}

function RecorderModal({
  toRecord,
  activelyRecording,
  show,
  onClose,
}: {
  toRecord: TrackInfo[];
  activelyRecording: TrackInfo[];
  show: boolean;
  onClose: () => void;
}) {
  return (
    <Modal show={show} size={'lg'} position={'center'} onClose={onClose}>
      <Modal.Header className={`${lusitana.className}`}>
        Record Multiple Tracks
      </Modal.Header>
      <Modal.Body>
        <div className={'flex w-full flex-col gap-2 text-black'}>
          <div className={'flex-1'}>
            <MultiTrackRecordForm tracks={toRecord} onSuccess={onClose} />
          </div>
        </div>
      </Modal.Body>
      <Modal.Footer>
        <Button onClick={() => onClose()}>Close</Button>
      </Modal.Footer>
    </Modal>
  );
}

function MultiTrackRecordForm({
  tracks,
  onSuccess,
}: {
  tracks: TrackInfo[];
  onSuccess: () => void;
}) {
  const animatedComponents = makeAnimated();
  const options = tracks.map((track) => {
    return {
      value: track.trackId,
      label: track.participant + ' - ' + track.kind,
    };
  });

  let [selectedTracks, setSelectedTracks] = useState([]);
  const [isBtnDisabled, setIsBtnDisabled] = useState(
    selectedTracks.length === 0,
  );

  const handleChange = (selectedOption: any) => {
    setSelectedTracks(selectedOption);
    setIsBtnDisabled(selectedOption.length === 0);
  };

  const toggleSelections = () => {
    if (selectedTracks.length === tracks.length) {
      setSelectedTracks([]);
      setIsBtnDisabled(true);
    } else {
      // @ts-ignore
      setSelectedTracks([...options]);
      setIsBtnDisabled(false);
    }
  };

  const startRecording = async () => {
    setIsBtnDisabled(true);
    const tracksToRecord = options
      .map((opt) => tracks.find((t) => t.trackId === opt.value))
      .filter((t) => t !== undefined);

    const roomName = tracksToRecord[0]?.roomName;
    if (!roomName) {
      return;
    }
    const trackIds = tracksToRecord.map((t) => t!.trackId);
    await beginTracksEgress(trackIds, roomName);
    setIsBtnDisabled(false);
    onSuccess();
  };

  return (
    <div className={'flex h-full min-h-48 w-full flex-col'}>
      <div className={'flex flex-row items-center justify-between'}>
        <h2 className={`${lusitana.className} text-xl`}>Available Tracks</h2>
        <Button
          className={'bg-teal-600 hover:bg-green-500'}
          onClick={toggleSelections}
        >
          {options.length === selectedTracks.length
            ? 'Clear All'
            : 'Select All'}
        </Button>
      </div>
      <div className={'flex-1'}>
        <Select
          isMulti
          options={options}
          value={selectedTracks}
          onChange={handleChange}
          components={animatedComponents}
          className={'w-full p-2'}
        />
      </div>
      <Button
        className={'self-center'}
        onClick={startRecording}
        aria-disabled={isBtnDisabled}
        disabled={isBtnDisabled}
      >
        Record Selected
        {isBtnDisabled && '...'}
      </Button>
    </div>
  );
}
