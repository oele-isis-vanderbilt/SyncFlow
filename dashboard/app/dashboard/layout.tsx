import { NavBar } from '@/app/ui/dashboard/nav-bar';
import { auth } from '@/auth';
import SideNav from '../ui/side-nav';

export default async function Layout({
  children,
}: {
  children: React.ReactNode;
}) {
  const session = await auth();
  return (
    <>
      <NavBar session={session} withBreadCrumb={true} />
      <div className="mx-auto flex px-2">
        <SideNav />
        <div className="flex w-full flex-col">
          <div className="w-full flex-1 overflow-x-auto">
            <div className="overflow-auto sm:h-[calc(99vh-150px)] ">
              <div className="relative mx-auto flex h-[calc(100vh-240px)] w-full justify-center overflow-auto overflow-y-auto">
                <div className="w-full md:max-w-8xl">{children}</div>
              </div>
            </div>
          </div>
          <footer className="py-6 md:px-8 md:py-0 dark:border-border dark:text-white">
            <div className="container flex flex-col items-center justify-between gap-4 md:h-24 md:flex-row">
              <p className="text-balance text-center text-muted-foreground text-sm leading-loose md:text-left">
                Built by{' '}
                <a
                  href="https://teachableagents.org"
                  target="_blank"
                  rel="noreferrer"
                  className="font-medium underline underline-offset-4"
                >
                  OELE, ISIS, Vanderbilt University
                </a>
                . The source code is available on{' '}
                <a
                  href="https://github.com/oele-isis-vanderbilt/syncflow.git"
                  target="_blank"
                  rel="noreferrer"
                  className="font-medium underline underline-offset-4"
                >
                  GitHub
                </a>
                .
              </p>
            </div>
          </footer>
        </div>
      </div>
    </>
  );
}
