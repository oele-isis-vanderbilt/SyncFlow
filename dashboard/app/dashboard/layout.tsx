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
      <div className="flex">
        <SideNav />
        <div className="w-full overflow-x-auto">
          <div className="overflow-auto sm:h-[calc(99vh-60px)] ">
            <div className="- 120px)] relative mx-auto flex h-[calc(100vh w-full justify-center overflow-auto overflow-y-auto">
              <div className="w-full md:max-w-8xl">{children}</div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
