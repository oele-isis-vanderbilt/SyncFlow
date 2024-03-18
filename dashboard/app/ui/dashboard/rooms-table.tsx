import type { Room } from 'livekit-server-sdk';
import { mmlaClient } from '@/app/lib/mmlaClient';
import RoomActions from '@/app/ui/dashboard/room-actions';
import Link from 'next/link';
import { auth } from '@/auth';
import { isAdmin } from '@/app/lib/utils';

export default async function RoomsTable({ navPath }: { navPath: string }) {
  const roomsResult = await mmlaClient.listRooms();
  let names: string[] = [];
  roomsResult
    .map((rooms) => {
      names = rooms.map((room) => room.name).flat();
    })
    .mapError((err) => (names = []));
  const session = await auth();
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
            {roomsResult.unwrap().map((room, index: number) => {
              return (
                <tr key={index} className="border-5 border-indigo-200 bg-black">
                  <td className="whitespace-nowrap px-6 py-4">
                    <Link href={`/dashboard/${navPath}/${room.name}`}>
                      <div className="text-blue text-sm hover:text-blue-400 hover:underline">
                        {room.sid.trim()}
                      </div>
                    </Link>
                  </td>
                  <td className="whitespace-nowrap px-6 py-4">
                    <Link href={`/dashboard/${navPath}/${room.name}`}>
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
                    <RoomActions room={room} isAdmin={isAdmin(session?.user)} />
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
