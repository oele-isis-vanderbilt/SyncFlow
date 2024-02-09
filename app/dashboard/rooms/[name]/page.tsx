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
  const isAdmin = session?.user?.role === Role.ADMIN;

  let token = await liveKitService.generateToken(userName, userName, {
    canPublish: true,
    canSubscribe: isAdmin,
    canUpdateOwnMetadata: true,
    roomJoin: true,
    roomCreate: isAdmin,
    room: roomName,
  });

  return (
    <main>
      <Room token={token} />
    </main>
  );
}
