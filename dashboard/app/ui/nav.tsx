import { Navbar } from 'flowbite-react';
import AppLogo from './app-logo';
import Link from 'next/link';
import { DarkModeSwitcher } from './dark-mode-switcher';
import MobileMenu from './mobile-menu';

const Nav = () => {
  return (
    <Navbar
      fluid
      theme={{
        root: {
          base: 'sticky top-0 z-40 mx-auto flex w-full items-center justify-between border-b border-gray-200 bg-white py-0 text-gray-500 dark:border-gray-700 dark:bg-gray-900 dark:text-gray-400',
          inner: {
            base: 'mx-auto flex w-full max-w-none flex-wrap items-center justify-between px-4 py-2.5 xl:max-w-8xl',
          },
        },
      }}
    >
      {' '}
      <div className="container mx-auto flex flex-row justify-between">
        <div className="flex items-center gap-2">
          <Link href="/" className="sr-only">
            <AppLogo w={200} h={200} />
          </Link>
          <Link
            aria-hidden
            href="/"
            className="flex items-center gap-2 font-semibold text-2xl text-gray-900 dark:text-white"
          >
            <AppLogo w={200} h={200} />
          </Link>
          <div className="ml-4 hidden lg:flex">
            <div className="hidden items-center gap-1 xl:flex">
              <Link
                href="/docs"
                className="rounded-lg p-2.5 font-medium text-gray-900 text-lg hover:text-cyan-700 dark:text-gray-300 dark:hover:text-cyan-500"
              >
                Docs
              </Link>
            </div>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <Link
            href="/login"
            className="hidden rounded-lg p-2.5 font-medium text-gray-900 text-lg hover:text-cyan-700 lg:flex dark:text-gray-300 dark:hover:text-cyan-500"
          >
            Login!
          </Link>
          <Link
            aria-hidden
            href="/signup"
            className="flex items-center gap-3 font-semibold text-2xl text-gray-900 dark:text-white"
          >
            {/* biome-ignore lint/a11y/useButtonType: <explanation> */}
            <button className="hidden items-center gap-1.5 rounded-lg bg-blue-700 p-2.5 font-medium text-lg text-white lg:flex">
              Get Started
            </button>
          </Link>
          <DarkModeSwitcher />
          <div className="lg:hidden">
            <MobileMenu />
          </div>
        </div>
      </div>
    </Navbar>
  );
};

export default Nav;
