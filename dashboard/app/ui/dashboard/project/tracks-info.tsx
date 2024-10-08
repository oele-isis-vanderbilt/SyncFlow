'use client';

import type { ParticipantInfo, TrackInfo } from 'livekit-server-sdk';
import { CustomDataTable } from './data-table';
import { Tooltip } from 'flowbite-react';
import { FaRecordVinyl } from 'react-icons/fa';

const tracksToColumns = (
  tracks: TrackInfo[],
  tracksToParticipantSid: Record<string, string>,
) => {
  return [
    {
      name: 'Track SID/Name',
      selector: (track) => track.sid,
      sortable: true,
      cell: (track) => {
        return (
          <div className="flex items-center">
            <div className="mr-2">{track.sid}</div>
            <div className="text-sm">
              {'( '} {track.name} {' )'}
            </div>
          </div>
        );
      },
    },
    {
      name: 'Publisher',
      sortable: true,
      selector: (track) => tracksToParticipantSid[track.sid] || 'Unknown',
    },
    {
      name: 'Kind',
      selector: (track) => track.type || 'Unknown',
    },
    {
      name: 'Actions',
      cell: (track) => {
        return (
          <div className="flex items-center">
            <button>
              <Tooltip content="Record Track">
                <FaRecordVinyl className="text-2xl hover:text-red-700" />
              </Tooltip>
            </button>
          </div>
        );
      },
    },
  ];
};

const tracksToTableData = (
  tracks: TrackInfo[],
  tracksToParticipantSid: Record<string, string>,
) => {
  return tracks.map((track) => {
    return {
      ...track,
      id: track.sid,
      publisher: tracksToParticipantSid[track.sid] || 'Unknown',
    };
  });
};

export function TracksInfo({
  participants,
  emptyTracksMessage,
}: {
  participants: ParticipantInfo[];
  emptyTracksMessage?: string;
}) {
  const tracks = participants.reduce<TrackInfo[]>((acc, participant) => {
    return acc.concat(participant.tracks || []);
  }, [] as TrackInfo[]);

  const tracksToParticipantSid = participants.reduce(
    (acc, participant) => {
      participant.tracks?.forEach((track) => {
        acc[track.sid] = participant.identity;
      });
      return acc;
    },
    {} as Record<string, string>,
  );

  return (
    <CustomDataTable
      columns={tracksToColumns(tracks, tracksToParticipantSid)}
      data={tracksToTableData(tracks, tracksToParticipantSid)}
      noDataComponent={emptyTracksMessage || 'No tracks available'}
    />
  );
}
