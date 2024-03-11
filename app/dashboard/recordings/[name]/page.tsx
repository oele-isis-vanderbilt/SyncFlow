import { liveKitService } from '@/app/lib/livekit';
import { lusitana } from '@/app/ui/fonts';
import ActiveRecordings from '@/app/ui/dashboard/recordings/active-recordings';
export default async function Page({ params }: { params: { name: string } }) {
  const roomName = params.name;

  const rooms = await liveKitService.listRooms();
  const room = rooms.find((r) => r.name === roomName);
  if (!room) {
    return (
      <div className={'flex h-full items-center justify-center'}>
        <h2 className={`${lusitana.className} text-2xl`}>
          Room {roomName} not found
        </h2>
      </div>
    );
  }

  return <ActiveRecordings roomName={roomName} />;
}
