'use client';

import {
  HomeIcon,
  Cog8ToothIcon as SettingsIcon,
} from '@heroicons/react/24/outline';

import { BsRecordBtn } from 'react-icons/bs';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import clsx from 'clsx';
import { SessionUser } from '@/types/next-auth';

// Map of links to display in the side navigation.
const links = [
  {
    name: 'Home',
    href: '/dashboard',
    icon: HomeIcon,
  },
  {
    name: 'Settings',
    href: '/dashboard/settings',
    icon: SettingsIcon,
  },
  {
    name: 'Recordings',
    href: '/dashboard/recordings',
    icon: BsRecordBtn,
  },
];

export default function NavLinks({ user }: { user: SessionUser | undefined }) {
  const pathName = usePathname();
  let linksToDisplay = links;
  return (
    <>
      {linksToDisplay.map((link) => {
        const LinkIcon = link.icon;
        return (
          <Link
            key={link.name}
            href={link.href}
            className={clsx(
              'flex h-[48px] grow items-center justify-center gap-2 rounded-md p-3 text-sm font-medium hover:text-blue-600 md:flex-none md:justify-start md:p-2 md:px-3 dark:text-white dark:hover:bg-gray-900',
              {
                'bg-sky-100 dark:bg-gray-800 dark:bg-gray-900 dark:text-white dark:hover:bg-gray-900':
                  pathName === link.href,
              },
            )}
          >
            <LinkIcon className="w-6" />
            <p className="hidden md:block">{link.name}</p>
          </Link>
        );
      })}
    </>
  );
}
