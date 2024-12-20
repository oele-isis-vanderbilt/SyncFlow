import { projectClient } from '@/app/lib/project-client';
import CreateApiKeys from '@/app/ui/dashboard/project/create-api-keys';
import ErrorComponent from '@/app/ui/dashboard/project/error-component';
import { lusitana } from '@/app/ui/fonts';
import dynamic from 'next/dynamic';

const ProjectDevices = dynamic(
  () => import('@/app/ui/dashboard/project/devices'),
  {
    ssr: false,
  },
);

const ApiKeysTable = dynamic(
  () => import('@/app/ui/dashboard/project/api-keys'),
  {
    ssr: false,
  },
);

export default async function Page({
  params,
}: {
  params: { project_id: string };
}) {
  let projectId = params.project_id;

  let projectApiKeysResult = await projectClient.listApiKeys(projectId);
  let projectDevicesResult = await projectClient.listDevices(projectId);

  return (
    <div className="flex h-full w-full flex-col p-2 dark:text-white">
      <h2 className={`font-bold text-2xl md:text-4xl ${lusitana.className}`}>
        {' '}
        Project Settings{' '}
      </h2>
      <div className="mt-4 flex flex-row items-center text-center">
        <div>
          <h3
            className={`font-semibold text-xl md:text-3xl ${lusitana.className}`}
          >
            API Keys
          </h3>
        </div>
        <div>
          <CreateApiKeys projectId={projectId} />
        </div>
      </div>
      <div>
        {projectApiKeysResult
          .map((apiKeys) => {
            return (
              <ApiKeysTable
                apiKeys={apiKeys}
                key={projectId}
                projectId={projectId}
              />
            );
          })
          .unwrapOrElse((error) => {
            return (
              <ErrorComponent
                title="An error occurred while fetching the project's API keys"
                error={error}
                projectId={projectId}
              />
            );
          })}
      </div>
      <div className="mt-4 flex flex-row items-center text-center">
        <div>
          <h3
            className={`font-semibold text-xl md:text-3xl ${lusitana.className}`}
          >
            Devices
          </h3>
        </div>
      </div>
      <div>
        {projectDevicesResult
          .map((projectDevices) => {
            return (
              <ProjectDevices
                devices={projectDevices}
                key={projectId}
                projectId={projectId}
              />
            );
          })
          .unwrapOrElse((error) => {
            return (
              <ErrorComponent
                title="An error occurred while fetching the project's Registered Devices"
                error={error}
                projectId={projectId}
              />
            );
          })}
      </div>
    </div>
  );
}
