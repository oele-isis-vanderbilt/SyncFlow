'use client';
import '@livekit/components-styles';

import {
  LiveKitRoom,
  RoomName,
  GridLayout,
  ParticipantTile,
  useTracks,
  ControlBar,
} from '@livekit/components-react';

import { Track, Room as RoomType } from 'livekit-client';
import { lusitana } from '@/app/ui/fonts';
import { redirectToDashboard } from '@/app/lib/actions';

export default function Room({ name, token }: { user: string; token: string }) {
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
      className="h-full w-full grow"
      // style={{ height: '100dvh' }}
    >
      <RoomName className={`${lusitana.className} text-2xl`} />
      <MyVideoConference />
      <ControlBar variation="verbose" />
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
    <GridLayout
      tracks={tracks}
      style={{ height: 'calc(100vh - var(--lk-control-bar-height))' }}
    >
      <ParticipantTile />
    </GridLayout>
  );
}
