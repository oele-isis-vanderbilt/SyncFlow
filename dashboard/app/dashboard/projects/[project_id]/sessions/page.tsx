import { projectClient } from '@/app/lib/project-client';
import { lusitana } from '@/app/ui/fonts';
import { CreateSession } from '@/app/ui/dashboard/project/create-session';
import ErrorComponent from '@/app/ui/dashboard/project/error-component';
import { ProjectSession } from '@/types/project';

import dynamic from 'next/dynamic';
import { ProjectHeader } from '@/app/ui/dashboard/project/project-header';

const ProjectSessions = dynamic(
  () => import('@/app/ui/dashboard/project/project-sessions'),
  {
    ssr: false,
  },
);

export default async function Project({
  params,
}: {
  params: { project_id: string };
}) {
  const id = params.project_id;

  const isActive = (session: ProjectSession) => {
    return session.status === 'Started';
  };

  const sessionResult = await projectClient.getSessions(id);
  let projectDevices = (await projectClient.listDevices(id)).unwrapOr([]);

  return (
    await (
      await projectClient.getProject(id)
    ).mapAsync(async (project) => {
      return (
        <div className="flex flex-col p-2 dark:text-white">
          <ProjectHeader projectName={project.name} projectId={project.id} />
          <div className="flex flex-row items-center text-center">
            <div>
              <h1
                className={`${lusitana.className} mb-4 mt-4 text-3xl font-bold dark:text-white`}
              >
                Active Sessions
              </h1>
            </div>
            <CreateSession project={project} devices={projectDevices} />
          </div>
          <div className="min-h-20">
            {(
              await sessionResult.mapAsync(async (sessions) => {
                const activeSessions = sessions.filter(isActive);
                let participantsCount = {};
                for (const session of activeSessions) {
                  let sessionInfo = await projectClient.getLivekitSessionInfo(
                    project.id,
                    session.id,
                  );
                  participantsCount[session.id] = sessionInfo.unwrapOr(null);
                }
                return (
                  <ProjectSessions
                    key={project.id}
                    sessions={activeSessions}
                    projectId={project.id}
                    livekitSessionInfo={participantsCount}
                    projectName={project.name}
                    emptyMessage={`No active sessions for ${project.name}. Create a new session to get started.`}
                  />
                );
              })
            ).unwrapOrElse((error) => {
              return (
                <ErrorComponent
                  title="An error occurred while fetching the project's session information"
                  error={error}
                  projectId={project.id}
                />
              );
            })}
          </div>
          <div className="flex flex-row items-center text-center">
            <div>
              <h1
                className={`${lusitana.className} mb-4 mt-4 text-3xl font-bold dark:text-white`}
              >
                Stopped Sessions
              </h1>
            </div>
          </div>
          <div className="min-h-20">
            {sessionResult
              .map((sessions) => {
                const stoppedSessions = sessions.filter(
                  (session) => !isActive(session),
                );
                return (
                  <ProjectSessions
                    key={project.id}
                    sessions={stoppedSessions}
                    projectId={project.id}
                    livekitSessionInfo={{}}
                    projectName={project.name}
                  />
                );
              })
              .unwrapOrElse((error) => {
                return (
                  <ErrorComponent
                    title="An error occurred while fetching the project's session information"
                    error={error}
                    projectId={project.id}
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
        title="An error occurred while fetching the project information"
        error={error}
        projectId={id}
      />
    );
  });
}
