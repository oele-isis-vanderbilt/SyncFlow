import { auth } from '@/auth';
import { isAdmin } from '@/app/lib/utils';
import { NoPermission } from '@/app/ui/no-permission';
import RecordingsSummary from '@/app/ui/dashboard/recordings/recordings-summary';
import RoomsTable from '@/app/ui/dashboard/rooms-table';

export default async function Page() {
  const session = await auth();

  if (!isAdmin(session?.user)) {
    return <NoPermission />;
  }

  return (
    <main className="p-2">
      <RoomsTable navPath={'recordings'} />
    </main>
  );
}
