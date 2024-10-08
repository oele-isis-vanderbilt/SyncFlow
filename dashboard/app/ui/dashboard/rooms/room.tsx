'use client';
import '@livekit/components-styles';
import VideoGallery from '@/app/ui/dashboard/rooms/video-gallery';
import {
  LiveKitRoom,
  RoomName,
  ControlBar,
  useRoomContext,
  useTracks,
} from '@livekit/components-react';

import { Tooltip } from 'flowbite-react';

import Link from 'next/link';
import AudioStreams from '@/app/ui/dashboard/rooms/audio-streams';

import { lusitana } from '@/app/ui/fonts';
import { redirectTo, redirectToDashboard } from '@/app/lib/actions';
import { BsRecordBtn } from 'react-icons/bs';
import type { SessionUser } from '@/types/next-auth';
import TopicalMessages from '@/app/ui/dashboard/rooms/topical-messages';

export default function Room({
  token,
  user,
  lkServerUrl,
  disconnectRedirectUrl,
}: {
  token: string;
  user: SessionUser | undefined;
  lkServerUrl?: string;
  disconnectRedirectUrl?: string;
}) {
  return (
    <LiveKitRoom
      video={false}
      audio={false}
      serverUrl={lkServerUrl || process.env.NEXT_PUBLIC_LIVEKIT_SERVER_URL}
      token={token}
      className={`h-full w-full`}
      options={{
        adaptiveStream: false,
        publishDefaults: {
          videoCodec: 'h264',
        },
      }}
      onDisconnected={() => {
        disconnectRedirectUrl
          ? redirectTo(disconnectRedirectUrl)
          : redirectToDashboard();
      }}
    >
      <div className="flex h-full w-full flex-col">
        <TopBar user={user} />
        <div className="flex h-full w-full flex-row">
          <div className="h-full w-1/2 flex-1">
            <VideoGallery title={'Videos'} />
          </div>
          <div className="h-full w-1/2">
            <div className="flex h-full w-full flex-col">
              <div className="h-1/2 w-full">
                <AudioStreams title={'Audio Streams'} />
              </div>
              <div className="h-1/2 w-full">
                <TopicalMessages title={'Topical Messages'} />
              </div>
            </div>
          </div>
        </div>
      </div>
    </LiveKitRoom>
  );
}

function TopBar({ user }: { user: SessionUser | undefined }) {
  return (
    <div className="flex h-20 w-full items-center justify-between py-2 md:py-5 dark:text-white">
      <div className="w-1/2">
        <RoomName className={`${lusitana.className} p-2 text-xl md:text-2xl`} />
      </div>
      <div className="flex w-1/2 items-center justify-end gap-4 p-2">
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
        <RoomRecorderNavigator />
      </div>
    </div>
  );
}

export function RoomRecorderNavigator() {
  const roomInfo = useRoomContext();
  const tracks = useTracks();

  return (
    <div className={tracks.length > 0 ? 'block' : 'hidden'}>
      <Tooltip content="Manage Room Recordings" animation={false}>
        <a href={`/dashboard/recordings/${roomInfo.name!}/`} target="_blank">
          <BsRecordBtn
            className="cursor-pointer text-4xl hover:text-red-500"
            onClick={() => {}}
          />
        </a>
      </Tooltip>
    </div>
  );
}
