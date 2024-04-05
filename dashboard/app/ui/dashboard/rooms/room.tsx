'use client';
import '@livekit/components-styles';
import VideoGallery from '@/app/ui/dashboard/rooms/video-gallery';
import {
  LiveKitRoom,
  RoomName,
  ControlBar,
  useRoomContext,
  useTracks,
  Chat,
  LayoutContextProvider,
} from '@livekit/components-react';

import { Tooltip } from 'flowbite-react';

import Link from 'next/link';
import AudioStreams from '@/app/ui/dashboard/rooms/audio-streams';

import { lusitana } from '@/app/ui/fonts';
import { redirectToDashboard } from '@/app/lib/actions';
import { BsRecordBtn } from 'react-icons/bs';
import type { SessionUser } from '@/types/next-auth';
import { isAdmin } from '@/app/lib/utils';
import TopicalMessages from '@/app/ui/dashboard/rooms/topical-messages';

export default function Room({
  token,
  user,
}: {
  token: string;
  user: SessionUser | undefined;
}) {
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
        <TopBar user={user} />
        <div className="flex h-full w-full flex-row bg-black">
          <div className="h-full w-1/2 flex-1">
            <VideoGallery title={'Videos'} />
          </div>
          <div className="h-full w-1/2">
            <div className="flex h-full w-full flex-col">
              <div className="h-1/2 w-full bg-white">
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
        {isAdmin(user) && <RoomRecorderNavigator />}
      </div>
    </div>
  );
}

export function RoomRecorderNavigator() {
  const roomInfo = useRoomContext();
  const tracks = useTracks();

  return (
    <div className={tracks.length > 0 ? 'block' : 'hidden'}>
      <Tooltip content={'Manage Room Recordings'}>
        <Link href={`/dashboard/recordings/${roomInfo.name!}/`} target="_blank">
          <BsRecordBtn
            className="cursor-pointer text-4xl hover:text-red-500"
            onClick={() => {}}
          />
        </Link>
      </Tooltip>
    </div>
  );
}
