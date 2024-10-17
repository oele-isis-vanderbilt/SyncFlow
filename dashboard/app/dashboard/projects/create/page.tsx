import CreateProjectForm from '@/app/ui/dashboard/create-project-form';
import { lusitana } from '@/app/ui/fonts';

export default async function CreateProject() {
  return (
    <div className="flex h-full w-full flex-col p-2">
      <h2
        className={`font-bold text-2xl dark:text-white ${lusitana.className}`}
      >
        Create a new project
      </h2>
      <div className="w-full flex-1 p-6 md:w-1/2">
        <CreateProjectForm />
      </div>
    </div>
  );
}
