'use client';
import { Sidebar } from 'flowbite-react';
import Link from 'next/link';
import { HiChartPie, HiViewBoards } from 'react-icons/hi';
import { MdOutlineSettings } from 'react-icons/md';
import AppLogo from '../app-logo';
import { usePathname } from 'next/navigation';

const links = [
  {
    name: 'Overview',
    href: '/dashboard',
    icon: HiChartPie,
  },
  {
    name: 'Projects',
    href: '/dashboard/projects',
    icon: HiViewBoards,
  },
  {
    name: 'Settings',
    href: '/dashboard/settings',
    icon: MdOutlineSettings,
    pathRegex:
      '/dashboard/projects/[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$',
  },
];

export default function SidebarComponent() {
  const currentPath = usePathname();
  return (
    <div>
      <Link
        className="mb-2 hidden h-20 items-end justify-start rounded-md p-4 md:flex md:h-40"
        href="/dashboard"
      >
        <div className="w-32 md:w-64">
          <AppLogo />
        </div>
      </Link>
      <Sidebar
        aria-label="Sidebar with content separator example"
        theme={{
          root: {
            inner: 'border-r bg-gray-200 dark:bg-gray-900',
          },
        }}
      >
        <Sidebar.Items>
          <Sidebar.ItemGroup>
            {links
              .filter((link) => {
                if (link.pathRegex) {
                  return new RegExp(link.pathRegex).test(currentPath);
                }
                return true;
              })
              .map((link, index) => {
                return (
                  <Sidebar.Item
                    key={index}
                    icon={link.icon}
                    href={link.href}
                    className={
                      currentPath === link.href
                        ? 'bg-sky-100 dark:bg-slate-600 dark:text-white dark:hover:bg-gray-900'
                        : ''
                    }
                  >
                    {link.name}
                  </Sidebar.Item>
                );
              })}
          </Sidebar.ItemGroup>
        </Sidebar.Items>
      </Sidebar>
    </div>
  );
}
