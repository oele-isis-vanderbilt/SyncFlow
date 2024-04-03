'use client';

import {
  LiveKitRoom,
  useLocalParticipant,
  useMediaDevices,
  useToken,
  VideoTrack,
} from '@livekit/components-react';
import { useEffect, useMemo, useRef, useState } from 'react';
import { useRouter } from 'next/navigation';
import {
  AudioPresets,
  createLocalAudioTrack,
  createLocalVideoTrack,
  LocalAudioTrack,
  LocalTrackPublication,
  RemoteAudioTrack,
  Track,
  videoCodecs,
  VideoPresets,
} from 'livekit-client';
import Select from 'react-select';
import { customSelectStyles } from '@/app/utils';

export interface RoomJoinOptions {
  videoPreset: keyof typeof VideoPresets;
  audioPreset: keyof typeof AudioPresets;
  videoCodec: (typeof videoCodecs)[number];
}

export default function LkRoom({
  roomName,
  participantId,
  joinOptions,
}: {
  roomName: string;
  participantId: string;
  joinOptions?: RoomJoinOptions;
}) {
  let tokenOptions = useMemo(() => {
    return {
      userInfo: {
        identity: participantId,
        name: participantId,
      },
    };
  }, [participantId]);
  const token = useToken('/api/token', roomName, tokenOptions);
  const lkUrl = process.env.NEXT_PUBLIC_LIVEKIT_SERVER_URL;
  const router = useRouter();

  const roomOptions = {
    videoPreset: 'h1080',
    audioPreset: 'speech',
    videoCodec: 'h264',
    ...(joinOptions || {}),
  };

  return (
    <div data-lk-theme="default">
      <LiveKitRoom
        className={'h-screen w-screen'}
        serverUrl={lkUrl}
        token={token}
        video={false}
        audio={false}
        onDisconnected={() => {
          router.push('/');
        }}
        options={{
          adaptiveStream: true,
          publishDefaults: {
            videoCodec:
              videoCodecs.find((codec) => codec === roomOptions.videoCodec) ||
              'h264',
            audioPreset:
              AudioPresets[
                roomOptions.audioPreset as keyof typeof AudioPresets
              ],
          },
        }}
      >
        <div className="flex h-screen w-screen flex-col p-10">
          <div className="mb-10 w-full text-center">
            <h1 className="text-2xl font-bold">
              Welcome to room {roomName}, {participantId}!{' '}
            </h1>
            <p>
              Connect Web Cameras and Microphones to your system for streaming
              to the room, or go{' '}
              <a href={'/'} className={'text-red-700 hover:underline'}>
                {' '}
                back
              </a>{' '}
              and change the room settings.
            </p>
          </div>
          <div className="flex flex-grow flex-row">
            <div className="h-full w-1/2">
              <VideoTracksPublisher
                preset={roomOptions.videoPreset as keyof typeof VideoPresets}
              />
            </div>
            <div className="h-full w-1/2">
              <AudioTracksPublisher
                preset={roomOptions.audioPreset as keyof typeof AudioPresets}
              />
            </div>
          </div>
        </div>
      </LiveKitRoom>
    </div>
  );
}

function VideoTracksPublisher({
  preset,
}: {
  preset: keyof typeof VideoPresets;
}) {
  const localVideoDevices = useMediaDevices({ kind: 'videoinput' });
  let videoDeviceOptions = localVideoDevices.map((device) => {
    return {
      label: device.label,
      value: device.deviceId,
    };
  });

  const [selectedDevices, setSelectedDevices] = useState<string[]>();
  const deviceNameByDeviceId = new Map<string, string>();
  localVideoDevices.forEach((device) => {
    deviceNameByDeviceId.set(device.deviceId, device.label);
  });

  return (
    <div className={'flex h-full w-full flex-col p-2'}>
      <h2 className={'text-xl'}>Select video devices to publish</h2>
      <Select
        isMulti
        styles={customSelectStyles}
        options={videoDeviceOptions}
        placeholder={'Select video devices'}
        onChange={(selected) => {
          // @ts-ignore
          setSelectedDevices(selected.map((device) => device.value));
        }}
      />
      <div className={'grid items-center md:grid-cols-2'}>
        {selectedDevices?.map((deviceId, index) => {
          return (
            <SingleVideoTrackPreviewAndPublish
              key={index}
              deviceId={deviceId}
              deviceName={deviceNameByDeviceId.get(deviceId)}
              preset={preset}
            />
          );
        })}
      </div>
    </div>
  );
}

function AudioTracksPublisher({
  preset,
}: {
  preset: keyof typeof AudioPresets;
}) {
  const localVideoDevices = useMediaDevices({ kind: 'audioinput' });
  let audioDeviceOptions = localVideoDevices.map((device) => {
    return {
      label: device.label,
      value: device.deviceId,
    };
  });

  const [selectedDevices, setSelectedDevices] = useState<string[]>();
  const deviceNameByDeviceId = new Map<string, string>();
  localVideoDevices.forEach((device) => {
    deviceNameByDeviceId.set(device.deviceId, device.label);
  });

  return (
    <div className={'flex h-full w-full flex-col p-2'}>
      <h2 className={'text-xl'}>Select audio devices to publish</h2>
      <Select
        isMulti
        styles={customSelectStyles}
        options={audioDeviceOptions}
        placeholder={'Select audio devices'}
        onChange={(selected) => {
          // @ts-ignore
          setSelectedDevices(selected.map((device) => device.value));
        }}
      />
      <div className={'grid items-center justify-center gap-2 md:grid-cols-2'}>
        {selectedDevices?.map((deviceId, index) => {
          return (
            <SingleAudioTrackPreviewAndPublish
              key={index}
              deviceId={deviceId}
              deviceName={deviceNameByDeviceId.get(deviceId)}
              preset={preset}
            />
          );
        })}
      </div>
    </div>
  );
}

