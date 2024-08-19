import Room from '@/app/ui/dashboard/rooms/room';
import { auth } from '@/auth';
import { mmlaClient } from '@/app/lib/mmla-client';

export default async function Page({ params }: { params: { name: string } }) {
  const roomName = params.name;
  const session = await auth();
  const userName = session?.user?.name || 'Anonymous';

  let { token } = (
    await mmlaClient.generateToken({
      identity: userName,
      videoGrants: {
        room: roomName,
        canPublish: true,
        canSubscribe: true,
        canPublishSources: [],
        canPublishData: true,
        canUpdateOwnMetadata: true,
        hidden: false,
        ingressAdmin: true,
        recorder: true,
        roomAdmin: true,
        roomCreate: true,
        roomJoin: true,
        roomList: true,
        roomRecord: true,
      },
    })
  ).unwrap();

  return <Room token={token} user={session?.user} />;
}
