'use client';
import dynamic from 'next/dynamic';
import { useState } from 'react';
import { LocalUserChoices } from '@livekit/components-core';
import { useRouter } from 'next/navigation';

const PreJoinNoSSR = dynamic(
  async () => {
    return (await import('@livekit/components-react')).PreJoin;
  },
  { ssr: false },
);
export default function RoomJoinForm({ roomNames }: { roomNames: string[] }) {
  if (roomNames.length === 0) {
    return (
      <div style={{ display: 'grid', placeItems: 'center', height: '100%' }}>
        <p>No rooms available</p>
      </div>
    );
  }

  const router = useRouter();

  let [activeRoomIndex, setActiveRoomIndex] = useState<number>(0);

  function handlePreJoinSubmit(values: LocalUserChoices) {
    localStorage.setItem('preJoinChoices', JSON.stringify(values));
    router.push(`/room?name=${roomNames[activeRoomIndex]}`);
  }

  return (
    <div style={{ display: 'grid', placeItems: 'center', height: '100%' }}>
      <div className="mb-4 text-lg">Select a room</div>
      <select className="mb-4 block h-8 w-48 rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50">
        {roomNames.map((roomName, index) => {
          return (
            <option key={index} id={`room-${index}`} role="option">
              {roomName}
            </option>
          );
        })}
      </select>

      <PreJoinNoSSR
        onError={(err) => console.log('error while setting up prejoin', err)}
        defaults={{
          username: '',
          videoEnabled: true,
          audioEnabled: true,
        }}
        onSubmit={handlePreJoinSubmit}
      ></PreJoinNoSSR>
    </div>
  );
}
