import { Suspense } from 'react';
import { lusitana } from '@/app/ui/fonts';
import type { Metadata } from 'next';
import UserActions from '@/app/ui/dashboard/user-actions';

export const metadata: Metadata = {
  title: 'Dashboard',
};

export default function Page({ children }: { children: React.ReactNode }) {
  return (
    <main>
      <h1 className={`${lusitana.className} mb-4 text-xl md:text-2xl`}>
        LiveKit ELP Dashboard
      </h1>
      <UserActions />
    </main>
  );
}
