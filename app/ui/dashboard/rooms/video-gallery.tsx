import {
  TrackRefContext,
  ParticipantTile,
  TrackReferenceOrPlaceholder,
  useTracks,
  VideoTrack,
  ParticipantContextIfNeeded,
  TrackLoop,
  useRoomContext,
  ParticipantName,
} from '@livekit/components-react';
import type { TrackPublication } from 'livekit-client';
import { Track } from 'livekit-client';
import { lusitana } from '@/app/ui/fonts';
import { PlayCircleIcon } from '@heroicons/react/16/solid';
import { Pagination, Tooltip } from 'flowbite-react';
import { useMediaQuery } from '@uidotdev/usehooks';
import { useState, useEffect, useRef } from 'react';
import clsx from 'clsx';
import { Button } from '@/app/ui/button';
import { RxEnterFullScreen } from 'react-icons/rx';
import { ParticipantInfo } from 'livekit-server-sdk';

function getHelpText(track: TrackReferenceOrPlaceholder) {
  if (track.publication?.source === Track.Source.Camera) {
    return track.participant.identity + "'s Video";
  }
  if (track.publication?.source === Track.Source.Microphone) {
    return track.participant.identity + "'s Audio";
  }
  if (track.publication?.source === Track.Source.ScreenShare) {
    return track.participant.identity + "'s Screen";
  }
  return 'Unknown';
}

export default function VideoGallery(
  { title }: { title: string } = { title: 'Video Streams' },
) {
  const tracks = useTracks(
    [
      { source: Track.Source.Camera, withPlaceholder: false },
      { source: Track.Source.ScreenShare, withPlaceholder: false },
      // { source: Track.Source.Microphone, withPlaceholder: false },
    ],
    { onlySubscribed: true },
  );

  function handleTrackSelection(trackSid: string) {
    const targetTrack = tracks.find(
      (t) => t.publication?.trackSid === trackSid,
    );
    setToRenderTrack(targetTrack || null);
  }

  const [toRenderTrack, setToRenderTrack] =
    useState<TrackReferenceOrPlaceholder | null>(null);

  useEffect(() => {
    const isRenderedTrackActive = (
      track: TrackReferenceOrPlaceholder | null,
    ) => {
      return !!tracks.find(
        (t) => t.publication?.trackSid === track?.publication?.trackSid,
      );
    };

    if (tracks.length > 0 && !toRenderTrack) {
      setToRenderTrack(tracks[0]);
    }

    if (!isRenderedTrackActive(toRenderTrack)) {
      setToRenderTrack(tracks[0]);
    }
  }, [tracks]);

  return (
    <>
      <div className={'flex items-center justify-between bg-black'}>
        <h2 className={`${lusitana.className} p-2 text-xl md:text-2xl`}>
          {title}
        </h2>
        <Tooltip content={'Enter Full Screen(Grid View)'}>
          <div
            role={'button'}
            onClick={() => {
              //     ToDo: Implement Full Screen(Grid View)
            }}
          >
            <RxEnterFullScreen className={'mr-2 h-6 w-6'} />
          </div>
        </Tooltip>
      </div>

      {/* A div here that divides the available vertical space into 70-30 */}
      <div className="flex h-full flex-col">
        <div
          className="items-center justify-center bg-white"
          style={{ height: '800px' }}
        >
          {toRenderTrack ? (
            <>
              <VideoTrack trackRef={toRenderTrack} controls={true} />
              <span className={'text-xl'}>{getHelpText(toRenderTrack)}</span>
            </>
          ) : (
            <NoTrackMessage />
          )}
        </div>
        <div className="h-2/5">
          <h2 className={`${lusitana.className} mt-10 p-2 text-xl md:text-2xl`}>
            Participants
          </h2>
          <PagedTrackView
            tracks={tracks}
            activeTrackSid={toRenderTrack?.publication?.trackSid}
            onTrackClick={(track) => {
              handleTrackSelection(track.publication?.trackSid);
            }}
          />
        </div>
      </div>
    </>
  );
}

function PagedTrackView({
  tracks,
  activeTrackSid,
  onTrackClick,
}: {
  tracks: TrackReferenceOrPlaceholder[];
  activeTrackSid?: string;
  onTrackClick: (track: TrackReferenceOrPlaceholder) => void;
}) {
  const isMdScreen = useMediaQuery('(min-width: 768px)');
  const [currentPage, setCurrentPage] = useState(1);

  const changePage = (page: number) => {
    setCurrentPage(page);
    setCurrentPageTracks([...pages[page - 1]]);
  };

  const NUM_TRACKS_PER_PAGE = isMdScreen ? 6 : 2;

  const pages = tracks.reduce(
    (acc, track, index) => {
      const pageIndex = Math.floor(index / NUM_TRACKS_PER_PAGE);
      if (!acc[pageIndex]) {
        acc[pageIndex] = [];
      }
      acc[pageIndex].push(track);
      return acc;
    },
    [[]] as TrackReferenceOrPlaceholder[][],
  );

  const [currentPageTracks, setCurrentPageTracks] = useState<
    TrackReferenceOrPlaceholder[]
  >([]);

  useEffect(() => {
    setCurrentPage(1);
    setCurrentPageTracks([...pages[0]]);
  }, [tracks]);

  return (
    <div className={'flex h-full w-full flex-col'}>
      <div className={'grid grid-cols-2 gap-2 md:grid-cols-6'}>
        {currentPageTracks?.map((track, index) => {
          return (
            <Tooltip key={index} content={getHelpText(track)}>
              <VideoTile
                track={track}
                selected={activeTrackSid === track.publication?.trackSid}
                {...{
                  role: 'button',
                  onClick: () => onTrackClick(track),
                  disabled: !track.publication?.isMuted,
                }}
              />
            </Tooltip>
          );
        })}
      </div>
      <div className={'mt-5 self-center'}>
        {pages.length > 1 ? (
          <Pagination
            totalPages={pages.length}
            currentPage={currentPage}
            onPageChange={changePage}
          />
        ) : null}
      </div>
    </div>
  );
}

function VideoTile({
  track,
  selected,
  ...props
}: {
  track: TrackReferenceOrPlaceholder | undefined;
  selected: boolean;
  props?: any;
}) {
  const active = track?.publication?.isMuted
    ? 'text-red-500'
    : selected
      ? 'text-green-500'
      : 'text-gray-500';

  return (
    <div className={'flex flex-col items-center'} {...props}>
      <PlayCircleIcon className={clsx('h-12 w-12', active)} />
      <p className={'text-sm text-white hover:underline md:text-lg'}>
        {track ? getHelpText(track) : 'Unknown Track'}
      </p>
    </div>
  );
}

function NoTrackMessage() {
  return (
    <div className={'flex h-full items-center justify-center'}>
      <p className={'text-lg text-black md:text-2xl'}>
        No Video Streams Available
      </p>
    </div>
  );
}
