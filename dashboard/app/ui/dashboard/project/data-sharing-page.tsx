'use client';

import {
  ChatEntry,
  LiveKitRoom,
  useLocalParticipant,
  useMediaDevices,
  VideoTrack,
} from '@livekit/components-react';
import { useEffect, useRef, useState } from 'react';
import {
  AudioPresets,
  createLocalAudioTrack,
  createLocalScreenTracks,
  createLocalVideoTrack,
  DataPacket_Kind,
  LocalAudioTrack,
  LocalTrackPublication,
  RemoteAudioTrack,
  Track,
  videoCodecs,
  VideoPresets,
} from 'livekit-client';
import Select from 'react-select';
import { customClassNames } from '@/app/ui/dashboard/rooms/widgets/utils';
import 'react-virtualized/styles.css';
import {
  List,
  AutoSizer,
  CellMeasurer,
  CellMeasurerCache,
} from 'react-virtualized';
import '@livekit/components-styles';
import { redirectTo, redirectToDashboard } from '@/app/lib/actions';

export interface RoomJoinOptions {
  videoPreset: keyof typeof VideoPresets;
  audioPreset: keyof typeof AudioPresets;
  videoCodec: (typeof videoCodecs)[number];
}

export default function DataSharer({
  token,
  sessionName,
  user,
  lkServerUrl,
  joinOptions,
  settingsUrl,
  disconnectRedirectUrl,
}: {
  token: string;
  sessionName: string;
  user: string;
  lkServerUrl?: string;
  settingsUrl?: string;
  joinOptions?: RoomJoinOptions;
  disconnectRedirectUrl?: string;
}) {
  const roomOptions = {
    videoPreset: 'h1080',
    audioPreset: 'speech',
    videoCodec: 'h264',
    ...(joinOptions || {}),
  };

  return (
    <div data-lk-theme="huddle" className="h-full w-full">
      <LiveKitRoom
        serverUrl={lkServerUrl || process.env.NEXT_PUBLIC_LIVEKIT_SERVER_URL}
        token={token}
        video={false}
        audio={false}
        onDisconnected={() => {
          disconnectRedirectUrl
            ? redirectTo(disconnectRedirectUrl)
            : redirectToDashboard();
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
        <div className="flex h-full w-full flex-col overflow-x-hidden p-2 dark:text-white">
          <div className="mb-10 w-full text-center">
            <h1 className="text-2xl font-bold">
              Welcome to session {sessionName}, {user}!{' '}
            </h1>
            <p>
              Connect Web Cameras and Microphones to your system for streaming
              to the room, or go{' '}
              <a
                href={settingsUrl || disconnectRedirectUrl || '/dashboard'}
                className={'text-red-700 hover:underline'}
              >
                {' '}
                back
              </a>{' '}
              and change the room settings.
            </p>
          </div>
          <div className={'mb-10 w-full'}>
            <DataSender />
          </div>
          <div className="flex flex-grow flex-row">
            <div className="h-full w-1/3">
              <VideoTracksPublisher
                preset={roomOptions.videoPreset as keyof typeof VideoPresets}
              />
            </div>
            <div className="h-full w-1/3">
              <AudioTracksPublisher
                preset={roomOptions.audioPreset as keyof typeof AudioPresets}
              />
            </div>
            <div className="h-full w-1/3">
              <ScreenSharer
                preset={roomOptions.videoPreset as keyof typeof VideoPresets}
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
        classNames={customClassNames}
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
        classNames={customClassNames}
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

function AudioRenderer({
  track,
  shouldMute,
  className,
}: {
  track: LocalAudioTrack | RemoteAudioTrack | undefined;
  shouldMute: boolean;
  className: string;
}) {
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
}

function DataSender() {
  const cache = new CellMeasurerCache({
    fixedWidth: true,
    defaultHeight: 100, // A default height estimation
  });

  const participantInfo = useLocalParticipant();
  const encoder = new TextEncoder();
  const [data, setData] = useState<string>('');
  const [chatMessages, setChatMessages] = useState<
    {
      message: string;
      timestamp: number;
    }[]
  >([]);
  const [topic, setTopic] = useState<string>('chat');
  const topics = ['chat', 'log'].map((topic) => {
    return {
      label: topic,
      value: topic,
    };
  });

  const send = () => {
    if (data) {
      const dataArray = encoder.encode(data);
      participantInfo.localParticipant
        .publishData(dataArray, DataPacket_Kind.RELIABLE, {
          topic: topic,
        })
        .then(() => {
          setData('');
          const newChatMessages = [
            {
              message: data,
              timestamp: Date.now(),
            },
            ...chatMessages,
          ];
          setChatMessages(newChatMessages);
        });
    }
  };

  const ChatMessageFormatter = (message: string) => {
    return (
      <span className="rounded-lg bg-gray-200 p-2 dark:bg-gray-900">
        {message}
      </span>
    );
  };

  const renderItem = ({ key, index, style, parent }) => {
    const chatMessage = chatMessages[index];
    return (
      <CellMeasurer
        key={key}
        cache={cache}
        parent={parent}
        columnIndex={0}
        rowIndex={index}
      >
        {({ measure, registerChild }) => (
          <div ref={registerChild} key={key} style={style} onLoad={measure}>
            <ChatEntry
              entry={{
                id: index.toString(),
                from: participantInfo.localParticipant,
                message: chatMessage.message,
                timestamp: chatMessage.timestamp,
              }}
              hideName={false}
              hideTimestamp={true}
              messageFormatter={ChatMessageFormatter}
            />
          </div>
        )}
      </CellMeasurer>
    );
  };

  return (
    <div className={'flex flex-row gap-5'}>
      <div className={'flex w-1/2 flex-col'}>
        <div className={'flex items-center justify-between'}>
          <h2 className={'text-xl'}>Send a message</h2>

          <div className={'w-1/4'}>
            <div>Select Topic</div>
            <Select
              classNames={customClassNames}
              placeholder={'select a topic'}
              options={topics}
              defaultValue={topics[0]}
              onChange={(selected) => {
                // @ts-ignore
                setTopic(selected.value || 'chat');
              }}
            />
          </div>
        </div>
        <div className="flex-grow">
          <textarea
            value={data}
            onChange={(e) => setData(e.target.value)}
            placeholder={'Type your message'}
            className={'mb-2 min-h-64 w-full p-2 dark:bg-gray-900'}
          />
        </div>
        <button
          onClick={send}
          className={
            'w-96 max-w-96 self-center rounded-md bg-blue-500 p-2 text-white'
          }
        >
          Send
        </button>
      </div>
      <div className="flex-grow">
        <h2 className="text-xl">Sent Messages</h2>
        <div className="h-full w-full">
          {' '}
          <AutoSizer>
            {({ width, height }) => (
              <List
                width={width}
                height={height}
                rowCount={chatMessages.length}
                deferredMeasurementCache={cache}
                rowHeight={cache.rowHeight}
                rowRenderer={renderItem}
                overscanRowCount={3}
              />
            )}
          </AutoSizer>
        </div>
      </div>
    </div>
  );
}

function ScreenSharer({ preset }: { preset: keyof typeof VideoPresets }) {
  const videoPreset = VideoPresets[preset as keyof typeof VideoPresets];
  const participantInfo = useLocalParticipant();
  const [publication, setPlublication] = useState<LocalTrackPublication | null>(
    null,
  );

  const [isSharing, setIsSharing] = useState<boolean>(false);

  const startSharing = async () => {
    const screenTrack = await createLocalScreenTracks({
      resolution: videoPreset.resolution,
      audio: false,
    });

    const localTrackPublication =
      await participantInfo.localParticipant.publishTrack(screenTrack[0], {
        name: `${participantInfo.localParticipant.name}-ScreenShare`,
      });
    // console.log('localTrackPublication', localTrackPublication);
    setPlublication(localTrackPublication);
    setIsSharing(true);
  };

  const stopSharing = async () => {
    if (publication && publication.track) {
      publication.track.stop();
      participantInfo.localParticipant
        .unpublishTrack(publication.track)
        .then(() => {
          setPlublication(null);
          setIsSharing(false);
        })
        .catch((error) => console.error('Error unpublishing track:', error));
    }
  };

  return (
    <div className={'flex h-full w-full flex-col justify-center p-2'}>
      <h2 className={'text-xl'}>
        {isSharing ? 'Stop Sharing your screen' : 'Start Sharing your screen'}
      </h2>
      <button
        onClick={() => {
          if (isSharing) {
            // Stop sharing
            stopSharing();
          } else {
            startSharing();
          }
          // setIsSharing(!isSharing);
        }}
        className={`${isSharing ? 'bg-red-900' : 'bg-blue-500'} w-full rounded-md p-2 text-white`}
      >
        {isSharing ? 'Stop Sharing' : 'Start Sharing'}
      </button>
      <div className="flex-1">
        {publication && publication.track ? (
          <div className={'flex flex-col items-center'}>
            <VideoTrack
              trackRef={{
                participant: participantInfo.localParticipant,
                publication: publication,
                source: Track.Source.ScreenShare,
              }}
            />
            <p>Screen-{publication.track?.sid?.slice(1, 5)}</p>
            <p>
              TrackId: {publication.track?.sid}.
              <button
                className={'text-red-700 hover:text-red-700'}
                onClick={() => {}}
              >
                {' '}
                &#8505;{' '}
              </button>
            </p>
          </div>
        ) : null}
      </div>
    </div>
  );
}
