'use client';
import { deleteRoom } from '@/app/lib/actions';
import clsx from 'clsx';

import { TrashIcon, CameraIcon } from '@heroicons/react/24/outline';

const iconsMap = {
  delete: TrashIcon,
  camera: CameraIcon,
};

export default function RoomActions({ room }) {
  return (
    <div className="flex items-center">
      <RoomAction
        type="delete"
        onClick={async () => {
          await deleteRoom(room.name);
        }}
        className="cursor-pointer hover:text-red-700"
        help={'Delete room'}
      />
    </div>
  );
}

export function RoomAction({ type, onClick, help, className }) {
  const Icon = iconsMap[type];
  return (
    <>
      <Icon
        role="button"
        onClick={onClick}
        className={clsx(className, 'h-5 w-5')}
      ></Icon>
    </>
  );
}
