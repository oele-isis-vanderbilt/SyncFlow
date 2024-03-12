import { liveKitService } from '@/app/lib/livekit';
import type { TrackInfo } from 'livekit-server-sdk';
import { TrackSource } from 'livekit-server-sdk/dist/proto/livekit_models';
import { lusitana } from '@/app/ui/fonts';
import {
  AllTracksRecordButton,
  TrackRecordButton,
} from '@/app/ui/dashboard/recordings/record-buttons';
import { EgressStatus } from 'livekit-server-sdk/dist/proto/livekit_egress';

export default async function ActiveRecordings({
  roomName,
}: {
  roomName: string;
}) {
  const participantInfos = await liveKitService.listParticipants(roomName);
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

  const tracks = participantInfos
    .map((participantInfo) => {
      return participantInfo.tracks.map((track) => {
        return {
          participant: participantInfo.identity,
          trackId: track.sid,
          kind: getTrackKind(track),
          roomName: roomName,
          egressId:
            egresses.find(
              (e) =>
                e.track?.trackId === track.sid &&
                [
                  EgressStatus.EGRESS_ACTIVE,
                  EgressStatus.EGRESS_STARTING,
                ].includes(e.status!),
            )?.egressId || undefined,
        };
      });
    })
    .flat();
  return (
    <div className={'flex h-full w-full flex-col'}>
      <h1 className={`${lusitana.className} mb-4 text-xl md:text-2xl`}>
        Manage Recordings for room {roomName}
      </h1>
      {tracks.length === 0 ? (
        <span>No Active Participants in the room</span>
      ) : (
        <div className={'flex flex-row gap-2'}>
          <div className={'flex-1'}>
            <TracksTable tracks={tracks} />
          </div>
        </div>
      )}
    </div>
  );
}

function TracksTable({
  tracks,
}: {
  tracks: {
    participant: string;
    trackId: string;
    kind: string;
    roomName: string;
    egressId: string | undefined;
  }[];
}) {
  return (
    <div className={'flex h-full w-full flex-col'}>
      <div className={'flex flex-row items-center justify-between p-2'}>
        <h2 className={`${lusitana.className} p-2 text-2xl text-white`}>
          Available Tracks
        </h2>
        <AllTracksRecordButton tracks={tracks} />
      </div>
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
            <th scope="col" className="px-6 py-3">
              Actions
            </th>
          </tr>
        </thead>
        <tbody>
          {tracks.map((track, index: number) => {
            console.log(track, index);
            return (
              <tr key={index} className="border-5 border-indigo-200 bg-black">
                <td className="whitespace-nowrap px-6 py-4">
                  {track.participant}
                </td>
                <td className="whitespace-nowrap px-6 py-4">{track.trackId}</td>
                <td className="whitespace-nowrap px-6 py-4">{track.kind}</td>
                <td className="whitespace-nowrap px-6 py-4">
                  <TrackRecordButton trackInfo={track} />
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}