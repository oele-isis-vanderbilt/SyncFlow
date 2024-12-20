'use client';

import React, { useEffect } from 'react';
import Link from 'next/link';
import { useState } from 'react';
import { IoIosHome } from 'react-icons/io';
import { FaRegWindowClose } from 'react-icons/fa';
import { HiOutlineMenuAlt1 } from 'react-icons/hi';
import { FaSignOutAlt } from 'react-icons/fa';
import { BriefcaseIcon } from '@heroicons/react/24/outline';
import { DarkModeSwitcher } from '../dark-mode-switcher';
import { Tooltip } from 'flowbite-react';
import { signOut } from '@/app/lib/actions';
import { useParams } from 'next/navigation';
import { SiSession } from 'react-icons/si';
import { CiSettings } from 'react-icons/ci';

const MobileMenu = () => {
  const [isNavOpen, setIsNavOpen] = useState(false);
  const [projectName, setProjectName] = useState<string | null>(null);
  const params = useParams();

  useEffect(() => {
    const fetchProject = async (projectId: string) => {
      const path = `/api/project/${projectId}`;
      const response = await fetch(path);
      const data = await response.json();
      return data;
    };
    if (params.project_id) {
      fetchProject(params.project_id as string).then((data) => {
        setProjectName(data.name);
      });
    }
  }, [params]);

  const toggleNavOpen = () => {
    setIsNavOpen(!isNavOpen);
  };

  return (
    <>
      {/* biome-ignore lint/a11y/useButtonType: <explanation> */}
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
          className={`absolute top-15 left-0 z-10 h-screen w-full transform bg-white shadow transition-transform duration-1000 ease-in-out dark:bg-gray-800 ${isNavOpen ? 'translate-x-0' : '-translate-x-full'}`}
        >
          {/* biome-ignore lint/a11y/useKeyWithClickEvents: targeted for mobile devices */}
          <nav
            className="flex h-full w-full flex-col p-4"
            onClick={toggleNavOpen}
          >
            <div>
              <Link
                href="/dashboard"
                className="bg-slate-100 dark:bg-slate-700"
              >
                <div className="flex items-center gap-2 px-2">
                  <IoIosHome className="h-6 w-6" />
                  <span className="py-2 font-bold text-gray-700 text-xl transition-colors duration-300 hover:bg-gray-200 dark:text-white">
                    Overview
                  </span>
                </div>
                <hr />
              </Link>
              <Link
                href="/dashboard/projects"
                className="bg-slate-100 dark:bg-slate-700"
              >
                <div className="flex items-center gap-2 px-2">
                  <BriefcaseIcon className="h-6 w-6" />
                  <span className="py-2 font-bold text-gray-700 text-xl transition-colors duration-300 hover:bg-gray-200 dark:text-white">
                    Projects
                  </span>
                </div>
                <hr />
              </Link>
            </div>
            {params.project_id && (
              <div className="mt-10">
                <span className="mb-10 font-bold text-xl underline dark:text-white">
                  {`Project: ${projectName}`}
                </span>
                <Link
                  href={`/dashboard/projects/${params.project_id}/sessions`}
                  className="bg-slate-100 dark:bg-slate-700"
                >
                  <div className="flex items-center gap-2 px-2">
                    <SiSession className="h-6 w-6" />
                    <span className="py-2 font-bold text-gray-700 text-xl transition-colors duration-300 hover:bg-gray-200 dark:text-white">
                      Sessions
                    </span>
                  </div>
                  <hr />
                </Link>
                <Link
                  href={`/dashboard/projects/${params.project_id}/settings`}
                  className="bg-slate-100 dark:bg-slate-700"
                >
                  <div className="flex items-center gap-2 px-2">
                    <CiSettings className="h-6 w-6" />
                    <span className="py-2 font-bold text-gray-700 text-xl transition-colors duration-300 hover:bg-gray-200 dark:text-white">
                      Settings
                    </span>
                  </div>
                  <hr />
                </Link>
              </div>
            )}
            <div className="mt-10 flex flex-row items-center justify-center">
              <DarkModeSwitcher />
              <form action={signOut}>
                <Tooltip content={'Sign Out '} placement="top">
                  <button
                    type="submit"
                    aria-label="Toggle dark mode"
                    data-testid="dark-theme-toggle"
                    className="rounded-lg p-2.5 text-gray-500 text-sm hover:bg-gray-100 focus:outline-none focus:ring-4 focus:ring-gray-200 dark:text-gray-400 dark:focus:ring-gray-700 dark:hover:bg-gray-700"
                  >
                    <FaSignOutAlt className="h-5 w-5" />
                  </button>
                </Tooltip>
              </form>
            </div>
          </nav>
        </div>
      </div>
    </>
  );
};

export default MobileMenu;
