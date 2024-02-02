'use client';

import { createRoom } from '@/app/lib/actions';
import { EyeSlashIcon } from '@heroicons/react/24/outline';
export default function Page() {
  return (
    <div className="mb-32 grid text-center lg:mb-0 lg:w-full lg:max-w-5xl lg:grid-cols-4 lg:text-left">
      <div
        className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30"
        rel="noopener noreferrer"
        role="button"
        onClick={async () => {
          await createRoom();
        }}
      >
        <h2 className={`mb-3 text-2xl font-semibold opacity-70`}>
          Offline{' '}
          <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
            <EyeSlashIcon className="w-6" />
          </span>
        </h2>
        <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
          Click to create a room in LiveKit
        </p>
      </div>
    </div>
  );
}
