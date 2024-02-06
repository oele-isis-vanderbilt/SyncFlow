import { lusitana } from '@/app/ui/fonts';
import type { Metadata } from 'next';
import { CardsSkeleton } from '@/app/ui/skeletons';
import { SummaryCards } from '@/app/ui/dashboard/summary-cards';
import { Button } from '@/app/ui/button';
import { PlusIcon } from '@heroicons/react/24/outline';
import CreateRoom from '@/app/ui/dashboard/create-room';
import RoomsTable from '@/app/ui/dashboard/rooms-table';
import { auth } from '@/auth';

import { Suspense } from 'react';
import { Role } from '@prisma/client';

export const metadata: Metadata = {
  title: 'Dashboard',
};

export default async function Page() {
  const session = await auth();

  return (
    <main>
      <h1 className={`${lusitana.className} mb-4 text-xl md:text-2xl`}>
        LiveKit ELP Dashboard
      </h1>
      <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
        <Suspense fallback={<CardsSkeleton />}>
          <SummaryCards />
        </Suspense>
      </div>
      <div className="mt-8 flex items-center">
        <div>
          <h1 className={`${lusitana.className} mb-4 mt-4 text-xl md:text-2xl`}>
            Rooms
          </h1>
        </div>
        {session?.user?.role === Role.ADMIN ? <CreateRoom /> : null}
      </div>
      <div className="mt-8 flex items-center">
        <RoomsTable />
      </div>
    </main>
  );
}
