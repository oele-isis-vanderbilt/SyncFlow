import { lusitana } from '@/app/ui/fonts';
import Room from '@/app/ui/dashboard/rooms/room';
import { liveKitService } from '@/app/lib/livekit';
import { useMemo } from 'react';

export default async function Page({ params }: { params: { name: string } }) {
  const roomName = params.name;
  let userName = 'admin';

  const token = await useMemo(async () => {
    return await liveKitService.generateToken(userName, userName, {
      canPublish: true,
      canSubscribe: true,
      canUpdateOwnMetadata: true,
      roomJoin: true,
      roomCreate: false,
      room: roomName,
    });
  }, [userName]);

  return (
    <main>
      <h1 className={`${lusitana.className} mb-4 text-xl md:text-2xl`}>
        Room {roomName}
      </h1>
      <Room token={token} name={userName} room={roomName} />
    </main>
  );
}
