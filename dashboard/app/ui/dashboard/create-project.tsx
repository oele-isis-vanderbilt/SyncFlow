'use client';

import { PlusIcon } from '@heroicons/react/24/outline';
import { useRef, useState } from 'react';
import { Button as ActionButton } from '@/app/ui/button';
import Link from 'next/link';

export default function Page() {
  const [showModal, setShowModal] = useState(false);
  const formRef = useRef<HTMLFormElement>(null);

  const handleSubmit = () => {
    if (formRef.current) {
      formRef.current.requestSubmit();
    }
  };

  return (
    <Link href="/dashboard/projects/create">
      <ActionButton
        className="ml-10 rounded-md bg-teal-900 p-2 text-white"
        onClick={() => setShowModal(true)}
      >
        Create a new Project
        <PlusIcon className="ml-2 h-5 w-5 text-gray-50" />
      </ActionButton>
    </Link>
  );
}
