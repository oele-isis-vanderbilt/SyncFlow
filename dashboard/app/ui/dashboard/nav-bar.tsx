'use client';

import { Navbar, Tooltip } from 'flowbite-react';
import Link from 'next/link';
import { FaSignOutAlt, FaSignInAlt } from 'react-icons/fa';
import AppLogo from '@/app/ui/app-logo';
import { DarkModeSwitcher } from '@/app/ui/dark-mode-switcher';
import { signOut } from '@/app/lib/actions';
import type { Session } from 'next-auth';

export function NavBar({ session }: { session: Session | null }) {
  return (
    <Navbar
      fluid
      theme={{
        root: {
          base: 'sticky top-0 z-[30] mx-auto flex w-full items-center justify-between border-b border-gray-200 bg-white text-gray-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400',
          inner: {
            base: 'mx-auto flex w-full max-w-8xl flex-wrap items-center justify-between px-4 py-2.5 lg:px-4',
          },
        },
      }}
    >
      <div className="flex items-center">
        <Link
          href="/"
          className="flex items-center gap-3 font-semibold text-2xl text-gray-900 dark:text-white"
        >
          <AppLogo w={200} h={200} />
        </Link>
      </div>
      <div className="flex items-center gap-2">
        <DarkModeSwitcher />
        {session?.user ? (
          <form action={signOut}>
            <Tooltip
              content={`Sign Out (${session.user.name})`}
              placement="top"
            >
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
        ) : (
          <Tooltip content={'Sign In'} placement="top">
            <Link
              href="/login"
              className="rounded-lg p-2.5 text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-4 focus:ring-gray-200 lg:block dark:text-gray-300 dark:focus:ring-gray-700 dark:hover:bg-gray-700"
            >
              <FaSignInAlt className="h-5 w-5" />
            </Link>
          </Tooltip>
        )}
      </div>
    </Navbar>
  );
}
