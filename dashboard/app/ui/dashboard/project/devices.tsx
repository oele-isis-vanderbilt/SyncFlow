'use client';

import type { ProjectDevice } from '@/types/project';
import { getDateFromTimeStamp } from '../utils';
import { CustomDataTable } from './data-table';
import { Tooltip } from 'flowbite-react';
import { deleteDevice } from '@/app/lib/project-actions';
import { useState } from 'react';
import InfoModal, { InfoModalContent } from '../info-modal';
import { MdDelete } from 'react-icons/md';

const devicesToColumns = (projectId: string) => {
  return [
    {
      name: 'Device Id',
      selector: (device) => device.id,
      sortable: true,
    },
    {
      name: 'Device Name',
      selector: (device) => device.name,
      sortable: true,
    },
    {
      name: 'Group',
      selector: (device) => device.group,
      sortable: true,
    },
    {
      name: 'Registered At',
      selector: (device) => getDateFromTimeStamp(device.registeredAt),
      sortable: true,
    },
    {
      name: 'Comments',
      selector: (device) => device.comments || 'N/A',
    },
    {
      name: 'Actions',
      cell: (device) => {
        /* eslint-disable react-hooks/rules-of-hooks */
        let [infoModalContent, setInfoModalContent] =
          useState<InfoModalContent | null>(null);
        /* eslint-enable react-hooks/rules-of-hooks */

        return (
          <div className="flex flex-row gap-2">
            <InfoModal
              content={infoModalContent}
              onClose={() => setInfoModalContent(null)}
              show={infoModalContent !== null}
            />
            <button
              onClick={async () => {
                const result = await deleteDevice(projectId, device.id);
                if (!result.success) {
                  setInfoModalContent({
                    title: 'Error',
                    headings: [
                      {
                        title: 'Error deleting device',
                        items: [
                          {
                            title: 'Error',
                            content: result.error,
                          },
                        ],
                      },
                    ],
                  });
                } else {
                  setInfoModalContent(null);
                }
              }}
            >
              <Tooltip content="Delete Device">
                <MdDelete className="cursor-pointer text-3xl text-red-600 hover:text-red-900" />
              </Tooltip>
            </button>
          </div>
        );
      },
    },
  ];
};

const devicesToData = (devices: ProjectDevice[]) => {
  return devices.map((device) => {
    return {
      id: device.id,
      name: device.name,
      registeredAt: device.registeredAt,
      group: device.group,
      comments: device.comments,
    };
  });
};

export default function ProjectDevices({
  devices,
  projectId,
}: {
  devices: ProjectDevice[];
  projectId: string;
}) {
  const sortedDevices = devices.sort((a, b) => b.registeredAt - a.registeredAt);

  return (
    <CustomDataTable
      columns={devicesToColumns(projectId)}
      data={devicesToData(sortedDevices)}
      noDataComponent="No devices registered for this project"
    />
  );
}
