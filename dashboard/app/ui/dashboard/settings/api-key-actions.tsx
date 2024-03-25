'use client';

import { TrashIcon } from '@heroicons/react/24/outline';
import clsx from 'clsx';
import { Tooltip } from 'flowbite-react';
import { deleteApiKey } from '@/app/lib/actions';

const iconsMap = {
  delete: TrashIcon,
};

export default function ApiKeyActions({ apiKey }: { apiKey: string }) {
  return (
    <div className="flex items-center">
      <ApiKeyAction
        type="delete"
        onClick={async () => {
          await deleteApiKey(apiKey);
        }}
        className="cursor-pointer hover:text-red-700"
      />
    </div>
  );
}

export function ApiKeyAction({
  type,
  onClick,
  className,
}: {
  type: string;
  onClick: () => void;
  className: string;
}) {
  const Icon = iconsMap[type as keyof typeof iconsMap];

  return (
    <Tooltip content={type}>
      <Icon
        role="button"
        onClick={onClick}
        className={clsx(className, 'h-5 w-5')}
      ></Icon>
    </Tooltip>
  );
}
