import { liveKitService } from '@/app/lib/livekit';

export default async function RecordingsSummary() {
  const rooms = await liveKitService.listRooms();

  return (
    <div className={'flex h-full w-full flex-col'}>
      {rooms.map((room) => {
        return (
          <div key={room.name}>
            <h1>{room.name}</h1>
          </div>
        );
      })}
    </div>
  );
}
