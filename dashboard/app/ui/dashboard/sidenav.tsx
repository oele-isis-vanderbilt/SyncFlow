import Link from 'next/link';
import NavLinks from '@/app/ui/dashboard/nav-links';
import AppLogo from '@/app/ui/app-logo';
import { PowerIcon } from '@heroicons/react/24/outline';
import { signOut } from '@/auth';
import { auth } from '@/auth';
import { DarkModeSwitcher } from '../dark-mode-switcher';

export default async function SideNav() {
  const session = await auth();
  return (
    <div className="flex h-full grow flex-col bg-gray-200 px-3 py-4 md:px-2 dark:bg-gray-800">
      <Link
        className="mb-2 flex h-20 items-end justify-start rounded-md p-4 md:h-40"
        href="/dashboard"
      >
        <div className="w-32 md:w-64">
          <AppLogo />
        </div>
      </Link>
      <div className="flex grow flex-row justify-between space-x-2 md:flex-col md:space-x-0 md:space-y-2">
        <div className="hidden p-2 md:block dark:text-white">
          {session?.user?.name ? `Welcome! ${session.user.id!}` : 'Welcome'}{' '}
        </div>
        <div className="dark:text-white">
          <DarkModeSwitcher />
        </div>
        <NavLinks user={session?.user} />
        <div className="hidden h-auto w-full grow rounded-md md:block dark:bg-gray-800"></div>
        <form
          action={async () => {
            'use server';
            await signOut();
          }}
        >
          <button className="flex h-[48px] w-full grow items-center justify-center gap-2 rounded-md bg-gray-200 p-3 text-sm font-medium hover:text-blue-600 md:flex-none md:justify-start md:p-2 md:px-3 dark:bg-gray-800 dark:text-white dark:hover:bg-gray-900">
            <PowerIcon className="w-6" />
            <div className="dark hidden md:block">Sign Out</div>
          </button>
        </form>
      </div>
    </div>
  );
}
