import { lusitana } from '@/app/ui/fonts';
import {
  useTracks,
  TrackLoop,
  ParticipantTile,
  TrackReference,
  ParticipantAudioTile,
} from '@livekit/components-react';
import {
  Track,
  AudioTrack,
  LocalAudioTrack,
  RemoteAudioTrack,
} from 'livekit-client';
import '@livekit/components-styles';
import { useEffect, useRef } from 'react';

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
      <div className={'flex items-center justify-between bg-black'}>
        <h2 className={`${lusitana.className} p-2 text-xl md:text-2xl`}>
          {title}
        </h2>
      </div>
      <div className={'flex-1 overflow-y-scroll'}>
        {tracks.length > 0 &&
          tracks.map((track, index) => {
            return (
              <div key={track.publication?.trackSid || index}>
                {index === 0 && (
                  <div
                    className={
                      'grid grid-cols-3 gap-4 px-6 pb-3 pt-6 text-xl font-bold text-black'
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
                  <span className={'text-xl'}>
                    Publisher &#8594;{' '}
                    {track.participant.name || track.participant.identity}
                  </span>
                  <AudioRenderer
                    track={track.publication?.audioTrack!}
                    shouldMute={true}
                  />
                  <span>{track.participant.metadata || 'No metadata'}</span>
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
