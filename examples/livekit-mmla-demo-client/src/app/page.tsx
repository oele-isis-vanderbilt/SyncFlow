import { jwtClient } from '@/app/lib/services';
import RoomJoinForm from '@/app/room-join-form';
import '@livekit/components-styles';
export default async function Home() {
  let rooms = await jwtClient.listRooms();
  let roomNames: string[] = rooms.map((room: any) => room.name as string);

  return (
    <main
      className="flex min-h-screen flex-col items-center p-24"
      data-lk-theme="default"
    >
      <div className="z-10 w-full max-w-5xl items-center justify-between font-mono text-sm lg:flex">
        <p className="fixed left-0 top-0 flex w-full justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl lg:static lg:w-auto lg:rounded-xl lg:border lg:bg-gray-200  lg:p-4 dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:dark:bg-zinc-800/30">
          <code className="font-mono font-bold">LivekitMMLA Client</code>
        </p>
      </div>
      <RoomJoinForm roomNames={roomNames} />
    </main>
  );
}
