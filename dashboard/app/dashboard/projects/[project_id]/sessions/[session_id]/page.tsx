import { projectClient } from '@/app/lib/project-client';
import DataSharingPrompt from '@/app/ui/dashboard/project/data-sharing-prompt';
import ErrorComponent from '@/app/ui/dashboard/project/error-component';
import ParticipantsInfo from '@/app/ui/dashboard/project/participants-info';
import { RecordingsInfo } from '@/app/ui/dashboard/project/recordings-info';
import { TracksInfo } from '@/app/ui/dashboard/project/tracks-info';
import { lusitana } from '@/app/ui/fonts';
import { auth } from '@/auth';
import { Project, ProjectSession } from '@/types/project';
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
  const session = await auth();
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

  const projectName = projectRes
    .map((project: Project) => project.name)
    .unwrapOr('Unknown Project');

  return (
    await (
      await projectClient.getSession(projectId, sessionId)
    ).map((sessionInfo: ProjectSession) => {
      return (
        <div
          className="flex h-full w-full flex-col p-2 dark:text-white"
          key={`${projectId}-${sessionId}`}
        >
          <div className="flex flex-row gap-2">
            <h2
              className={`font-bold text-lg lg:text-4xl ${lusitana.className}`}
            >
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
                      <div className="animate-pulse rounded-full bg-blue-200 px-4 py-2 text-center font-medium text-black-800 text-xs leading-none dark:bg-red-700 dark:text-white-200">
                        live
                      </div>
                    </div>
                  </Link>
                </Tooltip>
                <div className="-mt-3 flex items-center justify-center">
                  <DataSharingPrompt
                    projectId={projectId}
                    session={sessionInfo}
                  />
                </div>
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
              <div className="flex h-full w-full flex-col gap-2">
                <div className="flex h-full w-full flex-col gap-2">
                  <ParticipantsInfo
                    participants={sessionInfo.participants || []}
                    roomName={sessionInfo.livekitRoomName}
                    key={`${projectId}-${sessionId}`}
                  />
                </div>
              </div>
            </div>
            <div className="mt-10 h-full md:mt-0 md:w-1/2">
              <h2 className={`text-xl ${lusitana.className}`}>Tracks</h2>
              <div className="flex h-full w-full flex-col gap-2">
                <TracksInfo participants={sessionInfo.participants || []} />
              </div>
            </div>
          </div>
          <div className="mt-10 flex h-full w-full flex-row md:h-12">
            <h2 className={`text-xl ${lusitana.className}`}>Recordings</h2>
          </div>
          <div className="flex h-full w-full flex-col gap-2">
            <RecordingsInfo
              egresses={sessionInfo.recordings || []}
              key={`${projectId}-${sessionId}`}
              projectId={projectId}
              sessionId={sessionId}
            />
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
