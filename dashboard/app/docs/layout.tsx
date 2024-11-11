'use client';

import { PropsWithChildren, useState } from 'react';
import { useEffect } from 'react';

import { HiMenuAlt1, HiX } from 'react-icons/hi';
import { usePathname } from 'next/navigation';
import { twMerge } from 'tailwind-merge';
import { DOCS_SIDEBAR, type DocsSidebarItem } from '@/data/docs-sidebar';
import Link from 'next/link';
import { Navbar, Tooltip } from 'flowbite-react';
import AppLogo from '../ui/app-logo';
import { BsGithub } from 'react-icons/bs';

import '@/styles/docs.css';
import { DarkModeSwitcher } from '../ui/dark-mode-switcher';
import FooterComp from '../landing-page/Footer';

interface DocsLayoutState {
  isCollapsed: boolean;
  setCollapsed: (collapsed: boolean) => void;
}

export default function DocsLayout({ children }: PropsWithChildren) {
  const [isCollapsed, setCollapsed] = useState(true);

  const state: DocsLayoutState = {
    isCollapsed,
    setCollapsed,
  };

  return (
    <div className="mx-auto w-full max-w-8xl lg:flex dark:bg-gray-800">
      <div className="relative">
        <DocsNavbar {...state} />
        <div className="mx-auto w-full max-w-8xl lg:flex">
          <DocsSidebar {...state} />
          <div className="w-full min-w-0">{children}</div>
        </div>
        <div className="w-full">
          <FooterComp />
        </div>
      </div>
    </div>
  );
}

function DocsNavbar({ isCollapsed, setCollapsed }: DocsLayoutState) {
  return (
    <Navbar
      fluid
      theme={{
        root: {
          base: 'sticky top-0 z-[60] mx-auto flex w-full items-center justify-between border-b border-gray-200 bg-white text-gray-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400',
          inner: {
            base: 'mx-auto flex w-full max-w-8xl flex-wrap items-center justify-between px-4 py-2.5 lg:px-4',
          },
        },
      }}
    >
      <div className="flex items-center">
        {isCollapsed ? (
          <button
            type="button"
            className="-ml-3 mr-1 p-2 lg:mx-0 lg:hidden"
            onClick={() => setCollapsed(!isCollapsed)}
          >
            <HiMenuAlt1
              aria-label="Open sidebar"
              className="size-6 cursor-pointer text-gray-600 dark:text-gray-300"
            />
          </button>
        ) : (
          <button
            type="button"
            className="-ml-3 mr-1 rounded p-2 lg:mx-0 lg:hidden dark:bg-gray-700"
            onClick={() => setCollapsed(!isCollapsed)}
          >
            <HiX
              aria-label="Close sidebar"
              className="size-6 cursor-pointer text-gray-600 dark:text-gray-300"
            />
          </button>
        )}
        <Link
          href="/"
          className="flex items-center gap-3 font-semibold text-2xl text-gray-900 dark:text-white"
        >
          <AppLogo w={200} h={200} />
        </Link>
        <div className="hidden lg:ml-[4.5rem] lg:flex"></div>
      </div>
      <div className="flex items-center gap-2">
        <DarkModeSwitcher />
        <Tooltip animation={false} content="View on GitHub">
          <a
            rel="noopener"
            target="_blank"
            href="https://github.com/oele-isis-vanderbilt/SyncFlow"
            aria-label="View on GitHub"
            className="hidden rounded-lg p-2.5 text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-4 focus:ring-gray-200 lg:block dark:text-gray-300 dark:focus:ring-gray-700 dark:hover:bg-gray-700"
          >
            <BsGithub aria-hidden className="size-5" />
          </a>
        </Tooltip>
      </div>
    </Navbar>
  );
}

function DocsSidebar({ isCollapsed, setCollapsed }: DocsLayoutState) {
  const pathname = usePathname();

  // collapse sidebar on small screens when navigating to a new page
  useEffect(() => {
    if (typeof window !== 'undefined' && window.innerWidth < 768) {
      setCollapsed(true);
    }
  }, [pathname, setCollapsed]);

  return (
    <>
      <div
        className={twMerge(
          'fixed inset-0 z-50 size-full max-w-64 overflow-y-auto bg-gray-200 px-4 lg:sticky lg:top-[61px] lg:block lg:h-[calc(100vh-4rem)] dark:bg-gray-900',
          isCollapsed && 'hidden',
        )}
      >
        <nav
          aria-label="Docs navigation"
          className="px-1 pt-16 pb-8 pl-3 font-normal text-base lg:pt-2 lg:pl-0 lg:text-sm"
        >
          <ul>
            {DOCS_SIDEBAR.map((section) => (
              <SidebarSection key={section.title} title={section.title}>
                {section.items.map((item) => (
                  <SidebarItem
                    key={`section-${section.title}_item-${item.title}`}
                    onClick={() => setCollapsed(true)}
                    {...item}
                  />
                ))}
              </SidebarSection>
            ))}
          </ul>
        </nav>
      </div>
      {!isCollapsed && (
        <div
          onClick={() => setCollapsed(true)}
          onKeyUp={(key) => key.code === 'Escape' && setCollapsed(true)}
          className="fixed inset-0 z-40 bg-gray-900/50 lg:hidden dark:bg-gray-900/60"
        />
      )}
    </>
  );
}

function SidebarSection({
  title,
  children,
}: PropsWithChildren<{ title: string }>) {
  return (
    <li className="mt-8">
      <h5 className="mb-2 font-semibold text-gray-900 text-sm uppercase tracking-wide lg:text-xs dark:text-white">
        {title}
      </h5>
      <ul className="py-1">{children}</ul>
    </li>
  );
}

function SidebarItem({
  title,
  href,
  isNew,
  isExternal,
  onClick,
}: DocsSidebarItem & { onClick(): void }) {
  return (
    <li>
      <SidebarLink href={href} isExternal={isExternal} onClick={onClick}>
        {isNew ? <NewBadge>{title}</NewBadge> : title}
      </SidebarLink>
    </li>
  );
}

function SidebarLink({
  children,
  href,
  isExternal,
  onClick,
}: PropsWithChildren<{ href: string; isExternal?: boolean; onClick(): void }>) {
  const pathname = usePathname();

  return (
    <Link
      href={href}
      target={isExternal ? '_blank' : undefined}
      className={twMerge(
        'flex flex-wrap items-center py-2 font-medium',
        pathname === href
          ? 'text:black hover:text-blue-700 dark:text-white'
          : 'text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white',
      )}
      onClick={onClick}
    >
      {children}
    </Link>
  );
}

function NewBadge({ children }: PropsWithChildren) {
  return (
    <span className="flex items-center gap-2">
      {children}
      <span className="ml-2 inline-flex h-[1.1rem] items-center rounded border border-cyan-100 bg-cyan-100 px-1.5 font-semibold text-[10px] text-cyan-800 uppercase dark:border-cyan-400 dark:bg-gray-700 dark:text-cyan-400">
        new
      </span>
    </span>
  );
}
