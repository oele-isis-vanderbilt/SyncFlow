'use client';

import { CustomDataTable } from './data-table';
import { friendlyDateTimeFromNs } from '../utils';
import { Tooltip } from 'flowbite-react';
import { MdDelete } from 'react-icons/md';
import { SessionParticipant } from '@/types/project';

const participantsToColumns = (partcipants: SessionParticipant[]) => {
  return [
    {
      name: 'Identity',
      selector: (participant: SessionParticipant) => participant.identity,
      sortable: true,
    },
    {
      name: 'Metadata',
      selector: (participant: SessionParticipant) =>
        participant.metadata || 'No metadata',
      sortable: true,
    },
    {
      name: 'Joined',
      selector: (participant: SessionParticipant) =>
        `${friendlyDateTimeFromNs(participant.joinedAt)}`,
      sortable: true,
    },
    {
      name: 'Actions',
      cell: (participant: SessionParticipant) => {
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

export default function ParticipantsInfo({
  participants,
  roomName,
  noParticipantsMessage,
}: {
  participants: SessionParticipant[];
  roomName: string;
  noParticipantsMessage?: string;
}) {
  return (
    <CustomDataTable
      columns={participantsToColumns(participants)}
      data={participants}
      noDataComponent={
        noParticipantsMessage || `No participants in room ${roomName}`
      }
    />
  );
}
