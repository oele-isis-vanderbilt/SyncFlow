import { liveKitService } from '@/app/lib/livekit';
import { lusitana } from '@/app/ui/fonts';
import NoActiveRoom from '@/app/ui/dashboard/streams/no-active-room';
import Room from '@/app/ui/dashboard/streams/room';

export default async function Page() {
  const rooms = (await liveKitService.listRooms()).sort(
    (r1, r2) => r1.creationTime - r2.creationTime,
  );
  const isRoomActive = rooms.length > 0;

  const token = await liveKitService.generateToken('test', 'test', {
    canSubscribe: true,
    canPublish: true,
    room: rooms[0]?.name || '',
  });

  return (
    <main>
      <h1 className={`${lusitana.className} mb-4 text-xl md:text-2xl`}>
        Streams
      </h1>
      {isRoomActive ? (
        <Room user={'test'} token={token} room={rooms[0]} />
      ) : (
        <NoActiveRoom />
      )}
    </main>
  );
}
