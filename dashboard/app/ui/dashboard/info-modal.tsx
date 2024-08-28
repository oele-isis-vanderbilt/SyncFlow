'use client';

import { Modal } from 'flowbite-react';
import { Button } from '../button';

export interface InfoModalContent {
  title: string;
  headings: InfoModalHeading[];
}

export interface InfoModalHeading {
  title: string;
  items: InfoModalItem[];
}

export interface InfoModalItem {
  title: string;
  content: string;
}

export default function InfoModal({
  content,
  show,
  onClose,
}: {
  content: InfoModalContent | null;
  show: boolean;
  onClose: () => void;
}) {
  return (
    <Modal dismissible show={show} onClose={onClose}>
      <Modal.Header>{content?.title}</Modal.Header>
      <Modal.Body>
        <div className="space-y-6">
          {content?.headings.map((heading, index) => (
            <div key={index}>
              <h3 className="text-lg font-bold dark:text-white">
                {heading.title}
              </h3>
              <div className="space-y-2">
                {heading.items.map((item, itemIndex) => (
                  <div key={itemIndex}>
                    <h4 className="text-md font-bold dark:text-white">
                      {item.title}
                    </h4>
                    <p className="dark:text-white">{item.content}</p>
                  </div>
                ))}
              </div>
            </div>
          ))}
        </div>
      </Modal.Body>
      <Modal.Footer>
        <Button onClick={() => onClose()}>Close</Button>
      </Modal.Footer>
    </Modal>
  );
}
