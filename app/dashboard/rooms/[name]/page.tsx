import { lusitana } from '@/app/ui/fonts';
import Room from '@/app/ui/dashboard/rooms/room';
import { liveKitService } from '@/app/lib/livekit';
import { useMemo } from 'react';
import { auth } from '@/auth';
import { Role } from '@prisma/client';

export default async function Page({ params }: { params: { name: string } }) {
  const roomName = params.name;
  const session = await auth();
  const userName = session?.user?.name || 'Anonymous';

  let token = await liveKitService.generateToken(userName, userName, {
    canPublish: true,
    canSubscribe: session?.user?.role === Role.ADMIN,
    canUpdateOwnMetadata: true,
    roomJoin: true,
    roomCreate: session?.user?.role === Role.ADMIN,
    room: roomName,
  });

  return (
    <main>
      <h1 className={`${lusitana.className} mb-4 text-xl md:text-2xl`}>
        Room {roomName}
      </h1>
      <Room token={token} name={userName} room={roomName} />
    </main>
  );
}
