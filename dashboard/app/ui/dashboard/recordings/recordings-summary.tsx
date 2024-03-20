import { mmlaClient } from '@/app/lib/mmla-client';

export default async function RecordingsSummary() {
  const roomsResult = await mmlaClient.listRooms();
  let rooms = roomsResult.unwrap();

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
