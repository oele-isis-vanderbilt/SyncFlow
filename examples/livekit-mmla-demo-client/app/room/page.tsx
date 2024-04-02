/* eslint-disable */
import LkRoom from '@/app/lk-room';
import type { RoomJoinOptions } from '@/app/lk-room';
import type { AudioPresets, videoCodecs, VideoPresets } from 'livekit-client';

export default async function Page({
  searchParams,
}: {
  searchParams?: { [key: string]: string | string[] | undefined };
}) {
  let roomName = searchParams?.name;
  let identity = searchParams?.identity;
  let videoCodec = (searchParams?.videoCodec ||
    'h264') as (typeof videoCodecs)[number];
  let audioPreset = (searchParams?.audioPreset ||
    'speech') as keyof typeof AudioPresets;
  let videoPreset = (searchParams?.videoPreset ||
    'h1080') as keyof typeof VideoPresets;

  let joinOptions: RoomJoinOptions | undefined = {
    videoPreset: videoPreset,
    audioPreset: audioPreset,
    videoCodec: videoCodec,
  };

  if (!roomName || !identity) {
    return (
      <div style={{ display: 'grid', placeItems: 'center', height: '100%' }}>
        <p>Invalid Room</p>
      </div>
    );
  }

  return (
    <LkRoom
      roomName={roomName as string}
      participantId={identity as string}
      joinOptions={joinOptions}
    ></LkRoom>
  );
}
