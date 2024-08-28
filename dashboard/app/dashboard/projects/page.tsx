import ProjectCards from '@/app/ui/dashboard/paged-project-cards';
import { lusitana } from '@/app/ui/fonts';

export default async function Projects() {
  return (
    <div className="flex flex-col p-2">
      <h2 className={`${lusitana.className} text-2xl dark:text-white`}>
        Projects
      </h2>
      <div className="flex items-center p-2">
        <ProjectCards />
      </div>
    </div>
  );
}
