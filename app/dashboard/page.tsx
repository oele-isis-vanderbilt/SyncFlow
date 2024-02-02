import { lusitana } from '@/app/ui/fonts';
import type { Metadata } from 'next';
import UserActions from '@/app/ui/dashboard/user-actions';
import { liveKitService } from '@/app/lib/livekit';
import CreateRoom from '@/app/ui/dashboard/create-room';

export const metadata: Metadata = {
  title: 'Dashboard',
};

export default async function Page() {
  const rooms = (await liveKitService.listRooms()).sort(
    (r1, r2) => r1.creationTime - r2.creationTime,
  );
  const isRoomActive = rooms.length > 0;
  const roomName = isRoomActive ? rooms[0].name : '';
  if (isRoomActive) {
  }

  return (
    <main>
      <h1 className={`${lusitana.className} mb-4 text-xl md:text-2xl`}>
        LiveKit ELP Dashboard
      </h1>
      {isRoomActive ? <UserActions roomName={roomName} /> : <CreateRoom />}
    </main>
  );
}
