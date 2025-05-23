'use client';

import { createApiKey } from '@/app/lib/project-actions';
import { Button as ActionButton } from '@/app/ui/button';
import { ExclamationCircleIcon, PlusIcon } from '@heroicons/react/24/outline';
import { Modal } from 'flowbite-react';
import { useState } from 'react';
import { CopyBlock, dracula } from 'react-code-blocks';
import { useFormState } from 'react-dom';
import { Input } from '../../input';
import { lusitana } from '../../fonts';
import { FaCommentAlt } from 'react-icons/fa';

export default function CreateApiKeys({ projectId }: { projectId: string }) {
  const [showModal, setShowModal] = useState(false);
  return (
    <>
      <ActionButton
        className="ml-10 rounded-md bg-teal-900 p-2 text-white"
        onClick={() => setShowModal(true)}
      >
        Create new API key
        <PlusIcon className="ml-2 h-5 w-5 text-gray-50" />
      </ActionButton>
      <CreateApiKeyModal
        projectId={projectId}
        show={showModal}
        onClose={() => setShowModal(false)}
      />
    </>
  );
}

export function CreateApiKeyModal({
  projectId,
  onClose,
  show,
}: {
  projectId: string;
  onClose: () => void;
  show: boolean;
}) {
  let [messages, dispatch] = useFormState(
    createApiKey.bind(null, projectId),
    null,
  );

  return (
    <Modal
      show={show}
      size={'5xl'}
      onClose={() => {
        dispatch(null);
        onClose();
      }}
    >
      <Modal.Header className={`${lusitana.className} font-bold text-4xl`}>
        Create a new API Key
      </Modal.Header>

      <Modal.Body>
        <form action={dispatch} id="createApiKeyForm">
          <div className="w-full">
            <h3 className="text-xl dark:text-white">Key Details</h3>
            <Input
              id="comment"
              label="Comments"
              type="text"
              placeholder="Enter a description for this API key"
              required={false}
            >
              <FaCommentAlt className="-translate-y-1/2 pointer-events-none absolute top-1/2 left-3 h-[18px] w-[18px] text-gray-500 peer-focus:text-gray-900" />
            </Input>
          </div>
          <div aria-live="polite" aria-atomic="true">
            {messages &&
              !messages.success &&
              messages.errors?.length &&
              messages.errors?.length > 0 &&
              messages.errors?.map((message, index) => (
                <p
                  key={index}
                  className="flex items-center gap-2 p-2 text-red-500 text-xs"
                >
                  <ExclamationCircleIcon className="h-5 w-5 text-red-500" />
                  {message}
                </p>
              ))}
            {messages?.success && (
              <>
                <p className="mt-4 text-green-500 text-xs dark:text-white">
                  API key created successfully (copy it now, it will not be
                  shown again)
                </p>
                <CopyBlock
                  text={JSON.stringify(messages?.data.json, null, 2)}
                  language={'javascript'}
                  theme={dracula}
                  showLineNumbers={true}
                />
                <p className="mt-4 text-green-500 text-xs dark:text-white">
                  ENV Format to use in your .env file
                </p>
                <CopyBlock
                  text={messages?.data.env.join('\n')}
                  language={'bash'}
                  theme={dracula}
                  showLineNumbers={true}
                />
              </>
            )}
          </div>
        </form>
      </Modal.Body>
      <Modal.Footer>
        {!messages?.success && (
          <ActionButton
            className="bg-teal-900 text-white hover:bg-teal-800"
            type="submit"
            form="createApiKeyForm"
          >
            Create API Key
          </ActionButton>
        )}
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
