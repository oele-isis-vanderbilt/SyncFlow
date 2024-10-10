import { projectClient } from '@/app/lib/project-client';
import ErrorComponent from '@/app/ui/dashboard/project/error-component';
import ParticipantsInfo from '@/app/ui/dashboard/project/participants-info';
import RecordingsInfo from '@/app/ui/dashboard/project/recordings-info';
import { TracksInfo } from '@/app/ui/dashboard/project/tracks-info';
import { lusitana } from '@/app/ui/fonts';
import { auth } from '@/auth';
import { Tooltip } from 'flowbite-react';
import Link from 'next/link';

export default async function Page({
  params,
}: {
  params: {
    project_id: string;
    session_id: string;
  };
}) {
  const { project_id: projectId, session_id: sessionId } = params;
  let session = await auth();
  const user = session?.user;

  if (!user) {
    return (
      <ErrorComponent
        title="Error"
        error="Not authenticated"
        projectId={projectId}
      />
    );
  }
  const projectRes = await projectClient.getProject(projectId);

  let projectName = projectRes
    .map((project) => project.name)
    .unwrapOr('Unknown Project');

  return (
    await (
      await projectClient.getSession(projectId, sessionId)
    ).map(async (sessionInfo) => {
      let lkSessionInfoResult = await projectClient.getLivekitSessionInfo(
        projectId,
        sessionId,
      );
      return (
        <div
          className="flex h-full w-full flex-col p-2 dark:text-white"
          key={`${projectId}-${sessionId}`}
        >
          <div className="flex flex-row gap-2">
            <h2 className={`text-4xl font-bold ${lusitana.className}`}>
              {projectName} / {sessionInfo.name}
            </h2>
            {sessionInfo.status === 'Started' ? (
              <>
                <Tooltip content="Join Session">
                  <Link
                    href={`/dashboard/projects/${projectId}/sessions/${sessionId}/room`}
                    target="_blank"
                  >
                    <div className="flex items-center justify-center">
                      <div className="text-black-800 dark:text-white-200 animate-pulse rounded-full bg-blue-200 px-4 py-2 text-center text-xs font-medium leading-none dark:bg-red-700">
                        live
                      </div>
                    </div>
                  </Link>
                </Tooltip>
              </>
            ) : (
              <div>
                <p>{sessionInfo.status}</p>
              </div>
            )}
          </div>
          <div className="mt-2 flex h-full w-full flex-col gap-2 md:h-1/2 md:flex-row">
            <div className="h-full md:w-1/2">
              <h2 className={`text-xl ${lusitana.className}`}>Participants</h2>
              {sessionInfo.status !== 'Started' ? (
                <div className="flex h-full w-full items-center justify-center bg-gray-300 dark:bg-gray-900">
                  <h3 className={`text-2xl font-bold ${lusitana.className}`}>
                    Session has stopped
                  </h3>
                </div>
              ) : (
                <div className="flex h-full w-full flex-col gap-2">
                  <div className="flex h-full w-full flex-col gap-2">
                    {lkSessionInfoResult
                      .map((lkSessionInfo) => {
                        return (
                          <ParticipantsInfo
                            participants={lkSessionInfo.participants}
                            roomName={lkSessionInfo.roomName}
                            key={`${projectId}-${sessionId}`}
                          />
                        );
                      })
                      .unwrapOrElse((error) => {
                        return (
                          <ErrorComponent
                            title="Error fetching livekit session Info"
                            error={error}
                            projectId={projectId}
                          />
                        );
                      })}
                  </div>
                </div>
              )}
            </div>
            <div className="mt-10 h-full md:mt-0 md:w-1/2">
              <h2 className={`text-xl ${lusitana.className}`}>Tracks</h2>
              {sessionInfo.status !== 'Started' ? (
                <div className="flex h-full w-full items-center justify-center bg-gray-300 dark:bg-gray-900">
                  <h3 className={`text-2xl font-bold ${lusitana.className}`}>
                    Session has stopped
                  </h3>
                </div>
              ) : (
                <div className="flex h-full w-full flex-col gap-2">
                  {lkSessionInfoResult
                    .map((lkSessionInfo) => {
                      return (
                        <TracksInfo
                          participants={lkSessionInfo.participants}
                          key={`${projectId}-${sessionId}`}
                        />
                      );
                    })
                    .unwrapOrElse((error) => {
                      return (
                        <ErrorComponent
                          title="Error fetching livekit session Info"
                          error={error}
                          projectId={projectId}
                        />
                      );
                    })}
                </div>
              )}
            </div>
          </div>
          <div className="mt-10 flex h-full w-full flex-row md:h-12">
            <h2 className={`text-xl ${lusitana.className}`}>Recordings</h2>
          </div>
          <div className="flex h-full w-full flex-col gap-2">
            {lkSessionInfoResult
              .map((lkSessionInfo) => {
                return (
                  <RecordingsInfo
                    egresses={lkSessionInfo.recordings}
                    participants={lkSessionInfo.participants}
                    key={`${projectId}-${sessionId}`}
                  />
                );
              })
              .unwrapOrElse((error) => {
                return (
                  <ErrorComponent
                    title="Error fetching recordings"
                    error={error}
                    projectId={projectId}
                  />
                );
              })}
          </div>
        </div>
      );
    })
  ).unwrapOrElse((error) => {
    return (
      <ErrorComponent
        title="Error fetching session"
        error={error}
        projectId={projectId}
      />
    );
  });
}
