import ProjectCards from '@/app/ui/dashboard/paged-project-cards';
import { lusitana } from '@/app/ui/fonts';
import CreateProject from '@/app/ui/dashboard/create-project';

export default async function Projects() {
  return (
    <div className="flex flex-col p-2">
      <div className="flex flex-row items-center p-2 dark:text-white">
        <div>
          <h1
            className={`${lusitana.className} mt-4 mb-4 text-xl md:text-2xl dark:text-white`}
          >
            Projects
          </h1>
        </div>
        <CreateProject />
      </div>
      <div className="flex items-center p-2">
        <ProjectCards />
      </div>
    </div>
  );
}
