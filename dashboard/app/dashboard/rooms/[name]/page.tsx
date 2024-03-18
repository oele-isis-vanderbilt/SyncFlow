import Room from '@/app/ui/dashboard/rooms/room';
import { auth } from '@/auth';
import { isAdmin } from '@/app/lib/utils';
import { mmlaClient } from '@/app/lib/mmlaClient';
import { TrackSource } from 'livekit-server-sdk';

export default async function Page({ params }: { params: { name: string } }) {
  const roomName = params.name;
  const session = await auth();
  const userName = session?.user?.name || 'Anonymous';
  const isAdminUser = isAdmin(session?.user);

  let { token } = (
    await mmlaClient.generateToken({
      identity: userName,
      videoGrants: {
        room: roomName,
        canPublish: true,
        canSubscribe: isAdminUser,
        canPublishSources: [
          TrackSource.MICROPHONE,
          TrackSource.CAMERA,
          TrackSource.SCREEN_SHARE,
          TrackSource.SCREEN_SHARE_AUDIO,
        ],
        canPublishData: true,
        canUpdateOwnMetadata: isAdminUser,
        hidden: false,
        ingressAdmin: isAdminUser,
        recorder: isAdminUser,
        roomAdmin: isAdminUser,
        roomCreate: isAdminUser,
        roomJoin: true,
        roomList: true,
        roomRecord: isAdminUser,
      },
    })
  ).unwrap();

  return <Room token={token} user={session?.user} />;
}
