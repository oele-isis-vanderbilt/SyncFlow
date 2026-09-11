import { lusitana } from '@/app/ui/fonts';
import { useTracks } from '@livekit/components-react';
import { Track, LocalAudioTrack, RemoteAudioTrack } from 'livekit-client';
import '@livekit/components-styles';
import { useEffect, useRef } from 'react';
import { shortenText } from '@/app/ui/dashboard/rooms/utils';

export default function AudioStreams(
  { title }: { title: string } = { title: 'Audio Streams' },
) {
  let tracks = useTracks(
    [
      { source: Track.Source.Microphone, withPlaceholder: false },
      { source: Track.Source.ScreenShareAudio, withPlaceholder: false },
    ],
    { onlySubscribed: true },
  );

  return (
    <div className={'flex h-full w-full flex-col'}>
      <div className={'flex items-center justify-between'}>
        <h2
          className={`${lusitana.className} p-2 text-xl md:text-2xl dark:text-white`}
        >
          {title}
        </h2>
      </div>
      <div className={'scrollbar-thin flex-1 overflow-y-auto'}>
        {tracks.length > 0 &&
          tracks.map((track, index) => {
            return (
              <div key={track.publication?.trackSid || index}>
                {index === 0 && (
                  <div
                    className={
                      'grid grid-cols-3 gap-4 px-6 pt-6 pb-3 font-bold text-xl dark:text-white'
                    }
                  >
                    <span>Publisher</span>
                    <span>Audio</span>
                    <span>Metadata</span>
                  </div>
                )}
                <div
                  className={
                    'grid grid-cols-3 items-center gap-4 p-6 text-black'
                  }
                >
                  <span className={'overflow-ellipsis text-xl dark:text-white'}>
                    Publisher &#8594;{' '}
                    {shortenText(track.participant.identity, 10)}
                  </span>
                  <AudioRenderer
                    track={track.publication?.audioTrack!}
                    shouldMute={true}
                  />
                  <span className="dark:text-white">
                    {track.participant.metadata || 'No metadata'}
                  </span>
                </div>
              </div>
            );
          })}
      </div>
    </div>
  );
}

const AudioRenderer = ({
  track,
  shouldMute,
}: {
  track: LocalAudioTrack | RemoteAudioTrack | undefined;
  shouldMute: boolean;
}) => {
  const audioRef = useRef<HTMLAudioElement>(null);

  useEffect(() => {
    if (audioRef.current) {
      track?.attach(audioRef.current);
      audioRef.current.muted = shouldMute;
    }
    return () => {
      track?.detach();
    };
  }, [track, shouldMute]);

  if (
    !(track instanceof LocalAudioTrack) &&
    !(track instanceof RemoteAudioTrack)
  ) {
    return null;
  }

  return <audio ref={audioRef} muted={shouldMute} controls={true} />;
};
