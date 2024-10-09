import { projectClient } from '@/app/lib/project-client';
import type { ProjectSummary, ProjectSummary } from '@/types/project';

export async function ProjectSummaryComponent({
  projectId,
}: {
  projectId: string;
}) {
  return (await projectClient.summarizeProject(projectId))
    .map((summary: ProjectSummary) => {
      return (
        <div className="gap:20 flex w-full flex-row items-center justify-center md:gap-40" key={projectId}>
          <div className="grid grid-cols-1 gap-10 md:grid-cols-4 md:gap-40">
            <div className="flex flex-col items-center justify-center p-5 ring">
              <h2 className="text-2xl font-bold md:text-6xl">
                {summary.numSessions}
              </h2>
              <p className="text-sm md:text-2xl">Sessions</p>
            </div>
            <div className="flex flex-col items-center justify-center p-5 ring">
              <h2 className="text-2xl font-bold md:text-6xl">
                {summary.numActiveSessions}
              </h2>
              <p className="text-sm md:text-2xl">Active Sessions</p>
            </div>
            <div className="flex flex-col items-center justify-center p-5 ring">
              <h2 className="text-2xl font-bold md:text-6xl">
                {summary.numParticipants}
              </h2>
              <p className="text-sm md:text-2xl">Participants</p>
            </div>
            <div className="flex flex-col items-center justify-center p-5 ring">
              <h2 className="text-2xl font-bold md:text-6xl">
                {summary.numRecordings}
              </h2>
              <p className="text-sm md:text-2xl">Recordings</p>
            </div>
          </div>
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
                  projectId: projectId,
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
