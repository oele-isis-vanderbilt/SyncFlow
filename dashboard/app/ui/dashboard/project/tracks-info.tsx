'use client';

import type { ParticipantInfo, TrackInfo } from 'livekit-server-sdk';
import { CustomDataTable } from './data-table';
import { Tooltip } from 'flowbite-react';
import { FaRecordVinyl } from 'react-icons/fa';
import { SessionParticipant, SessionTrack } from '@/types/project';

type DisplayTrack = SessionTrack & {
  publisher: string;
};

const tracksToColumns = () => {
  return [
    {
      name: 'Track SID/Name',
      selector: (track: SessionTrack) => track.sid,
      sortable: true,
      cell: (track: SessionTrack) => {
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
      selector: (track: DisplayTrack) => track.participantId || 'Unknown',
    },
    {
      name: 'Kind',
      selector: (track: DisplayTrack) => track.kind || 'Unknown',
    },
    {
      name: 'Source',
      selector: (track: DisplayTrack) => track.source || 'Unknown',
    },
    {
      name: 'Actions',
      cell: (track: DisplayTrack) => {
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

export function TracksInfo({
  participants,
  emptyTracksMessage,
}: {
  participants: SessionParticipant[];
  emptyTracksMessage?: string;
}) {
  const tracksData: DisplayTrack[] = participants.flatMap((participant) => {
    return participant.tracks.map((track) => {
      return {
        ...track,
        publisher: participant.identity,
      };
    });
  });

  return (
    <CustomDataTable
      columns={tracksToColumns()}
      data={tracksData}
      noDataComponent={emptyTracksMessage || 'No tracks available'}
    />
  );
}
