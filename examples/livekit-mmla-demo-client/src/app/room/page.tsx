'use client';
import { useMemo } from 'react';
import {
  LiveKitRoom,
  useToken,
  VideoConference,
} from '@livekit/components-react';
import { LocalUserChoices } from '@livekit/components-core';
import { useRouter, useSearchParams } from 'next/navigation';

export default function Page() {
  let roomName = useSearchParams().get('name');

  if (!roomName) {
    return (
      <div style={{ display: 'grid', placeItems: 'center', height: '100%' }}>
        <p>Invalid Room</p>
      </div>
    );
  }

  const preJoinChoices: LocalUserChoices = JSON.parse(
    localStorage.getItem('preJoinChoices') || '{}',
  );
  let tokenOptions = useMemo(() => {
    return {
      userInfo: {
        identity: preJoinChoices.username,
        name: preJoinChoices.username,
      },
    };
  }, [preJoinChoices.username]);
  const token = useToken('/api/token', roomName, tokenOptions);
  let lkUrl = process.env.NEXT_PUBLIC_LIVEKIT_SERVER_URL;
  const router = useRouter();

  return (
    lkUrl && (
      <div data-lk-theme="default">
        <LiveKitRoom
          className={'h-screen w-screen'}
          serverUrl={lkUrl}
          token={token}
          video={preJoinChoices.videoEnabled}
          audio={preJoinChoices.audioEnabled}
          onDisconnected={() => {
            router.push('/');
          }}
        >
          <VideoConference />
        </LiveKitRoom>
      </div>
    )
  );
}
