'use client';

import type { ApiKeyResponse } from '@/types/api';
import { CustomDataTable } from './data-table';
import { getDateFromTimeStamp } from '../utils';
import { Tooltip } from 'flowbite-react';
import { MdDelete } from 'react-icons/md';
import InfoModal, { InfoModalContent } from '../info-modal';
import { useState } from 'react';
import { deleteApiKey } from '@/app/lib/project-actions';

const apiKeysToColumns = (projectId: string) => {
  return [
    {
      name: 'Api Key',
      sortable: true,
      selector: (apiKey) => apiKey.apiKey,
    },
    {
      name: 'Created At',
      sortable: true,
      selector: (apiKey) => getDateFromTimeStamp(apiKey.createdAt),
    },
    {
      name: 'Comment',
      selector: (apiKey) => apiKey.comment,
    },
    {
      name: 'Actions',
      cell: (apiKey) => {
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
                const result = await deleteApiKey(projectId, apiKey.id);
                if (!result.success) {
                  setInfoModalContent({
                    title: 'Error',
                    headings: [
                      {
                        title: 'Error deleting session',
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
              <Tooltip content="Delete API Key">
                <MdDelete className="cursor-pointer text-3xl text-red-600 hover:text-red-900" />
              </Tooltip>
            </button>
          </div>
        );
      },
    },
  ];
};

const apiKeysToData = (apiKeys: ApiKeyResponse[]) => {
  return apiKeys.map((apiKey) => {
    return {
      id: apiKey.id,
      apiKey: apiKey.key,
      createdAt: apiKey.createdAt,
      comment: apiKey.comment || 'N/A',
    };
  });
};

export default function ApiKeysTable({
  apiKeys,
  projectId,
}: {
  apiKeys: ApiKeyResponse[];
  projectId: string;
}) {
  const sortedKeys = apiKeys.sort((a, b) => b.createdAt - a.createdAt);
  const columns = apiKeysToColumns(projectId);
  const data = apiKeysToData(sortedKeys);
  return (
    <CustomDataTable
      columns={columns}
      data={data}
      noDataComponent={'No api keys found. Create one to get started'}
    />
  );
}
