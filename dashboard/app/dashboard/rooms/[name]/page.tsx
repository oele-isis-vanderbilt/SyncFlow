import Room from '@/app/ui/dashboard/rooms/room';
import { liveKitService } from '@/app/lib/livekit';
import { auth } from '@/auth';
import { Role } from '@prisma/client';
import { isAdmin } from '@/app/lib/utils';

export default async function Page({ params }: { params: { name: string } }) {
  const roomName = params.name;
  const session = await auth();
  const userName = session?.user?.name || 'Anonymous';
  const isAdminUser = isAdmin(session?.user);

  let token = await liveKitService.generateToken(userName, userName, {
    canPublish: true,
    canSubscribe: isAdminUser,
    canUpdateOwnMetadata: true,
    roomJoin: true,
    roomCreate: isAdminUser,
    room: roomName,
  });

  return <Room token={token} user={session?.user} />;
}
