import { Tooltip } from 'flowbite-react';
import Link from 'next/link';
import { lusitana } from '@/app/ui/fonts';
import { CiSettings } from 'react-icons/ci';

export function ProjectHeader({
  projectName,
  projectId,
}: {
  projectName: string;
  projectId: string;
}) {
  return (
    <div className="flex flex-row items-center justify-between">
      <h2 className={`text-4xl font-bold ${lusitana.className}`}>
        {projectName}
      </h2>
      <Link href={`/dashboard/projects/${projectId}/settings`}>
        <Tooltip content="Project Settings">
          <CiSettings className="text-2xl hover:text-red-700" />
        </Tooltip>
      </Link>
    </div>
  );
}
