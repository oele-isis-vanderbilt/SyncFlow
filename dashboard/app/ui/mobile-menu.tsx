'use client';

import React from 'react';
import Link from 'next/link';
import { useState } from 'react';
import { IoIosHome } from 'react-icons/io';
import { FaRegWindowClose } from 'react-icons/fa';
import { HiOutlineMenuAlt1 } from 'react-icons/hi';
import { SiDocsdotrs } from 'react-icons/si';
import { IoIosLogIn } from 'react-icons/io';

const MobileMenu = () => {
  const [isNavOpen, setIsNavOpen] = useState(false);

  const toggleNavOpen = () => {
    setIsNavOpen(!isNavOpen);
  };

  return (
    <>
      <button className="flex flex-col items-center justify-center">
        {!isNavOpen && (
          <HiOutlineMenuAlt1 onClick={toggleNavOpen} className="h-6 w-6" />
        )}
        {isNavOpen && (
          <FaRegWindowClose onClick={toggleNavOpen} className="h-6 w-6" />
        )}
      </button>
      <div className={`${isNavOpen ? 'block' : 'hidden'} lg:hidden`}>
        <div
          className={`absolute top-20 left-0 z-10 h-screen w-full transform bg-white shadow transition-transform duration-1000 ease-in-out dark:bg-gray-800 ${isNavOpen ? 'translate-x-0' : '-translate-x-full'}`}
        >
          <nav
            className="flex h-full w-full flex-col p-4"
            onClick={toggleNavOpen}
          >
            <div>
              <Link href="/" className="bg-slate-100 dark:bg-slate-700">
                <div className="flex items-center gap-2 px-2">
                  <IoIosHome className="h-6 w-6" />
                  <span className="py-2 font-bold text-gray-700 text-xl transition-colors duration-300 hover:bg-gray-200 dark:text-white">
                    Home
                  </span>
                </div>
                <hr />
              </Link>
              <Link href="/docs" className="bg-slate-100 dark:bg-slate-700">
                <div className="flex items-center gap-2 px-2">
                  <SiDocsdotrs className="h-6 w-6" />
                  <span className="py-2 font-bold text-gray-700 text-xl transition-colors duration-300 hover:bg-gray-200 dark:text-white">
                    Docs
                  </span>
                </div>
                <hr />
              </Link>
              <Link href="/login" className="bg-slate-100 dark:bg-slate-700">
                <div className="flex items-center gap-2 px-2">
                  <IoIosLogIn className="h-6 w-6" />
                  <span className="py-2 font-bold text-gray-700 text-xl transition-colors duration-300 hover:bg-gray-200 dark:text-white">
                    Login
                  </span>
                </div>
                <hr />
              </Link>
            </div>
            <div className="mt-10">
              <Link href="/docs">
                <button className="w-full items-center gap-1.5 rounded-lg bg-blue-700 p-2.5 font-medium text-lg text-white">
                  Get Started
                </button>
              </Link>
            </div>
          </nav>
        </div>
      </div>
    </>
  );
};

export default MobileMenu;
