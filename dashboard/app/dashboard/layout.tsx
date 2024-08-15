import SideNav from '@/app/ui/dashboard/sidenav';

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex h-full flex-col md:flex-row md:overflow-hidden dark:bg-gray-800">
      <div className="w-full  flex-none border-r md:w-64">
        <SideNav />
      </div>
      <div className="flex-grow py-2 md:overflow-y-auto md:py-5">
        {children}
      </div>
    </div>
  );
}
