import { liveKitService } from '@/app/lib/livekit';
import { EyeSlashIcon, EyeIcon, ShareIcon } from '@heroicons/react/24/outline';

export default async function Page() {
  const rooms = await liveKitService.listRooms();

  const isRoomActive = rooms.length > 0;

  return (
    <div className="mb-32 grid text-center lg:mb-0 lg:w-full lg:max-w-5xl lg:grid-cols-4 lg:text-left">
      <div
        className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
        rel="noopener noreferrer"
        role="button"
      >
        <h2 className={`mb-3 text-2xl font-semibold`}>
          {isRoomActive ? 'Active' : 'Offline'}{' '}
          <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
            {isRoomActive ? (
              <EyeIcon className="w-6" />
            ) : (
              <EyeSlashIcon className="w-6" />
            )}
          </span>
        </h2>
        <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
          {isRoomActive
            ? 'Yay you can now stream.'
            : 'No active room. Create one to get started.'}
        </p>
      </div>
      {isRoomActive ? (
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
      ) : null}
    </div>
  );
}
