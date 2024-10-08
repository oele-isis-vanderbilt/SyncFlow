import { mmlaClient } from '@/app/lib/mmla-client';
import {
  HomeModernIcon,
  UserGroupIcon,
  VideoCameraIcon,
} from '@heroicons/react/24/outline';
import { lusitana } from '@/app/ui/fonts';
import { HiChartPie } from 'react-icons/hi';
import { SiSession } from 'react-icons/si';
import { CgMediaLive } from 'react-icons/cg';

const iconMap = {
  rooms: HomeModernIcon,
  participants: UserGroupIcon,
  recordings: VideoCameraIcon,
  projects: HiChartPie,
  sessions: SiSession,
  liveSessions: CgMediaLive,
};

async function roomsSummary() {
  const roomsResult = await mmlaClient.listRooms();
  let rooms = roomsResult.unwrap();
  return rooms
    .filter((room) => !!room.name)
    .map((room) => {
      return {
        name: room.name,
        participants: room.numParticipants,
        recording: room.activeRecording,
      };
    });
}

export async function SummaryCards() {
  const rooms = await roomsSummary();
  const numParticipants = rooms.reduce(
    (acc, room) => acc + room.participants,
    0,
  );
  const numRecordings = rooms.reduce(
    (acc, room) => acc + (room.recording ? 1 : 0),
    0,
  );

  return (
    <>
      <Card title="Rooms" value={rooms.length} type="rooms" />
      <Card title="Participants" value={numParticipants} type="participants" />
      <Card title="Active Recordings" value={numRecordings} type="recordings" />
    </>
  );
}

export async function UserSummaryCards({
  numProjects,
  numSessions,
  numActiveSessions,
}: {
  numProjects: number;
  numSessions: number;
  numActiveSessions: number;
}) {
  return (
    <>
      <Card title="Projects" value={numProjects} type="projects" />
      <Card title="Sessions" value={numSessions} type="sessions" />
      <Card
        title="Active Sessions"
        value={numActiveSessions}
        type="liveSessions"
      />
    </>
  );
}

export function Card({
  title,
  value,
  type,
}: {
  title: string;
  value: string | number;
  type:
    | 'rooms'
    | 'participants'
    | 'recordings'
    | 'projects'
    | 'sessions'
    | 'liveSessions';
}) {
  const Icon = iconMap[type];

  return (
    <div className="rounded-xl bg-gray-200 p-2 shadow-sm dark:bg-gray-700 dark:text-white">
      <div className="flex p-4">
        {Icon ? <Icon className="h-5 w-5" /> : null}
        <h3 className="ml-2 text-sm font-medium">{title}</h3>
      </div>
      <p
        className={`${lusitana.className}
          truncate rounded-xl bg-gray-400 px-4 py-8 text-center text-2xl dark:bg-gray-900`}
      >
        {value}
      </p>
    </div>
  );
}
