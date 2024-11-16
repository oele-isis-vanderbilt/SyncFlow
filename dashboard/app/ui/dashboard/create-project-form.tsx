'use client';

import {
  ExclamationCircleIcon,
  FingerPrintIcon,
} from '@heroicons/react/24/outline';
import { Input, TextArea } from '../input';
import { FaGlobe } from 'react-icons/fa';
import { FaMapMarkerAlt } from 'react-icons/fa';
import { Button } from 'flowbite-react';
import { useFormState, useFormStatus } from 'react-dom';
import { createProject, FormSubmissionState } from '@/app/lib/project-actions';
import { Project } from '@/types/project';

export default function CreateProjectForm() {
  const { pending } = useFormStatus();
  const [messages, dispatch] = useFormState<
    Promise<FormSubmissionState<Project> | null>
  >(createProject, null);

  return (
    <form action={dispatch}>
      <div className="mx-auto w-full md:w-1/2">
        <h3 className="text-xl dark:text-white">Project Details</h3>
        <Input
          id="name"
          label="Name of the new project"
          type="text"
          placeholder="awesome-syncflow-project"
          required={true}
        >
          <FingerPrintIcon className="-translate-y-1/2 pointer-events-none absolute top-1/2 left-3 h-[18px] w-[18px] text-gray-500 peer-focus:text-gray-900" />
        </Input>
        <TextArea
          id="description"
          label="Description"
          type="text"
          placeholder="Project Description"
          required={false}
        />

        <h3 className="mt-4 text-xl dark:text-white">Livekit Settings</h3>
        <Input
          id="livekitServerUrl"
          label="Livekit Server WS/WSS URL"
          type="text"
          placeholder="wss://livekit.syncflow.live"
          required={true}
        >
          <FaGlobe className="-translate-y-1/2 pointer-events-none absolute top-1/2 left-3 h-[18px] w-[18px] text-gray-500 peer-focus:text-gray-900" />
        </Input>
        <Input
          id="livekitServerApiKey"
          label="Livekit Server ApiKey"
          type="text"
          placeholder="livekit-api-key"
          required={true}
        >
          <FingerPrintIcon className="-translate-y-1/2 pointer-events-none absolute top-1/2 left-3 h-[18px] w-[18px] text-gray-500 peer-focus:text-gray-900" />
        </Input>
        <Input
          id="livekitServerApiSecret"
          label="Livekit Server Api Secret"
          type="password"
          placeholder="livekit-api-secret"
          required={true}
        >
          <FingerPrintIcon className="-translate-y-1/2 pointer-events-none absolute top-1/2 left-3 h-[18px] w-[18px] text-gray-500 peer-focus:text-gray-900" />
        </Input>

        <h3 className="mt-4 text-xl dark:text-white">S3 Bucket</h3>
        <Input
          id="bucketName"
          label="S3 Bucket Name"
          type="text"
          placeholder="syncflow-livekit"
          required={true}
        >
          <FaGlobe className="-translate-y-1/2 pointer-events-none absolute top-1/2 left-3 h-[18px] w-[18px] text-gray-500 peer-focus:text-gray-900" />
        </Input>
        <Input
          id="accessKey"
          label="S3 Access Key"
          type="text"
          placeholder="syncflow-livekit-s3-access-key"
          required={true}
        >
          <FingerPrintIcon className="-translate-y-1/2 pointer-events-none absolute top-1/2 left-3 h-[18px] w-[18px] text-gray-500 peer-focus:text-gray-900" />
        </Input>
        <Input
          id="secretKey"
          label="S3 Secret Key"
          type="password"
          placeholder="syncflow-livekit-s3-secret-key"
          required={true}
        >
          <FingerPrintIcon className="-translate-y-1/2 pointer-events-none absolute top-1/2 left-3 h-[18px] w-[18px] text-gray-500 peer-focus:text-gray-900" />
        </Input>
        <Input
          id="endpoint"
          label="S3 Endpoint"
          type="text"
          placeholder="syncflow-livekit-s3-endpoint"
          required={false}
        >
          <FaGlobe className="-translate-y-1/2 pointer-events-none absolute top-1/2 left-3 h-[18px] w-[18px] text-gray-500 peer-focus:text-gray-900" />
        </Input>
        <Input
          id="region"
          label="S3 Region"
          type="text"
          placeholder="syncflow-livekit-s3-region"
          required={false}
        >
          <FaMapMarkerAlt className="-translate-y-1/2 pointer-events-none absolute top-1/2 left-3 h-[18px] w-[18px] text-gray-500 peer-focus:text-gray-900" />
        </Input>
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
        </div>
        <div className="mt-6 flex flex-row justify-center">
          <Button color="success" type="submit">
            Create Project
          </Button>
        </div>
      </div>
    </form>
  );
}
