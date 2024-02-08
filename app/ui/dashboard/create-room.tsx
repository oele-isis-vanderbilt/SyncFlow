'use client';

import { createRoom } from '@/app/lib/actions';

import { PlusIcon } from '@heroicons/react/24/outline';
import { Button } from '@/app/ui/button';
import { useState } from 'react';

export default function Page() {
  const [isBtnDisabled, setIsBtnDisabled] = useState(false);
  return (
    <Button
      className="ml-10 rounded-md bg-teal-900 p-2 text-white"
      onClick={async () => {
        setIsBtnDisabled(true);
        await createRoom();
        setIsBtnDisabled(false);
      }}
      aria-disabled={isBtnDisabled}
    >
      Create a new Room
      <PlusIcon className="ml-2 h-5 w-5 text-gray-50" />
    </Button>
  );
}
