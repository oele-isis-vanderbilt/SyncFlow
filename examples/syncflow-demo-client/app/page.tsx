import { jwtClient } from './lib/services';
import RoomJoinForm from './room-join-form';
import '@livekit/components-styles';

export default async function Home() {
  let rooms = await jwtClient.listRooms();
  let roomNames: string[] = rooms.map((room: any) => room.name as string);

  return (
    <main className="flex min-h-screen flex-col items-center p-24">
      <div className={'p-2 text-center'}>
        <h1 className={'p-2 text-2xl'}>SyncFlow Demo/Test App</h1>
        <p>
          This is a demo app for testing SyncFlow. It lists available rooms and
          allows you to join a room. After you join a room, you will be able to
          see share different video and audio devices and record them with the{' '}
          <code>SyncFlow</code> Dashboard.
        </p>
      </div>
      <RoomJoinForm roomNames={roomNames} />
    </main>
  );
}
