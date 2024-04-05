import { ReceivedDataMessage } from '@livekit/components-core';
import { LazyLog } from '@melloware/react-logviewer';
import Select from 'react-select';
import { customSelectStyles } from '@/app/ui/dashboard/rooms/widgets/utils';
import { useState } from 'react';

export function LogViewerWidget({
  logMessages,
}: {
  logMessages: ReceivedDataMessage<'log'>[];
}) {
  const decoder = new TextDecoder('utf-8');
  let [participants, setParticipants] = useState<Set<string>>(new Set());
  let [participantsFilter, setParticipantsFilter] = useState<string[]>([]);

  function processMessage(message: ReceivedDataMessage<'log'>) {
    if (message.from?.identity && !participants.has(message.from.identity)) {
      setParticipants(new Set(participants.add(message.from.identity)));
    }
    return `${new Date().toLocaleTimeString()} ${message.from?.identity || 'UNKNOWN'}: ${decoder.decode(message.payload)}`;
  }

  function formatMessages(messages: ReceivedDataMessage<'log'>[]) {
    return messages
      .filter(
        (message) =>
          participantsFilter.length === 0 ||
          participantsFilter.includes(message.from?.identity || ''),
      )
      .map(processMessage)
      .join('\n');
  }

  const onSelectOptionChanged = (selectedOptions) => {
    if (selectedOptions) {
      const selectedParticipants = selectedOptions.map(
        (option) => option.value,
      );
      setParticipantsFilter(selectedParticipants);
    } else {
      setParticipantsFilter([]);
    }
  };

  // Make label/value pairs for the participants
  let options = Array.from(participants).map((participant) => {
    return { value: participant, label: participant };
  });

  return (
    <div className={'h-full w-full p-2'}>
      <Select
        isMulti
        options={options}
        styles={customSelectStyles}
        onChange={onSelectOptionChanged}
        placeholder={'Filter logs by participants'}
      />
      <div className="h-full w-full">
        <LazyLog
          caseInsensitive
          enableHotKeys
          enableLinks
          enableSearch
          selectableLines
          text={formatMessages(logMessages)}
        />
      </div>
    </div>
  );
}
