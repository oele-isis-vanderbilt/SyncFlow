import { NavBar } from '@/app/ui/dashboard/nav-bar';
import { auth } from '@/auth';
import SideNav from '../ui/side-nav';
import { Footer, FooterCopyright } from 'flowbite-react';

export default async function Layout({
  children,
}: {
  children: React.ReactNode;
}) {
  const session = await auth();
  return (
    <>
      <NavBar session={session} withBreadCrumb={true} />
      <div className="mx-auto flex px-2 md:max-w-8xl">
        <SideNav />
        <div className="mb-24 flex h-full w-full flex-col md:mb-0">
          <div className="h-full w-full flex-1 overflow-x-auto">
            <div className="overflow-auto sm:h-[calc(99vh-150px)]">
              <div className="scrollbar-thin scrollbar-thumb-slate-700 scrollbar-track-slate-300 relative mx-auto flex h-full w-full justify-center overflow-auto overflow-y-auto">
                <div className="w-full md:max-w-8xl">{children}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <Footer className="fixed bottom-0 w-full bg-white dark:bg-gray-900">
        <div className="mx-auto flex w-full flex-col py-6 lg:py-6">
          <div className="flex items-center justify-center px-4 text-center">
            <FooterCopyright
              href="https://teachableagents.org"
              by="Open Ended Learning Environments Lab, Vanderbilt University"
              year={new Date().getFullYear()}
            />
          </div>
        </div>
      </Footer>
    </>
  );
}
