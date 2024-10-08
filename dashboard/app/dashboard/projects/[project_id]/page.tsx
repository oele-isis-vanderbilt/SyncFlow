import { projectClient } from '@/app/lib/project-client';
import { lusitana } from '@/app/ui/fonts';
import { ProjectSummaryComponent } from '@/app/ui/dashboard/project/summary-components';
import { CreateSession } from '@/app/ui/dashboard/project/create-session';
import ErrorComponent from '@/app/ui/dashboard/project/error-component';
import { ProjectSession } from '@/types/project';
import { CiCirclePlus } from 'react-icons/ci';

import dynamic from 'next/dynamic';
import Link from 'next/link';
import { Tooltip } from 'flowbite-react';
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

  return (
    await (
      await projectClient.getProject(id)
    ).mapAsync(async (project) => {
      return (
        <div className="flex flex-col p-2 dark:text-white">
          <ProjectHeader projectName={project.name} projectId={project.id} />
          <h3 className={`mt-3 text-3xl font-semibold ${lusitana.className}`}>
            Stats
          </h3>
          <div className="p-2">
            <ProjectSummaryComponent projectId={project.id} />
          </div>
          <div className="flex flex-row items-center text-center">
            <div>
              <h1
                className={`${lusitana.className} mb-4 mt-4 text-3xl font-bold dark:text-white`}
              >
                Sessions
              </h1>
            </div>
            <CreateSession project={project} />
            <Link
              href={`/dashboard/projects/${project.id}/sessions`}
              className="ml-2"
            >
              <Tooltip content="View All Sessions">
                <CiCirclePlus className="text-2xl hover:text-red-700" />
              </Tooltip>
            </Link>
          </div>
          <div>
            {(
              await (
                await projectClient.getSessions(project.id)
              ).mapAsync(async (sessions) => {
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
                    sessions={sessions}
                    projectId={project.id}
                    livekitSessionInfo={participantsCount}
                    projectName={project.name}
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
