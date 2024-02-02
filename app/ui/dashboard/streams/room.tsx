'use client';
import '@livekit/components-styles';

import {
  LiveKitRoom,
  VideoConference,
  GridLayout,
  ParticipantTile,
  useTracks,
  RoomAudioRenderer,
  ControlBar,
} from '@livekit/components-react';

import { Track, Room as RoomType } from 'livekit-client';

export default function Room({
  user,
  token,
  room,
}: {
  user: string;
  token: string;
}) {
  console.log(user, token, room);

  return (
    <LiveKitRoom
      video={true}
      audio={true}
      name={user}
      serverUrl={process.env.NEXT_PUBLIC_LIVEKIT_SERVER_URL}
      token={token}
      data-lk-theme="default"
      style={{ height: '100dvh' }}
    >
      <MyVideoConference />
      <RoomAudioRenderer />
      <ControlBar />
    </LiveKitRoom>
  );
}

function MyVideoConference() {
  const tracks = useTracks(
    [
      { source: Track.Source.Camera, withPlaceholder: true },
      { source: Track.Source.ScreenShare, withPlaceholder: true },
    ],
    { onlySubscribed: false },
  );
  console.log(tracks);
  return (
    <GridLayout
      tracks={tracks}
      style={{ height: 'calc(100vh - var(--lk-control-bar-height))' }}
    >
      <ParticipantTile />
    </GridLayout>
  );
}
