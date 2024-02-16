'use server';
import { getRoomRecordings } from '@/app/lib/actions';

export default async function RoomRecordingsList({
  roomName,
}: {
  roomName: string;
}) {
  const recordings = await getRoomRecordings(roomName);
  return recordings.map((recording) => {
    return <div>{JSON.stringify(recording, null, 2)}</div>;
  });
}
