'use client';
import { deleteSession, stopSession } from '@/app/lib/project-actions';
import { ProjectSession } from '@/types/project';
import { Tooltip } from 'flowbite-react';
import { MdDelete } from 'react-icons/md';
import { FaStop } from 'react-icons/fa';
import InfoModal from '../info-modal';
import { useState } from 'react';
import type { InfoModalContent } from '../info-modal';

import { CustomDataTable } from './data-table';
import Link from 'next/link';
import { getDateFromTimeStamp } from '../utils';
import DataSharingPrompt from './data-sharing-prompt';

const isActive = (session: ProjectSession): boolean => {
  return session.status === 'Started';
};

const sortedSessionsToColumns = (projectId: string) => {
  return [
    {
      name: 'Session Id',
      selector: (session: ProjectSession) => session.id,
      cell: (session: ProjectSession) => (
        <Link
          href={`/dashboard/projects/${projectId}/sessions/${session.id}`}
          className="text-xs hover:underline"
        >
          {session.id}
        </Link>
      ),
    },
    {
      name: 'Session Name',
      selector: (session: ProjectSession) => session.name,
      cell: (session: ProjectSession) => (
        <Link
          href={`/dashboard/projects/${projectId}/sessions/${session.id}`}
          className="text-xs hover:underline"
        >
          {session.name}
        </Link>
      ),
      sortable: true,
    },
    {
      name: 'Start Time',
      selector: (session: ProjectSession) =>
        getDateFromTimeStamp(session.startedAt),
      sortable: true,
    },
    {
      name: 'Participants',
      selector: (session: ProjectSession) => session.numParticipants,
      sortable: true,
    },
    {
      name: 'Status',
      selector: (session: ProjectSession) => session.status,
      cell: (session: ProjectSession) => {
        return isActive(session) ? (
          <Tooltip content="Join Session">
            <Link
              href={`/dashboard/projects/${projectId}/sessions/${session.id}/room`}
            >
              <div className="flex">
                <div className="animate-pulse rounded-full bg-blue-200 px-4 py-2 text-center font-medium text-black-800 text-xs leading-none dark:bg-red-700 dark:text-white-200">
                  live
                </div>
              </div>
            </Link>
          </Tooltip>
        ) : (
          <div>
            <p>{session.status}</p>
          </div>
        );
      },
    },
    {
      name: 'Recordings',
      selector: (session: ProjectSession) => session.numRecordings,
    },
    {
      name: 'Actions',
      selector: (session: ProjectSession) => session.id,
      cell: (session: ProjectSession) => {
        /* eslint-disable react-hooks/rules-of-hooks */
        let [infoModalContent, setInfoModalContent] =
          useState<InfoModalContent | null>(null);
        /* eslint-enable react-hooks/rules-of-hooks */

        return (
          <div className="flex flex-row gap-2">
            <InfoModal
              content={infoModalContent}
              onClose={() => setInfoModalContent(null)}
              show={infoModalContent !== null}
            />
            {isActive(session) ? (
              <button
                onClick={async () => {
                  let result = await stopSession(projectId, session.id);
                  if (!result.success) {
                    setInfoModalContent({
                      title: 'Error',
                      headings: [
                        {
                          title: 'Error stopping session',
                          items: [
                            {
                              title: 'Error',
                              content: result.error,
                            },
                          ],
                        },
                      ],
                    });
                  } else {
                    setInfoModalContent(null);
                  }
                }}
              >
                <Tooltip content="Stop Session">
                  <FaStop className="cursor-pointer text-2xl text-red-600 hover:text-red-900" />
                </Tooltip>
              </button>
            ) : null}
            {isActive(session) ? (
              <DataSharingPrompt projectId={projectId} session={session} />
            ) : null}
            <button
              onClick={async () => {
                const result = await deleteSession(projectId, session.id);
                if (!result.success) {
                  setInfoModalContent({
                    title: 'Error',
                    headings: [
                      {
                        title: 'Error deleting session',
                        items: [
                          {
                            title: 'Error',
                            content: result.error,
                          },
                        ],
                      },
                    ],
                  });
                } else {
                  setInfoModalContent(null);
                }
              }}
            >
              <Tooltip content="Delete session">
                <MdDelete className="cursor-pointer text-3xl text-red-600 hover:text-red-900" />
              </Tooltip>
            </button>
          </div>
        );
      },
    },
  ];
};

export default function ProjectSessions({
  projectId,
  projectName,
  sessions,
  emptyMessage,
}: {
  projectId: string;
  projectName: string;
  sessions: ProjectSession[];
  emptyMessage?: string;
}) {
  const sortedSessions = sessions.sort((a, b) => {
    return b.startedAt - a.startedAt;
  });

  let sessionHeaders = sortedSessionsToColumns(projectId);

  return (
    <CustomDataTable
      columns={sessionHeaders}
      data={sortedSessions}
      noDataComponent={
        emptyMessage ||
        `No Sessions for project found for ${projectName}. Create one to get started.`
      }
    />
  );
}
