import { lusitana } from '@/app/ui/fonts';
import type { Metadata } from 'next';
import { CardsSkeleton } from '@/app/ui/skeletons';
import { UserSummaryCards } from '@/app/ui/dashboard/summary-cards';
import CreateProject from '@/app/ui/dashboard/create-project';
import ProjectCards from '../ui/dashboard/paged-project-cards';
import { auth } from '@/auth';

import { Suspense } from 'react';
import { projectClient } from '../lib/project-client';

export const metadata: Metadata = {
  title: 'Dashboard',
};

export default async function Page() {
  const session = await auth();
  const summary = (await projectClient.summarizeUserProjects()).unwrapOr({
    numProjects: 0,
    numSessions: 0,
    numActiveSessions: 0,
  });

  return (
    <main className="dark:bg-gray-800">
      <h1
        className={`${lusitana.className} mb-4 p-2 text-xl md:text-2xl dark:text-white`}
      >
        Welcome to SyncFlow, {session?.user?.name ? session.user.name : 'User'}!
      </h1>
      <div className="grid gap-6 p-2 sm:grid-cols-2 lg:grid-cols-4">
        <Suspense fallback={<CardsSkeleton />}>
          <UserSummaryCards
            numProjects={summary.numProjects}
            numActiveSessions={summary.numActiveSessions}
            numSessions={summary.numSessions}
          />
        </Suspense>
      </div>
      <div className="mt-8 flex items-center p-2">
        <div>
          <h1
            className={`${lusitana.className} mt-4 mb-4 text-xl md:text-2xl dark:text-white`}
          >
            Projects
          </h1>
        </div>
        <CreateProject />
      </div>
      <div className="mt-8 flex items-center p-2">
        <ProjectCards />
      </div>
    </main>
  );
}
