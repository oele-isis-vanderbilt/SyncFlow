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
import { isAdmin } from '@/app/lib/utils';

// Map of links to display in the side navigation.
const links = [
  {
    name: 'Home',
    href: '/dashboard',
    icon: HomeIcon,
  },
  {
    name: 'LiveKit Settings',
    href: '/dashboard/settings',
    icon: SettingsIcon,
  },
  {
    name: 'Recordings',
    href: '/dashboard/recordings',
    icon: BsRecordBtn,
    adminOnly: true,
  },
];

export default function NavLinks({ user }: { user: SessionUser | undefined }) {
  const pathName = usePathname();
  let linksToDisplay = links.filter((link) => !link.adminOnly || isAdmin(user));
  return (
    <>
      {linksToDisplay.map((link) => {
        const LinkIcon = link.icon;
        return (
          <Link
            key={link.name}
            href={link.href}
            className={clsx(
              'flex h-[48px] grow items-center justify-center gap-2 rounded-md bg-gray-900 p-3 text-sm font-medium hover:bg-sky-100 hover:text-blue-600 md:flex-none md:justify-start md:p-2 md:px-3',
              {
                'bg-sky-100 text-gray-600': pathName === link.href,
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
