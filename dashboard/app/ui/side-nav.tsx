'use client';

import { Fragment, useEffect, useState } from 'react';

import Link from 'next/link';
import { twMerge } from 'tailwind-merge';
import type { ClassValue } from 'clsx';
import clsx from 'clsx';

import { FaChevronRight, FaChevronLeft } from 'react-icons/fa';
import { FaBriefcase as Briefcase } from 'react-icons/fa';
import { FaHome as Home } from 'react-icons/fa';
import { FaCog as Settings } from 'react-icons/fa';
import { SiSession } from 'react-icons/si';
import { CiSettings } from 'react-icons/ci';

import { Tooltip } from 'flowbite-react';

function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

import { useParams, usePathname } from 'next/navigation';

export const NavItems = () => {
  const pathname = usePathname();

  function isNavItemActive(pathname: string, nav: string) {
    return pathname === nav;
  }

  return [
    {
      name: 'Home',
      href: '/dashboard',
      icon: <Home size={20} />,
      active: pathname === '/dashboard',
      position: 'top',
    },
    {
      name: 'Projects',
      href: '/dashboard/projects',
      icon: <Briefcase size={20} />,
      active: isNavItemActive(pathname, '/dashboard/projects'),
      position: 'top',
    },
    {
      name: 'Settings',
      href: '#',
      icon: <Settings size={20} />,
      active: isNavItemActive(pathname, '#'),
      position: 'bottom',
    },
  ];
};

export default function SideNav() {
  const navItems = NavItems();
  const params = useParams();

  const [isSidebarExpanded, setIsSidebarExpanded] = useState(() => {
    if (typeof window !== 'undefined') {
      const saved = window.localStorage.getItem('sidebarExpanded');
      if (saved === null) {
        return true;
      }
      return JSON.parse(saved);
    }
    return true; // default state if window is not defined
  });

  useEffect(() => {
    if (typeof window !== 'undefined') {
      window.localStorage.setItem(
        'sidebarExpanded',
        JSON.stringify(isSidebarExpanded),
      );
    }
  }, [isSidebarExpanded]);

  const toggleSidebar = () => {
    setIsSidebarExpanded(!isSidebarExpanded);
  };

  return (
    <div className="md:pr-4">
      <div
        className={cn(
          isSidebarExpanded ? 'w-[200px]' : 'w-[64px]',
          'hidden h-full transform border-r bg-accent transition-all duration-300 ease-in-out sm:flex',
        )}
      >
        <aside className="flex h-full w-full columns-1 flex-col overflow-x-hidden break-words px-4">
          {/* Top */}
          <div className="relative mt-4 pb-2">
            <div className="flex flex-col space-y-1">
              {navItems.map((item, idx) => {
                if (item.position === 'top') {
                  return (
                    <Fragment key={idx}>
                      <div className="space-y-1">
                        <SideNavItem
                          label={item.name}
                          icon={item.icon}
                          path={item.href}
                          active={item.active}
                          isSidebarExpanded={isSidebarExpanded}
                        />
                      </div>
                    </Fragment>
                  );
                }
              })}
            </div>
          </div>
          {
            // Project Menu
            params.project_id && (
              <ProjectMenu
                projectId={params.project_id as string}
                isExpanded={isSidebarExpanded}
              />
            )
          }
          {/* Bottom */}
          <div className="sticky bottom-0 mt-auto mb-4 block whitespace-nowrap transition duration-200">
            {navItems.map((item, idx) => {
              if (item.position === 'bottom') {
                return (
                  <Fragment key={idx}>
                    <div className="space-y-1">
                      <SideNavItem
                        label={item.name}
                        icon={item.icon}
                        path={item.href}
                        active={item.active}
                        isSidebarExpanded={isSidebarExpanded}
                      />
                    </div>
                  </Fragment>
                );
              }
            })}
          </div>
        </aside>
        <div className="relative mt-[calc(calc(90vh)-40px)]">
          <button
            type="button"
            className="absolute right-[-12px] bottom-32 flex h-6 w-6 items-center justify-center rounded-full border border-muted-foreground/20 bg-accent shadow-md transition-shadow duration-300 ease-in-out hover:shadow-lg dark:bg-gray-900 dark:text-white"
            onClick={toggleSidebar}
          >
            {isSidebarExpanded ? (
              <FaChevronLeft size={16} className="stroke-foreground" />
            ) : (
              <FaChevronRight
                size={16}
                className="stroke-foreground dark:text-white"
              />
            )}
          </button>
        </div>
      </div>
    </div>
  );
}

export function ProjectMenu({
  projectId,
  isExpanded,
}: { projectId: string; isExpanded: boolean }) {
  const [projectName, setProjectName] = useState<string | null>(null);
  const pathName = usePathname();

  useEffect(() => {
    const path = `/api/project/${projectId}`;

    const fetchProjectName = async () => {
      const response = await fetch(path);
      const data = await response.json();
      return data.name;
    };

    fetchProjectName().then((name) => {
      setProjectName(name);
    });
  }, []);

  return (
    <div className="mt-[50px] dark:text-neutral-50">
      <h2
        className={clsx(
          'overflow-ellipsis p-2 font-bold',
          isExpanded ? 'block' : 'hidden',
        )}
      >
        {projectName}
      </h2>

      <div className="space-y-1">
        <SideNavItem
          label={'Sessions'}
          icon={<SiSession size={18} />}
          path={`/dashboard/projects/${projectId}/sessions`}
          active={pathName === `/dashboard/projects/${projectId}/sessions`}
          isSidebarExpanded={isExpanded}
        />
      </div>
      <div className="space-y-1">
        <SideNavItem
          label={'Settings'}
          icon={<CiSettings size={20} />}
          path={`/dashboard/projects/${projectId}/settings`}
          active={pathName === `/dashboard/projects/${projectId}/settings`}
          isSidebarExpanded={isExpanded}
        />
      </div>
    </div>
  );
}

export const SideNavItem: React.FC<{
  label: string;
  icon: any;
  path: string;
  active: boolean;
  isSidebarExpanded: boolean;
}> = ({ label, icon, path, active, isSidebarExpanded }) => {
  return (
    <>
      {isSidebarExpanded ? (
        <Link
          href={path}
          className={`relative flex h-full items-center whitespace-nowrap rounded-md ${
            active
              ? 'bg-gray-200 font-base text-neutral-700 text-sm shadow-sm dark:bg-gray-900 dark:text-white'
              : 'text-neutral-500 hover:bg-neutral-200 hover:text-neutral-700 dark:text-neutral-400 dark:hover:bg-neutral-800 dark:hover:text-white'
          }`}
        >
          <div className="relative flex flex-row items-center space-x-2 rounded-md px-2 py-1.5 font-base text-sm duration-100">
            {icon}
            <span>{label}</span>
          </div>
        </Link>
      ) : (
        <Tooltip content={label} placement="top">
          <Link
            href={path}
            className={`relative flex h-full items-center whitespace-nowrap rounded-md ${
              active
                ? 'bg-gray-200 font-base text-neutral-700 text-sm dark:bg-blue-900 dark:text-white'
                : 'text-neutral-500 hover:bg-neutral-200 hover:text-neutral-700 dark:text-neutral-400 dark:hover:bg-neutral-800 dark:hover:text-white'
            }`}
          >
            <div className="relative flex flex-row items-center space-x-2 rounded-md p-2 font-base text-sm duration-100">
              {icon}
            </div>
          </Link>
        </Tooltip>
      )}
    </>
  );
};
