'use client';

import type { EgressInfo } from 'livekit-server-sdk';
import { CustomDataTable } from './data-table';
import { getDateFromTimeStampNs } from '../utils';
import Link from 'next/link';
import { Tooltip } from 'flowbite-react';
import { TiCloudStorageOutline } from 'react-icons/ti';

const egressesToColumns = (egresses: EgressInfo[]) => {
  return [
    {
      name: 'Egress ID',
      selector: (egress) => egress.id,
      sortable: true,
    },
    {
      name: 'Started At',
      selector: (egress) => getDateFromTimeStampNs(egress.startedAt),
      sortable: true,
    },
    {
      name: 'Track ID',
      selector: (egress) => egress.track,
    },
    {
      name: 'Status',
      selector: (egress) => egress.status,
      sortable: false,
    },
    {
      name: 'Type',
      selector: (egress) => egress.type,
    },
    {
      name: 'Room Name',
      selector: (egress) => egress.roomName,
    },
    {
      name: 'Destination',
      cell: (egress) => {
        return (
          <div>
            {egress.status === 'EGRESS_COMPLETE' ? (
              <Tooltip content={egress.destination}>
                <Link href={egress.destination} target="_blank">
                  <TiCloudStorageOutline className="text-2xl hover:text-red-700" />
                </Link>
              </Tooltip>
            ) : (
              <span>{egress.destination}</span>
            )}
          </div>
        );
      },
    },
    {
      name: 'Actions',
      selector: (egress) => 'ToDo',
    },
  ];
};

const egressesToData = (egresses: EgressInfo[]) => {
  return egresses.map((egress) => {
    return {
      id: egress.egressId,
      startedAt: egress.startedAt,
      status: egress.status,
      track: egress.track.trackId,
      type: egress.egressType || 'Track',
      roomName: egress.roomName,
      destination: egress.file?.location || 'Pending',
    };
  });
};

export default function RecordingsInfo({
  egresses,
  emptyMessage,
}: {
  egresses: EgressInfo[];
  emptyMessage?: string;
}) {
  return (
    <CustomDataTable
      columns={egressesToColumns(egresses)}
      data={egressesToData(egresses)}
      noDataComponent={emptyMessage || 'No Recordings found'}
    />
  );
}
