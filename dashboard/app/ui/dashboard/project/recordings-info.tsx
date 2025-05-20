'use client';

import type { SessionEgress } from '@/types/project';

import { CustomDataTable } from './data-table';
import { getDateFromTimeStampNs } from '../utils';
import { Tooltip } from 'flowbite-react';
import { TiCloudStorageOutline } from 'react-icons/ti';
import { getEgressMediaDownloadUrl } from '@/app/lib/project-actions';

const getFileName = (path: string) => {
  const parts = path.split('/');
  return parts[parts.length - 1];
};

const egressesToColumns = (
  egresses: SessionEgress[],
  projectId: string,
  sessionId: string,
) => {
  return [
    {
      name: 'Egress Id',
      selector: (egress: SessionEgress) => egress.egressId,
    },
    {
      name: 'Started At',
      selector: (egress: SessionEgress) =>
        getDateFromTimeStampNs(egress.startedAt),
      sortable: true,
    },
    {
      name: 'Track ID',
      selector: (egress: SessionEgress) => egress.trackId,
    },
    {
      name: 'Status',
      selector: (egress: SessionEgress) => egress.status,
      sortable: false,
    },
    {
      name: 'Type',
      selector: (egresses: SessionEgress) => egresses.egressType,
    },
    {
      name: 'Room Name',
      selector: (egress: SessionEgress) => egress.roomName,
    },
    {
      name: 'File Name',
      selector: (egress: SessionEgress) => getFileName(egress.destination),
      cell: (egress: SessionEgress) => {
        return (
          <span>
            {egress.destination ? getFileName(egress.destination) : 'N/A'}
          </span>
        );
      },
    },
    {
      name: 'Destination',
      cell: (egress: SessionEgress) => {
        return (
          <div>
            {egress.status === 'EGRESS_COMPLETE' ? (
              <>
                <Tooltip content="Download File">
                  <button
                    onClick={async () => {
                      let result = await getEgressMediaDownloadUrl(
                        projectId,
                        sessionId,
                        egress.destination,
                      );
                      if (result.success) {
                        window.open(result.data.mediaUrl, '_blank');
                      } else {
                        console.error(result.error);
                      }
                    }}
                  >
                    <TiCloudStorageOutline className="text-2xl hover:text-red-700" />
                  </button>
                </Tooltip>
              </>
            ) : (
              <span>N/A</span>
            )}
          </div>
        );
      },
    },
  ];
};

const egressToData = (egresses: SessionEgress[]) => {
  return egresses.map((egress) => {
    return {
      ...egress,
    };
  });
};

export const RecordingsInfo = ({
  egresses,
  projectId,
  sessionId,
}: { egresses: SessionEgress[]; projectId: string; sessionId: string }) => {
  return (
    <CustomDataTable
      columns={egressesToColumns(egresses, projectId, sessionId)}
      data={egressToData(egresses)}
      noDataComponent="No recordings found"
    />
  );
};
