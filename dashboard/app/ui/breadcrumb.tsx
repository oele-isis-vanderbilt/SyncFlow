'use client';

import React, { ReactNode, useState } from 'react';

import { useParams, usePathname } from 'next/navigation';
import Link from 'next/link';
import { useEffect } from 'react';
import clsx from 'clsx';

type TBreadCrumbProps = {
  homeElement: ReactNode;
  separator: ReactNode;
  containerClasses?: string;
  listClasses?: string;
  activeClasses?: string;
  capitalizeLinks?: boolean;
};

type Path = {
  displayName: string;
  link: string;
};

const NextBreadcrumb = ({
  homeElement,
  separator,
  containerClasses,
  listClasses,
  activeClasses,
  capitalizeLinks,
}: TBreadCrumbProps) => {
  const paths = usePathname();
  const [pathNames, setPathNames] = useState<Path[]>([]);
  const pathParams = useParams();

  useEffect(() => {
    const projectFetch = async (projectId: string) => {
      const path = `/api/project/${projectId}`;
      const response = await fetch(path);
      const data = await response.json();
      return data;
    };

    const sessionFetch = async (projectId: string, sessionId: string) => {
      const path = `/api/project/${projectId}/session/${sessionId}`;
      const response = await fetch(path);
      const data = await response.json();
      return data;
    };

    const getPathNamesWithProjectAndSessions = async () => {
      let splitPaths = paths.split('/').filter((path) => path !== '');
      return await Promise.all(
        splitPaths.map(async (path) => {
          if (path === pathParams.project_id) {
            let name = (await projectFetch(pathParams.project_id)).name;
            return {
              displayName: name,
              link: path,
            };
          } else if (path === pathParams.session_id) {
            let sessionResult = await sessionFetch(
              pathParams.project_id as string,
              pathParams.session_id,
            );
            let name = sessionResult.name;
            return {
              displayName: name,
              link: path,
            };
          } else {
            return {
              displayName: path,
              link: path,
            };
          }
        }),
      );
    };

    getPathNamesWithProjectAndSessions()
      .then((pathNames) => {
        setPathNames(pathNames);
      })
      .catch((error) => {
        console.error(error);
      });
  }, [paths, pathParams]);

  return (
    <div>
      <ul className={containerClasses}>
        {pathNames.map((link, index) => {
          const href = `/${pathNames
            .map((p) => p.link)
            .slice(0, index + 1)
            .join('/')}`;
          const itemClasses =
            paths === href ? `${listClasses} ${activeClasses}` : listClasses;
          const itemLink = capitalizeLinks
            ? link.displayName[0].toUpperCase() +
              link.displayName.slice(1, link.displayName.length)
            : link.displayName;
          return (
            <React.Fragment key={index}>
              <li
                className={clsx(
                  itemClasses,
                  index !== pathNames.length - 2 && 'hidden md:block',
                )}
              >
                <Link href={href}>{itemLink}</Link>
              </li>
              {pathNames.length !== index + 1 && (
                <div className="hidden items-center justify-center md:flex">
                  {separator}
                </div>
              )}
            </React.Fragment>
          );
        })}
      </ul>
    </div>
  );
};

export default NextBreadcrumb;
