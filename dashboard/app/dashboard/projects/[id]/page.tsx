import { projectClient } from '@/app/lib/project-client';
import { lusitana } from '@/app/ui/fonts';

export default async function Project({ params }: { params: { id: string } }) {
  const id = params.id;

  return (await projectClient.getProject(id))
    .map((project) => {
      return (
        <div className="flex flex-row p-2 dark:text-white">
          <h2 className={`text-2xl font-bold ${lusitana.className}`}>
            {project.name}
          </h2>
        </div>
      );
    })
    .unwrapOrElse((error) => {
      return (
        <div className="flex flex-col p-2 dark:text-white">
          <span>An error occurred while fetching the project information</span>
          <br />
          <div className="mt-2 min-h-20 bg-gray-200 p-4 dark:bg-indigo-950">
            <pre>
              {JSON.stringify(
                {
                  projectId: id,
                  error: error,
                },
                null,
                2,
              )}
            </pre>
          </div>
        </div>
      );
    });
}
