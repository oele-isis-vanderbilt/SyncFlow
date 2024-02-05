'use client';
import '@livekit/components-styles';

import {
  LiveKitRoom,
  RoomName,
  VideoConference,
  GridLayout,
  ParticipantTile,
  useTracks,
  RoomAudioRenderer,
  ControlBar,
} from '@livekit/components-react';

import { Track, Room as RoomType } from 'livekit-client';
import { lusitana } from '@/app/ui/fonts';

export default function Room({ name, token }: { user: string; token: string }) {
  return (
    <LiveKitRoom
      video={false}
      audio={false}
      name={name}
      serverUrl={process.env.NEXT_PUBLIC_LIVEKIT_SERVER_URL}
      token={token}
      data-lk-theme="default"
      className="h-full w-full"
      // style={{ height: '100dvh' }}
    >
      <RoomName className={`${lusitana.className} text-2xl`} />
      <MyVideoConference />
      <ControlBar />
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
