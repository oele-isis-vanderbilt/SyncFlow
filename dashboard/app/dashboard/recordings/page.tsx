import { auth } from '@/auth';
import { NoPermission } from '@/app/ui/no-permission';
import RecordingsSummary from '@/app/ui/dashboard/recordings/recordings-summary';
import RoomsTable from '@/app/ui/dashboard/rooms-table';

export default async function Page() {
  const session = await auth();
  return (
    <main className="p-2">
      <RoomsTable navPath={'recordings'} />
    </main>
  );
}