function SingleVideoTrackPreviewAndPublish({
  deviceId,
  deviceName,
  preset,
}: {
  deviceId: string;
  preset: keyof typeof VideoPresets;
  deviceName?: string;
}) {
  const videoPreset = VideoPresets[preset as keyof typeof VideoPresets];
  const participantInfo = useLocalParticipant();
  const [videoTrack, setVideoTrack] = useState<LocalTrackPublication | null>(
    null,
  );

  useEffect(() => {
    let isCancelled = false; // Flag to prevent state update after unmount

    // Function to publish video track
    const publishVideoTrack = async () => {
      const videoTrack = await createLocalVideoTrack({
        deviceId: deviceId,
        resolution: videoPreset.resolution,
        facingMode: 'user',
      });

      if (isCancelled) return; // Prevent proceeding if component has unmounted

      const localTrackPublication =
        await participantInfo.localParticipant.publishTrack(videoTrack, {
          name: deviceName || deviceId,
        });
      setVideoTrack(localTrackPublication);
    };

    publishVideoTrack();

    return () => {
      isCancelled = true; // Set flag to true when component unmounts

      if (videoTrack && videoTrack.track) {
        videoTrack.track?.stop();
        participantInfo.localParticipant
          .unpublishTrack(videoTrack.track)
          .then(() => setVideoTrack(null))
          .catch((error) => console.error('Error unpublishing track:', error));
      }
    };
  }, [deviceId, deviceName, participantInfo.localParticipant]);

  return videoTrack && videoTrack.track ? (
    <div className={'flex flex-col items-center'}>
      <VideoTrack
        trackRef={{
          participant: participantInfo.localParticipant,
          publication: videoTrack,
          source: Track.Source.Camera,
        }}
      />
      <p>
        {deviceName}-{deviceId.slice(1, 5)}
      </p>
      <p>
        TrackId: {videoTrack.track.sid}.
        <button
          className={'text-red-700 hover:text-red-700'}
          onClick={() => {}}
        >
          {' '}
          &#8505;{' '}
        </button>
      </p>
    </div>
  ) : null;
}

function SingleAudioTrackPreviewAndPublish({
  deviceId,
  preset,
  deviceName,
}: {
  deviceId: string;
  preset: keyof typeof AudioPresets;
  deviceName?: string;
}) {
  const audioPreset = AudioPresets[preset as keyof typeof AudioPresets];
  const participantInfo = useLocalParticipant();
  const [audioTrack, setAudioTrack] = useState<LocalTrackPublication | null>(
    null,
  );

  useEffect(() => {
    let isCancelled = false; // Flag to prevent state update after unmount

    // Function to publish video track
    const publishAudioTrack = async () => {
      const audioTrack = await createLocalAudioTrack({
        deviceId: deviceId,
        sampleRate: audioPreset.maxBitrate,
        channelCount: 1, // FixMe: Mono audio for now
      });

      if (isCancelled) return; // Prevent proceeding if component has unmounted

      const localTrackPublication =
        await participantInfo.localParticipant.publishTrack(audioTrack, {
          name: deviceName || deviceId,
        });
      setAudioTrack(localTrackPublication);
    };

    publishAudioTrack();

    return () => {
      isCancelled = true; // Set flag to true when component unmounts

      if (audioTrack && audioTrack.track) {
        audioTrack.track.stop();
        participantInfo.localParticipant
          .unpublishTrack(audioTrack.track)
          .then(() => setAudioTrack(null))
          .catch((error) => console.error('Error unpublishing track:', error));
      }
    };
  }, [deviceId, deviceName, participantInfo.localParticipant]);

  return audioTrack && audioTrack.track ? (
    <div className={'flex flex-col items-center'}>
      <AudioRenderer
        track={audioTrack.audioTrack}
        className={'p-2'}
        shouldMute={true}
      />
      <p>
        {deviceName}-{deviceId.slice(1, 5)}
      </p>
      <p>
        TrackId: {audioTrack.track.sid}.
        <button
          className={'text-red-700 hover:text-red-700'}
          onClick={() => {}}
        >
          {' '}
          &#8505;{' '}
        </button>
      </p>
    </div>
  ) : null;
}

const AudioRenderer = ({
  track,
  shouldMute,
  className,
}: {
  track: LocalAudioTrack | RemoteAudioTrack | undefined;
  shouldMute: boolean;
  className: string;
}) => {
  const audioRef = useRef<HTMLAudioElement>(null);

  useEffect(() => {
    if (audioRef.current) {
      track?.attach(audioRef.current);
      audioRef.current.muted = shouldMute;
    }
    return () => {
      track?.detach();
    };
  }, [track, shouldMute]);

  if (
    !(track instanceof LocalAudioTrack) &&
    !(track instanceof RemoteAudioTrack)
  ) {
    return null;
  }

  return (
    <audio
      ref={audioRef}
      muted={shouldMute}
      controls={true}
      className={className}
    />
  );
};
