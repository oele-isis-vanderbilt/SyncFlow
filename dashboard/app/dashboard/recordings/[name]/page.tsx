import { lusitana } from '@/app/ui/fonts';
import ActiveRecordings from '@/app/ui/dashboard/recordings/active-recordings';
import { mmlaClient } from '@/app/lib/mmla-client';
export default async function Page({ params }: { params: { name: string } }) {
  const roomName = params.name;

  const roomsResult = await mmlaClient.listRooms();
  let rooms = roomsResult.unwrap();
  const room = rooms.find((r) => r.name === roomName);
  if (!room) {
    return (
      <div className={'flex h-full items-center justify-center'}>
        <h2 className={`${lusitana.className} text-2xl dark:text-white`}>
          Room {roomName} not found
        </h2>
      </div>
    );
  }

  return (
    <main className="p-2">
      <ActiveRecordings roomName={roomName} />
    </main>
  );
}
