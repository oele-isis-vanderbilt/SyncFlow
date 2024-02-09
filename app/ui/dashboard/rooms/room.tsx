'use client';
import '@livekit/components-styles';
import clsx from 'clsx';

import {
  LiveKitRoom,
  RoomName,
  GridLayout,
  ParticipantTile,
  useTracks,
  ControlBar,
} from '@livekit/components-react';

import { Track } from 'livekit-client';
import { lusitana } from '@/app/ui/fonts';
import { redirectToDashboard } from '@/app/lib/actions';

export default function Room({ token }: { token: string }) {
  return (
    <LiveKitRoom
      video={false}
      audio={false}
      serverUrl={process.env.NEXT_PUBLIC_LIVEKIT_SERVER_URL}
      token={token}
      // data-lk-theme="default"
      onDisconnected={() => {
        redirectToDashboard();
      }}
      style={{ height: 'ca' }}
    >
      <TopBar />
      <MyVideoConference />
      {/*<ControlBar variation="verbose" />*/}
    </LiveKitRoom>
  );
}

function MyVideoConference() {
  const tracks = useTracks(
    [
      { source: Track.Source.Camera, withPlaceholder: false },
      { source: Track.Source.ScreenShare, withPlaceholder: false },
    ],
    { onlySubscribed: true },
  );

  return (
    <GridLayout tracks={tracks} className="h-full">
      <ParticipantTile />
    </GridLayout>
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
      </div>
    </div>
  );
}
