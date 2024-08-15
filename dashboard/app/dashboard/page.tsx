import { lusitana } from '@/app/ui/fonts';
import type { Metadata } from 'next';
import { CardsSkeleton } from '@/app/ui/skeletons';
import { SummaryCards } from '@/app/ui/dashboard/summary-cards';
import CreateRoom from '@/app/ui/dashboard/create-room';
import RoomsTable from '@/app/ui/dashboard/rooms-table';
import { auth } from '@/auth';

import { Suspense } from 'react';
import { isAdmin } from '@/app/lib/utils';

export const metadata: Metadata = {
  title: 'Dashboard',
};

export default async function Page() {
  const session = await auth();

  return (
    <main className="dark:bg-gray-800">
      <h1
        className={`${lusitana.className} mb-4 p-2 text-xl md:text-2xl dark:text-white`}
      >
        SyncFlow Dashboard
      </h1>
      <div className="grid gap-6 p-2 sm:grid-cols-2 lg:grid-cols-4">
        <Suspense fallback={<CardsSkeleton />}>
          <SummaryCards />
        </Suspense>
      </div>
      <div className="mt-8 flex items-center p-2">
        <div>
          <h1
            className={`${lusitana.className} mb-4 mt-4 text-xl md:text-2xl dark:text-white`}
          >
            Rooms
          </h1>
        </div>
        {isAdmin(session?.user) ? <CreateRoom /> : null}
      </div>
      <div className="mt-8 flex items-center p-2">
        <RoomsTable navPath={'rooms'} />
      </div>
    </main>
  );
}
