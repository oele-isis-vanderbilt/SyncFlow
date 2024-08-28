'use client';

import { Card, Button, Modal } from 'flowbite-react';
import clsx from 'clsx';
import { lusitana } from '../fonts';
import Link from 'next/link';
import { FaArrowRight } from 'react-icons/fa';
import { FaInfoCircle } from 'react-icons/fa';
import { MdDelete } from 'react-icons/md';
import type { Project } from '@/types/project';
import { useState } from 'react';
import InfoModal, { InfoModalContent } from './info-modal';
import { deleteProject } from '@/app/lib/project-actions';
import { dateFromTimestamp } from '../utils';

export function ProjectCard({ project }: { project: Project }) {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const localDateTime = dateFromTimestamp(project.lastUpdated);

  const projectToInfoModalContent = (
    project: Project,
    title?: string,
  ): InfoModalContent => {
    return {
      title: title || project.name,
      headings: [
        {
          title: 'Project Details',
          items: [
            {
              title: 'Name',
              content: project.name,
            },
            {
              title: 'ID',
              content: project.id,
            },
            {
              title: 'Description',
              content: project.description,
            },
            {
              title: 'Last Updated',
              content: localDateTime,
            },
          ],
        },
        {
          title: 'Project Storage',
          items: [
            {
              title: 'Bucket Name',
              content: project.bucketName,
            },
            {
              title: 'Endpoint',
              content: project.endpoint,
            },
          ],
        },
        {
          title: 'Livekit Configuration',
          items: [
            {
              title: 'Server URL',
              content: project.livekitServerUrl,
            },
          ],
        },
      ],
    };
  };

  let [infoModalContent, setInfoModalContent] =
    useState<InfoModalContent | null>(projectToInfoModalContent(project));

  return (
    <Card>
      <h5
        className={clsx(
          'text-2xl font-bold tracking-tight text-gray-900 dark:text-white',
          lusitana.className,
        )}
      >
        <Link
          href={`/dashboard/projects/${project.id}`}
          className="hover:underline"
        >
          {project.name}
        </Link>
      </h5>
      <div className="font-normal text-gray-700 dark:text-gray-400">
        <pre className="overflow-hidden text-ellipsis">id: {project.id}</pre>
        <p className="overflow-hidden text-ellipsis">{project.description}</p>
        <p className="overflow-hidden text-ellipsis text-xs italic">
          Last updated on {localDateTime}
        </p>
      </div>

      <div className="flex items-center gap-2">
        <Link href={`/dashboard/projects/${project.id}`}>
          <Button color="success">
            Open
            <FaArrowRight className="ml-2 h-5 w-5" />
          </Button>
        </Link>
        <Button
          color="light"
          onClick={() => {
            setInfoModalContent(projectToInfoModalContent(project));
            setIsModalOpen(true);
          }}
        >
          Details
          <FaInfoCircle className="ml-2 h-5 w-5" />
        </Button>

        <Button
          color="failure"
          onClick={async () => {
            let result = await deleteProject(project.id);
            if (result.status !== 'success') {
              setInfoModalContent({
                title: 'Error',
                headings: [
                  {
                    title: 'Error',
                    items: [
                      {
                        title: 'Message',
                        content: result.error,
                      },
                    ],
                  },
                ],
              });
              setIsModalOpen(true);
            }
          }}
        >
          Delete
          <MdDelete className="ml-2 h-5 w-5" />
        </Button>
      </div>
      <InfoModal
        content={infoModalContent}
        show={isModalOpen}
        onClose={() => {
          setIsModalOpen(false);
        }}
      />
    </Card>
  );
}
