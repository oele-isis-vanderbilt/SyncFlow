'use client';
import { Button as ActionButton } from '@/app/ui/button';
import { Project, ProjectSession } from '@/types/project';
import {
  ExclamationCircleIcon,
  FingerPrintIcon,
  PlusIcon,
} from '@heroicons/react/24/outline';
import { Modal } from 'flowbite-react';
import { lusitana } from '@/app/ui/fonts';
import { useState } from 'react';
import { useFormState } from 'react-dom';
import { Checkbox, Input, RangeSlider } from '../../input';
import { BsFillChatSquareTextFill } from 'react-icons/bs';
import {
  createProjectSession,
  FormSubmissionState,
} from '@/app/lib/project-actions';

import { useEffect } from 'react';

export function CreateSession({ project }: { project: Project }) {
  const [showModal, setShowModal] = useState(false);
  return (
    <>
      <ActionButton
        className="ml-10 rounded-md bg-teal-900 p-2 text-white"
        onClick={() => {
          setShowModal(true);
        }}
      >
        Start a new Session
        <PlusIcon className="ml-2 h-5 w-5 text-gray-50" />
      </ActionButton>
      <CreateSessionModal
        show={showModal}
        projectName={project.name}
        projectId={project.id}
        onClose={() => setShowModal(false)}
      />
    </>
  );
}

function CreateSessionModal({
  show,
  projectName,
  projectId,
  onClose,
}: {
  show: boolean;
  projectName: string;
  projectId: string;
  onClose: () => void;
}) {
  let [messages, dispatch] = useFormState(
    createProjectSession.bind(null, projectId),
    null,
  );

  useEffect(() => {
    if (messages?.success) {
      dispatch(null);
      onClose();
    }
  }, [messages]);

  return (
    <Modal
      show={show}
      size={'4xl'}
      onClose={() => {
        dispatch(null);
        onClose();
      }}
    >
      <Modal.Header className={`${lusitana.className} text-4xl font-bold`}>
        Create a new session for project {projectName}
      </Modal.Header>

      <Modal.Body>
        <form action={dispatch} id="createSessionForm">
          <div className="w-full">
            <h3 className="text-xl dark:text-white">Session Details</h3>
            <Input
              id="name"
              label="Name of the new session"
              type="text"
              placeholder="leave blank for auto-generated session name"
              required={false}
            >
              <FingerPrintIcon className="pointer-events-none absolute left-3 top-1/2 h-[18px] w-[18px] -translate-y-1/2 text-gray-500 peer-focus:text-gray-900" />
            </Input>
            <Input
              id="comments"
              label="comments"
              type="text"
              placeholder="comments"
              required={false}
            >
              <BsFillChatSquareTextFill className="pointer-events-none absolute left-3 top-1/2 h-[18px] w-[18px] -translate-y-1/2 text-gray-500 peer-focus:text-gray-900" />
            </Input>
            <RangeSlider
              id="maxParticipants"
              label="Max Participants"
              min={1}
              max={200}
              step={1}
              defaultValue={100}
              onChange={(e) => {}}
            />
            <RangeSlider
              id="emptyTimeout"
              label="Empty Timeout (seconds)"
              min={600}
              max={3600}
              step={60}
              defaultValue={600}
              onChange={(e) => {}}
            />
            <Checkbox
              id="autoRecording"
              label="Automatically start recording when the session starts"
              checked={false}
            ></Checkbox>
          </div>
          <div aria-live="polite" aria-atomic="true">
            {messages &&
              !messages.success &&
              messages.errors?.length &&
              messages.errors?.length > 0 &&
              messages.errors?.map((message, index) => (
                <p
                  key={index}
                  className="flex items-center gap-2 p-2 text-xs text-red-500"
                >
                  <ExclamationCircleIcon className="h-5 w-5 text-red-500" />
                  {message}
                </p>
              ))}
          </div>
        </form>
      </Modal.Body>
      <Modal.Footer>
        <ActionButton
          className="bg-teal-900 text-white hover:bg-teal-800"
          type="submit"
          form="createSessionForm"
        >
          Create Session
        </ActionButton>
        <ActionButton
          onClick={() => {
            dispatch(null);
            onClose();
          }}
        >
          Close
        </ActionButton>
      </Modal.Footer>
    </Modal>
  );
}
