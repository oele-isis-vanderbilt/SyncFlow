'use client';
import { lusitana } from '@/app/ui/fonts';
import { HiArchive, HiChat, HiClipboardList } from 'react-icons/hi';
import { Tab, TabList, TabPanel, Tabs } from 'react-tabs';
import { ChatViewerWidget } from '@/app/ui/dashboard/rooms/widgets/chat-widget';
import { LogViewerWidget } from '@/app/ui/dashboard/rooms/widgets/log-widget';
import 'react-tabs/style/react-tabs.css';
import { useState } from 'react';
import { ReceivedDataMessage } from '@livekit/components-core';
import { useDataChannel } from '@livekit/components-react';

export default function TopicalMessages(
  { title }: { title: string } = { title: 'Topical Messages' },
) {
  const [chatMessages, setChatMessages] = useState<
    ReceivedDataMessage<'chat'>[]
  >([]);

  const [logMessages, setLogMessages] = useState<ReceivedDataMessage<'log'>[]>(
    [],
  );

  const [allMessages, setAllMessages] = useState<ReceivedDataMessage[]>([]);

  useDataChannel('chat', (msg) => {
    if (chatMessages.length < 1000) {
      setChatMessages((prev) => [msg, ...prev]);
    } else {
      setChatMessages((prev) => [...prev.slice(1), msg]);
    }
  });

  useDataChannel('log', (msg) => {
    if (logMessages.length < 1000) {
      setLogMessages((prev) => [msg, ...prev]);
    } else {
      setLogMessages((prev) => [...prev.slice(1), msg]);
    }
  });

  useDataChannel((msg) => {
    if (logMessages.length < 1000) {
      setAllMessages((prev) => [msg, ...prev]);
    } else {
      setAllMessages((prev) => [...prev.slice(1), msg]);
    }
  });

  return (
    <div className={'flex h-full w-full flex-col'}>
      <div className={'flex items-center justify-between'}>
        <h2 className={`${lusitana.className} p-2 text-xl md:text-2xl`}>
          {title}
        </h2>
      </div>
      <div className="h-full w-full flex-grow">
        <div className={'flex h-full w-full flex-row'}>
          <Tabs className={'h-full w-full'}>
            <TabList>
              <Tab>
                <div className={'flex items-center text-xl'}>
                  <HiArchive />
                  All
                </div>
              </Tab>
              <Tab>
                <div className={'flex items-center text-xl'}>
                  <HiChat />
                  Chat
                </div>
              </Tab>
              <Tab>
                <div className={'flex items-center text-xl'}>
                  <HiClipboardList />
                  Logs
                </div>
              </Tab>
            </TabList>

            <TabPanel
              selectedClassName={
                'react-tabs__tab-panel--selected h-full w-full'
              }
            >
              <LogViewerWidget logMessages={allMessages} />
            </TabPanel>
            <TabPanel
              selectedClassName={
                'react-tabs__tab-panel--selected h-full w-full'
              }
            >
              <ChatViewerWidget chatMessages={chatMessages} />
            </TabPanel>
            <TabPanel
              selectedClassName={
                'react-tabs__tab-panel--selected h-full w-full'
              }
            >
              <LogViewerWidget logMessages={logMessages} />
            </TabPanel>
          </Tabs>
        </div>
      </div>
    </div>
  );
}
