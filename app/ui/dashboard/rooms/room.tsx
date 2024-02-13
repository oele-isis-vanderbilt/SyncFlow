'use client';
import '@livekit/components-styles';
import VideoGallery from '@/app/ui/dashboard/rooms/video-gallery';

import {
  LiveKitRoom,
  RoomName,
  GridLayout,
  ParticipantTile,
  useTracks,
  ControlBar,
  FocusLayout,
  CarouselLayout,
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
      className={`h-full w-full`}
      onDisconnected={() => {
        redirectToDashboard();
      }}
    >
      <div className="flex h-full w-full flex-col bg-black">
        <TopBar />
        <div className="flex h-full w-full flex-row bg-black">
          <div className="h-full w-1/2 flex-1">
            <VideoGallery title={'Videos'} />
          </div>
          <div className="h-full flex-1 border-l-4 border-red-900 bg-gray-100">
            <div className="flex h-full w-full flex-col items-center justify-center">
              <h2
                className={`${lusitana.className} p-2 text-xl text-black md:text-2xl`}
              >
                ToDo: Audio Components
              </h2>
              <p className="text-lg text-black md:text-xl">Coming soon...</p>
            </div>
          </div>
        </div>
      </div>
    </LiveKitRoom>
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
