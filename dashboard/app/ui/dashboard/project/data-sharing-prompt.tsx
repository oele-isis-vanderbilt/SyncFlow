'use client';

import { useState } from 'react';
import SessionJoinForm from './session-join-form';
import { ProjectSession } from '@/types/project';
import { Tooltip } from 'flowbite-react';
import { GiMirrorMirror } from 'react-icons/gi';

export default function DataSharingPrompt({
  projectId,
  session,
}: {
  projectId: string;
  session: ProjectSession;
}) {
  let [showSessionJoinForm, setShowSessionJoinForm] = useState<boolean>(false);

  return (
    <>
      <SessionJoinForm
        show={showSessionJoinForm}
        onClose={() => setShowSessionJoinForm(false)}
        projectId={projectId}
        sessionId={session.id}
        sessionName={session.name}
      />
      <button
        onClick={async () => {
          setShowSessionJoinForm(true);
        }}
      >
        <Tooltip content="Open Session Data Sharing Page">
          <GiMirrorMirror className="cursor-pointer text-2xl text-red-600 hover:text-red-900" />
        </Tooltip>
      </button>
    </>
  );
}
