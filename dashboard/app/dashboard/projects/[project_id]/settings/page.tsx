import { projectClient } from '@/app/lib/project-client';
import ApiKeysTable from '@/app/ui/dashboard/project/api-keys';
import CreateApiKeys from '@/app/ui/dashboard/project/create-api-keys';
import ErrorComponent from '@/app/ui/dashboard/project/error-component';
import { lusitana } from '@/app/ui/fonts';

export default async function Page({
  params,
}: {
  params: { project_id: string };
}) {
  let projectId = params.project_id;

  let projectApiKeysResult = await projectClient.listApiKeys(projectId);

  return (
    <div className="flex flex-col p-2 dark:text-white">
      <h2 className={`text-4xl font-bold ${lusitana.className}`}>
        {' '}
        Project Settings{' '}
      </h2>
      <div className="mt-4 flex flex-row items-center text-center">
        <div>
          <h3 className={`text-3xl font-semibold ${lusitana.className}`}>
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
    </div>
  );
}
