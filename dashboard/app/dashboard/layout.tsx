import Link from 'next/link';
import NextBreadcrumb from '../ui/breadcrumb';
import AppLogo from '../ui/app-logo';
import { DarkModeSwitcher } from '../ui/dark-mode-switcher';
import { NavBar } from '@/app/ui/dashboard/nav-bar';
import { auth } from '@/auth';

export default async function Layout({
  children,
}: {
  children: React.ReactNode;
}) {
  const session = await auth();
  return (
    <div className="mx-auto h-full w-full max-w-8xl lg:flex dark:bg-gray-800">
      <div className="relative w-full">
        <NavBar session={session} />
        <div className="mx-auto w-full max-w-8xl lg:flex">
          <div className="w-full min-w-0 p-2">{children}</div>
        </div>
      </div>
    </div>
  );
}
