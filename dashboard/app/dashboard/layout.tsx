import { NavBar } from '@/app/ui/dashboard/nav-bar';
import { auth } from '@/auth';
import Link from 'next/link';
import { lusitana } from '../ui/fonts';
import SideNav from '../ui/side-nav';
import { SessionProvider } from 'next-auth/react';
import NextBreadcrumb from '../ui/breadcrumb';

export default async function Layout({
  children,
}: {
  children: React.ReactNode;
}) {
  const session = await auth();
  return (
    <div className="h-screen overflow-hidden">
      <NavBar session={session} withBreadCrumb={false} />
      <div className="mb-20 flex">
        <SessionProvider>
          <SideNav />
        </SessionProvider>
        <div className="w-full overflow-x-auto">
          <div className="overflow-auto sm:h-[calc(99vh-60px)] ">
            <div className="- 120px)] relative mx-auto flex h-[calc(100vh w-full justify-center overflow-auto overflow-y-auto">
              <div className="flex w-full flex-col pb-10">
                <div className="w-full md:max-w-8xl">{children}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <footer className="sticky bottom-0 z-[30] h-10 w-full border-gray-200 border-t bg-white text-center text-gray-500 md:h-15 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400">
        <span className={`text-sm ${lusitana.className}`}>
          &copy; {new Date().getFullYear()} -{' '}
          <Link
            href="https://teachableagents.org"
            target="_blank"
            className="underline"
          >
            OELE, ISIS, Vanderbilt University
          </Link>
        </span>
      </footer>
    </div>
  );
}
