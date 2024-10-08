'use client';

import { ParticipantInfo } from 'livekit-server-sdk';
import { CustomDataTable } from './data-table';
import { getTimeDifferenceInMinutes } from '../utils';
import { Tooltip } from 'flowbite-react';
import { MdDelete } from 'react-icons/md';
import { ParticipantInfo_Kind } from '@livekit/protocol';

const participantsToColumns = (partcipants: ParticipantInfo[]) => {
  return [
    {
      name: 'Identity',
      selector: (participant) => participant.id,
      sortable: true,
    },
    {
      name: 'Metadata',
      selector: (participant) => participant.metadata || 'No metadata',
      sortable: true,
    },
    {
      name: 'Joined',
      selector: (participant) =>
        `${getTimeDifferenceInMinutes(participant.joinedAt)} Minutes ago`,
      sortable: true,
    },
    {
      name: 'Actions',
      cell: (participant) => {
        return (
          <button>
            <Tooltip content="Remove Participant">
              <MdDelete className="h-5 w-5 cursor-pointer text-red-700" />
            </Tooltip>
          </button>
        );
      },
    },
  ];
};

const participantsToData = (participants: ParticipantInfo[]) => {
  return participants
    .filter((p) => {
      return !['EGRESS', 'INGRESS', 'AGENT'].includes(p.kind);
    })
    .map((participant) => {
      return {
        id: participant.identity,
        metadata: participant.metadata,
        joinedAt: participant.joinedAt,
      };
    });
};

export default function ParticipantsInfo({
  participants,
  roomName,
  noParticipantsMessage,
}: {
  participants: ParticipantInfo[];
  roomName: string;
  noParticipantsMessage?: string;
}) {
  return (
    <CustomDataTable
      columns={participantsToColumns(participants)}
      data={participantsToData(participants)}
      noDataComponent={
        noParticipantsMessage || `No participants in room ${roomName}`
      }
    />
  );
}
