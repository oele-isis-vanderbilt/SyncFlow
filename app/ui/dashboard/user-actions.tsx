'use client';
import { EyeIcon, ShareIcon, CameraIcon } from '@heroicons/react/24/outline';
import { deleteRoom, generateToken } from '@/app/lib/actions';
import { Room } from 'livekit-client';

export default function Page({ roomName }: { roomName: string }) {
  return (
    <div className="mb-32 grid text-center lg:mb-0 lg:w-full lg:max-w-5xl lg:grid-cols-4 lg:text-left">
      <div
        className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
        rel="noopener noreferrer"
        role="button"
        onClick={() => {
          deleteRoom(roomName);
        }}
      >
        <h2 className={`mb-3 text-2xl font-semibold`}>
          Online{' '}
          <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
            <EyeIcon className="w-6" />
          </span>
        </h2>
        <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
          Yay! you can now stream to room {roomName}. Click to delete the room.
        </p>
      </div>
      <div
        className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
        rel="noopener noreferrer"
        role="button"
      >
        <h2 className={`mb-3 text-2xl font-semibold`}>
          Share screen{' '}
          <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
            <ShareIcon className="w-6" />
          </span>
        </h2>
        <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
          Share your screen to the room.
        </p>
      </div>
      <div
        className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
        rel="noopener noreferrer"
        role="button"
        onClick={async () => {
          const token = await generateToken(roomName);
          const URL = process.env.NEXT_PUBLIC_LIVEKIT_SERVER_URL;
          const room = new Room();
          await room.connect(URL, token);
          await room.localParticipant.enableCameraAndMicrophone();
        }}
      >
        <h2 className={`mb-3 text-2xl font-semibold`}>
          Share webcam{' '}
          <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
            <CameraIcon className="w-6" />
          </span>
        </h2>
        <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
          Share your webcam to the room.
        </p>
      </div>
    </div>
  );
}
