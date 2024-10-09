import { projectClient } from '@/app/lib/project-client';
import { ProjectCard } from '@/app/ui/dashboard/project-info';
import { lastUpdatedProjectComparator } from '../utils';

export default async function ProjectCards() {
  const projectsResult = await projectClient.listProjects();

  return projectsResult
    .map((projects) => {
      if (projects.length === 0) {
        return <div className="dark:text-white" key="projects">No projects found.</div>;
      }

      return (
        <div className="grid h-full w-full grid-cols-1 gap-5 p-2 md:grid-cols-4" key="projects">
          {projects.sort(lastUpdatedProjectComparator).map((project) => {
            return <ProjectCard key={project.id} project={project} />;
          })}
        </div>
      );
    })
    .unwrapOrElse((err) => {
      return <div>An error occurred.</div>;
    });
}
