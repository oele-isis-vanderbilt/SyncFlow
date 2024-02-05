import type { Room } from 'livekit-server-sdk';
import { liveKitService } from '@/app/lib/livekit';
import RoomActions from '@/app/ui/dashboard/room-actions';
import Link from 'next/link';

export default async function RoomsTable() {
  const rooms = await liveKitService.listRooms();
  const names = rooms.map((room: Room) => room.name);
  return (
    <>
      {names.length === 0 ? (
        <p>No active rooms. Create one to get started.</p>
      ) : (
        <table className="w-full text-left text-sm rtl:text-right">
          <thead className="text-gray bg-gray-900 text-xs uppercase ">
            <tr>
              <th scope="col" className="px-6 py-3">
                ID
              </th>
              <th scope="col" className="px-6 py-3">
                Name
              </th>

              <th scope="col" className="px-6 py-3">
                Created At
              </th>
              <th scope="col" className="px-6 py-3">
                Participants
              </th>
              <th scope="col" className="px-6 py-3">
                Active Recording
              </th>
              <th scope="col" className="px-6 py-3">
                Actions
              </th>
              <th scope="col" className="px-6 py-3">
                Comments
              </th>
            </tr>
          </thead>
          <tbody>
            {rooms.map((room, index: number) => {
              return (
                <tr key={index} className="border-5 border-indigo-200 bg-black">
                  <td className="whitespace-nowrap px-6 py-4">
                    <Link href={`/dashboard/rooms/${room.name}`}>
                      <div className="text-blue text-sm hover:text-blue-400 hover:underline">
                        {room.sid.trim()}
                      </div>
                    </Link>
                  </td>
                  <td className="whitespace-nowrap px-6 py-4">
                    <Link href={`/dashboard/rooms/${room.name}`}>
                      <div className="text-blue text-sm hover:text-blue-400 hover:underline">
                        {room.name.trim()}
                      </div>
                    </Link>
                  </td>
                  <td className="whitespace-nowrap px-6 py-4">
                    <div className="text-sm">
                      {new Date(room.creationTime * 1000).toISOString()}
                    </div>
                  </td>
                  <td className="whitespace-nowrap px-6 py-4">
                    <div className="text-sm">{room.numParticipants}</div>
                  </td>
                  <td className="whitespace-nowrap px-6 py-4">
                    <div className="text-sm">
                      {room.activeRecording ? 'Yes' : 'No'}
                    </div>
                  </td>

                  <td className="whitespace-nowrap px-6 py-4">
                    <RoomActions room={room} />
                  </td>
                  <td className="whitespace-nowrap px-6 py-4">
                    <div className="text-sm">{room.metadata || 'N/A'}</div>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      )}
    </>
  );
}
