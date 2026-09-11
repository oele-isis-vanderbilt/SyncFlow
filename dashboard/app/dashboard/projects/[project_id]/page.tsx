import { projectClient } from '@/app/lib/project-client';
import { lusitana } from '@/app/ui/fonts';
import { ProjectSummaryComponent } from '@/app/ui/dashboard/project/summary-components';
import { CreateSession } from '@/app/ui/dashboard/project/create-session';
import ErrorComponent from '@/app/ui/dashboard/project/error-component';
import type { ProjectSession } from '@/types/project';
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
      const projectDevices = (
        await projectClient.listDevices(project.id)
      ).unwrapOr([]);

      return (
        <div className="flex h-full w-full flex-col p-2 dark:text-white">
          <ProjectHeader projectName={project.name} projectId={project.id} />
          <h3
            className={`mt-3 font-semibold text-xl lg:text-3xl ${lusitana.className}`}
          >
            Stats
          </h3>
          <div className="p-2">
            <ProjectSummaryComponent projectId={project.id} />
          </div>
          <div className="flex flex-row items-center text-center">
            <div>
              <h1
                className={`${lusitana.className} mt-4 mb-4 font-bold text-xl lg:text-3xl dark:text-white`}
              >
                Sessions
              </h1>
            </div>
            <CreateSession project={project} devices={projectDevices} />
            <Link
              href={`/dashboard/projects/${project.id}/sessions`}
              className="ml-2"
            >
              <Tooltip content="View All Sessions">
                <CiCirclePlus className="text-2xl hover:text-red-700" />
              </Tooltip>
            </Link>
          </div>
          <div className="h-full w-full">
            {(
              await (
                await projectClient.getSessions(project.id)
              ).mapAsync(async (sessions: ProjectSession[]) => {
                return (
                  <ProjectSessions
                    sessions={sessions}
                    projectId={project.id}
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
