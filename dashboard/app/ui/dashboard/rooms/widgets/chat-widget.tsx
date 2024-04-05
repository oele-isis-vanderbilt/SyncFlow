'use client';

import { ChatEntry } from '@livekit/components-react';
import { ReceivedDataMessage } from '@livekit/components-core';
import {
  AutoSizer,
  List,
  CellMeasurer,
  CellMeasurerCache,
  ListRowProps,
} from 'react-virtualized';

export function ChatViewerWidget({
  chatMessages,
}: {
  chatMessages: ReceivedDataMessage[];
}) {
  const decoder = new TextDecoder('utf-8');

  const cache = new CellMeasurerCache({
    fixedWidth: true,
    defaultHeight: 100, // A default height estimation
  });

  const ChatMessageFormatter = (message: string) => {
    return (
      <div className={'rounded-md bg-gray-700 p-2 text-white'}>{message}</div>
    );
  };
  const renderItem = ({ key, index, style, parent }: ListRowProps) => {
    const chatMessage = chatMessages[index];
    return (
      <CellMeasurer
        key={key}
        cache={cache}
        parent={parent}
        columnIndex={0}
        rowIndex={index}
      >
        {({ measure, registerChild }) => (
          <div ref={registerChild} key={key} style={style} onLoad={measure}>
            <ChatEntry
              entry={{
                from: chatMessage.from,
                message: decoder.decode(chatMessage.payload),
                timestamp: Date.now(), // FixMe: This should be the actual timestamp
              }}
              hideName={false}
              hideTimestamp={false}
              messageFormatter={ChatMessageFormatter}
            />
          </div>
        )}
      </CellMeasurer>
    );
  };

  return (
    <div className={'-my-2 h-full w-full  p-2'}>
      {chatMessages.length === 0 ? (
        <div className={'flex h-full w-full items-center justify-center'}>
          <p className={'text-white'}>No messages yet</p>
        </div>
      ) : (
        <div className="-my-2 h-full w-full  flex-grow">
          <div className="h-full w-full">
            {' '}
            <AutoSizer>
              {({ width, height }) => (
                <List
                  width={width}
                  height={height}
                  rowCount={chatMessages.length}
                  deferredMeasurementCache={cache}
                  rowHeight={cache.rowHeight}
                  rowRenderer={renderItem}
                  overscanRowCount={3}
                />
              )}
            </AutoSizer>
          </div>
        </div>
      )}
    </div>
  );
}
