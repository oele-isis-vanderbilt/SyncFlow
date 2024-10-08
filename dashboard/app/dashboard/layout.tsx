import SideNav from '@/app/ui/dashboard/sidenav2';
import { SessionProvider } from 'next-auth/react';
import NextBreadcrumb from '../ui/breadcrumb';

export default async function Layout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <div className="flex h-full flex-row overflow-hidden dark:bg-gray-800">
      <SessionProvider>
        <SideNav />
      </SessionProvider>
      <div className="flex flex-1 flex-col md:overflow-y-auto">
        <NextBreadcrumb
          homeElement={'Home'}
          separator={<span> {'>'} </span>}
          activeClasses="dark:text-amber-500 underline text-blue-900"
          containerClasses="flex py-5 dark:bg-gray-900 dark:text-white bg-gray-200"
          listClasses="hover:underline mx-2 font-bold"
        />
        <main className="flex-1 overflow-y-auto p-5">{children}</main>
        <div className="w-full text-center">
          <footer className="p-5 dark:text-white">
            <p className="text-sm">
              &copy; {new Date().getFullYear()}{' '}
              <a
                href="https://wp0.vanderbilt.edu/oele/"
                target="_blank"
                rel="noopener noreferrer"
                className={'hover:text-blue-400 hover:underline'}
              >
                Vanderbilt University, Open Ended Learning Environments Lab
              </a>
            </p>
          </footer>
        </div>
      </div>
    </div>
  );
}
