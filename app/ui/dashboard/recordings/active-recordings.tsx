import { liveKitService } from '@/app/lib/livekit';
import type { TrackInfo } from 'livekit-server-sdk';
import { TrackSource } from 'livekit-server-sdk/dist/proto/livekit_models';
import { getHelpText } from '@/app/ui/dashboard/rooms/utils';
import { lusitana } from '@/app/ui/fonts';
import { auth } from '@/auth';
import { Role } from '@prisma/client';
import { NoPermission } from '@/app/ui/no-permission';

export default async function ActiveRecordings({
  roomName,
}: {
  roomName: string;
}) {
  const recordings = await liveKitService.listParticipants(roomName);
  const egresses = await liveKitService.getRoomEgresses(roomName);

  const getTrackKind = (track: TrackInfo) => {
    switch (track.source) {
      case TrackSource.MICROPHONE:
        return 'microphone audio';
      case TrackSource.CAMERA:
        return 'camera video';
      case TrackSource.SCREEN_SHARE:
        return 'screen capture';
      case TrackSource.SCREEN_SHARE_AUDIO:
        return 'screen capture audio';
      case TrackSource.UNKNOWN:
        return 'unknown';
      default:
        return 'unknown';
    }
  };

  const tracks = recordings
    .map((recording) => {
      return recording.tracks.map((track) => {
        return {
          participant: recording.identity,
          trackId: track.sid,
          kind: getTrackKind(track),
        };
      });
    })
    .flat();

  return (
    <div className={'flex h-full w-full flex-col'}>
      <h1 className={`${lusitana.className} mb-4 text-xl md:text-2xl`}>
        Recordings for room {roomName}
      </h1>
      {tracks.length === 0 ? (
        <span>No Active Participants in the room</span>
      ) : (
        <TracksTable tracks={tracks} />
      )}
    </div>
  );
}

function TracksTable({
  tracks,
}: {
  tracks: { participant: string; trackId: string; kind: string }[];
}) {
  return (
    <table className="w-full overflow-scroll text-left text-sm rtl:text-right">
      <thead className="text-gray bg-gray-900 text-xs uppercase ">
        <tr>
          <th scope="col" className="px-6 py-3">
            Participant
          </th>
          <th scope="col" className="px-6 py-3">
            Track ID
          </th>
          <th scope="col" className="px-6 py-3">
            Kind
          </th>
        </tr>
      </thead>
      <tbody>
        {tracks.map((track, index: number) => {
          return (
            <tr key={index} className="border-5 border-indigo-200 bg-black">
              <td className="whitespace-nowrap px-6 py-4">
                {track.participant}
              </td>
              <td className="whitespace-nowrap px-6 py-4">{track.trackId}</td>
              <td className="whitespace-nowrap px-6 py-4">{track.kind}</td>
            </tr>
          );
        })}
      </tbody>
    </table>
  );
}
