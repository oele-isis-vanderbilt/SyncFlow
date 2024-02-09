'use client';
import '@livekit/components-styles';

import {
    LiveKitRoom,
    RoomName,
    GridLayout,
    ParticipantTile,
    useTracks,
    ControlBar, FocusLayout, CarouselLayout,
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
      <TopBar />
      <div className="flex flex-row bg-black">
        <div className="h-full w-1/2 flex-1">
            <VideoGallery/>
        </div>
        <div className="h-full bg-gray-100 flex-1 border-red-900 border-l-4">
            <div className="h-full w-full flex flex-col justify-center items-center">
                <h2 className={`${lusitana.className} p-2 text-xl md:text-2xl text-black`}>
                ToDo: Audio Components
                </h2>
                <p className="text-lg md:text-xl text-black">Coming soon...</p>
            </div>
        </div>
      </div>
    </LiveKitRoom>
  );
}

function VideoGallery() {
  const tracks = useTracks(
    [
      { source: Track.Source.Camera, withPlaceholder: false },
      { source: Track.Source.ScreenShare, withPlaceholder: false },
    ],
    { onlySubscribed: true },
  );

  return (
    <>
      <h2 className={`${lusitana.className} p-2 text-xl md:text-2xl`}>
        Video Streams
      </h2>

    </>
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
