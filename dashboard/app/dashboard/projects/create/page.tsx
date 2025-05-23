import CreateProjectForm from '@/app/ui/dashboard/create-project-form';
import { lusitana } from '@/app/ui/fonts';

export default async function CreateProject() {
  return (
    <div className="mb-24 flex flex-col p-2">
      <h2
        className={`font-bold text-2xl dark:text-white ${lusitana.className}`}
      >
        Create a new project
      </h2>
      <div className="w-full flex-1 flex-col items-center justify-center">
        <CreateProjectForm />
      </div>
    </div>
  );
}
