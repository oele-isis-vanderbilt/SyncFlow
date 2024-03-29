'use client';

import { useState } from 'react';
import { Button } from '@/app/ui/button';
import { ExclamationCircleIcon, PlusIcon } from '@heroicons/react/24/outline';
import { Modal } from 'flowbite-react';
import { lusitana } from '@/app/ui/fonts';
import { useFormState, useFormStatus } from 'react-dom';
import { ArrowRightIcon } from '@heroicons/react/20/solid';
import { createApiKeys, redirectToSettings } from '@/app/lib/actions';

export default function CreateApiKey() {
  const [isBtnDisabled, setIsBtnDisabled] = useState(false);
  const [openModal, setOpenModal] = useState(false);
  return (
    <>
      <Button
        className="ml-10 rounded-md bg-teal-900 p-2 text-white"
        onClick={() => {
          setIsBtnDisabled(true);
          setOpenModal(true);
          setIsBtnDisabled(false);
        }}
        aria-disabled={isBtnDisabled}
      >
        Create new API key
        <PlusIcon className="ml-2 h-5 w-5 text-gray-50" />
      </Button>
      <CreateAPIKeyModal
        show={openModal}
        initialState={{}}
        onClose={async () => {
          setOpenModal(false);
          await redirectToSettings();
        }}
      />
    </>
  );
}

function CreateAPIKeyModal({
  show,
  onClose,
  initialState,
}: {
  show: boolean;
  onClose: () => void;
  initialState: any;
}) {
  const [state, dispatch] = useFormState(createApiKeys, initialState);

  return (
    <Modal show={show}>
      <Modal.Header className={`${lusitana.className}`}>
        Create a new API Key
      </Modal.Header>
      <Modal.Body>
        <form
          action={(formData) => {
            dispatch({
              type: 'create',
              formData: formData,
            });
          }}
        >
          <div className="flex flex-row items-center">
            <label className="text-black" htmlFor="description">
              Description
            </label>
            <input
              type="text"
              id="description"
              className="mx-2 text-black"
              name="description"
            />
            <div>
              <CreateKeyButton />
            </div>
          </div>
          {state?.error && (
            <>
              <ExclamationCircleIcon className="h-5 w-5 text-red-500" />
              <p className="text-xs text-red-500">{state.error}</p>
            </>
          )}
          {state?.success && (
            <>
              <p className="text-xs text-green-500">
                API key created successfully (copy it now, it will not be shown
                again)
              </p>
              <pre className={'text-black'}>
                {JSON.stringify(state?.success, null, 2)}
              </pre>
            </>
          )}
        </form>
      </Modal.Body>
      <Modal.Footer>
        <Button
          onClick={() => {
            dispatch({ type: 'reset' });
            onClose();
          }}
        >
          Close
        </Button>
      </Modal.Footer>
    </Modal>
  );
}

function CreateKeyButton() {
  const { pending } = useFormStatus();
  return (
    <Button
      type={'submit'}
      className="w-full bg-teal-900"
      aria-disabled={pending}
    >
      Create key <ArrowRightIcon className="ml-auto h-5 w-5 text-gray-50" />
    </Button>
  );
}
