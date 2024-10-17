import clsx from 'clsx';
import { Tooltip } from 'flowbite-react';

import { FaSignOutAlt } from 'react-icons/fa';
import Sidebar from './sidebar';
import { auth, signOut } from '@/auth';
import { DarkModeSwitcher } from '../dark-mode-switcher';

export default async function SideNav() {
  const session = await auth();
  const user = session?.user;
  return (
    <>
      <div
        className={clsx(
          'hidden h-full w-64 flex-col border-r bg-gray-200 pt-10 transition-transform duration-500 md:flex dark:bg-gray-900',
        )}
      >
        <div className="flex-1">
          <Sidebar />
        </div>
        <div className="flex items-center justify-center gap-2 p-2 text-black dark:text-white">
          <div>
            <DarkModeSwitcher />
          </div>
          <div>
            <form
              action={async () => {
                'use server';
                await signOut();
              }}
            >
              <Tooltip content="Sign Out" placement="top">
                <button
                  type="submit"
                  aria-label="Toggle dark mode"
                  data-testid="dark-theme-toggle"
                  className="rounded-lg p-2.5 text-gray-500 text-sm hover:bg-gray-100 focus:outline-none focus:ring-4 focus:ring-gray-200 dark:text-gray-400 dark:focus:ring-gray-700 dark:hover:bg-gray-700"
                >
                  <FaSignOutAlt className="h-5 w-5" />
                </button>
              </Tooltip>
            </form>
          </div>
          <div className="ml-2 text-xs">
            Welcome <span className="underline">{user?.name}</span>
          </div>
        </div>
      </div>
    </>
  );
}
