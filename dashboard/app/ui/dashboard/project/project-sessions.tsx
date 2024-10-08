'use client';
import { deleteSession, stopSession } from '@/app/lib/project-actions';
import { LivekitSessionInfo, ProjectSession } from '@/types/project';
import { Tooltip } from 'flowbite-react';
import { MdDelete } from 'react-icons/md';
import { FaStop } from 'react-icons/fa';
import InfoModal from '../info-modal';
import { useState } from 'react';
import type { InfoModalContent } from '../info-modal';

import { CustomDataTable } from './data-table';
import Link from 'next/link';
import { getDateFromTimeStamp } from '../utils';

const isActive = (session: ProjectSession): boolean => {
  return session.status === 'Started';
};

const sortedSessionsToColumns = (projectId: string) => {
  return [
    {
      name: 'Session Id',
      selector: (session) => session.id,
      cell: (session) => (
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
      selector: (session) => session.name,
      cell: (session) => (
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
      selector: (session) => session.startedAt,
      sortable: true,
    },
    {
      name: 'Participants',
      selector: (session) => session.participants,
      sortable: true,
    },
    {
      name: 'Status',
      selector: (row) => row.status,
      cell: (session) => {
        return isActive(session) ? (
          <Tooltip content="Join Session">
            <Link
              href={`/dashboard/projects/${projectId}/sessions/${session.id}/room`}
            >
              <div className="flex">
                <div className="text-black-800 dark:text-white-200 animate-pulse rounded-full bg-blue-200 px-4 py-2 text-center text-xs font-medium leading-none dark:bg-red-700">
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
      selector: (session) => session.recordings,
    },
    {
      name: 'Actions',
      selector: (session) => session.id,
      cell: (session) => {
        let [infoModalContent, setInfoModalContent] =
          useState<InfoModalContent | null>(null);

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

const sortedSessionsToTableData = (
  sessions: ProjectSession[],
  livekitSessionInfo: { [sessionId: string]: LivekitSessionInfo },
) => {
  return sessions.map((session) => {
    return {
      id: session.id,
      name: session.name,
      startedAt: getDateFromTimeStamp(session.startedAt),
      status: session.status,
      participants: livekitSessionInfo[session.id]?.participants.length || 0,
      recordings: livekitSessionInfo[session.id]?.recordings.length || 0,
    };
  });
};

export default function ProjectSessions({
  projectId,
  projectName,
  sessions,
  livekitSessionInfo,
  emptyMessage,
}: {
  projectId: string;
  projectName: string;
  sessions: ProjectSession[];
  livekitSessionInfo: { [sessionId: string]: LivekitSessionInfo };
  emptyMessage?: string;
}) {
  const sortedSessions = sessions.sort((a, b) => {
    return b.startedAt - a.startedAt;
  });

  let sessionHeaders = sortedSessionsToColumns(projectId);
  let sessionData = sortedSessionsToTableData(
    sortedSessions,
    livekitSessionInfo,
  );

  return (
    <CustomDataTable
      columns={sessionHeaders}
      data={sessionData}
      noDataComponent={
        emptyMessage ||
        `No Sessions for project found for ${projectName}. Create one to get started.`
      }
    />
  );
}
