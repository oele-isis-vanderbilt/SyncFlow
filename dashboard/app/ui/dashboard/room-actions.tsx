'use client';
import {
  deleteRoom,
  redirectToDashboard,
  redirectToRoomRecording,
} from '@/app/lib/actions';
import clsx from 'clsx';

import { TrashIcon, CameraIcon } from '@heroicons/react/24/outline';
import type { Room } from 'livekit-server-sdk';
import { BsRecordBtn } from 'react-icons/bs';
import { Tooltip } from 'flowbite-react';

const iconsMap = {
  delete: TrashIcon,
  camera: CameraIcon,
  record: BsRecordBtn,
};

export default function RoomActions({ room }: { room: Room }) {
  return (
    <div className="flex items-center">
      <RoomAction
        type="delete"
        onClick={async () => {
          await deleteRoom(room.name);
        }}
        className="cursor-pointer hover:text-red-700"
      />
      <RoomAction
        type={'record'}
        onClick={async () => {
          await redirectToRoomRecording(room.name);
        }}
        className={'ml-2 cursor-pointer hover:text-red-700'}
      />
    </div>
  );
}

export function RoomAction({
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
