import {
  TrackReference,
  TrackReferenceOrPlaceholder,
  useTracks,
  VideoTrack,
} from '@livekit/components-react';
import { Track } from 'livekit-client';
import { lusitana } from '@/app/ui/fonts';
import { PlayCircleIcon } from '@heroicons/react/16/solid';
import { Pagination, Tooltip } from 'flowbite-react';
import { useMediaQuery } from '@uidotdev/usehooks';
import { useState, useEffect } from 'react';
import clsx from 'clsx';
import { RxEnterFullScreen } from 'react-icons/rx';

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
    <div className={'flex h-full w-full flex-col'}>
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
      <div className="flex-1 gap-2">
        <div className="h-3/5 items-center justify-center bg-gray-300">
          {toRenderTrack ? (
            <>
              {toRenderTrack.publication?.isMuted ? (
                <VideoMutedIndicator trackRef={toRenderTrack} />
              ) : (
                <VideoTrack
                  trackRef={toRenderTrack as TrackReference}
                  controls={true}
                />
              )}
              <span className={'text-xl'}>{getHelpText(toRenderTrack)}</span>
            </>
          ) : (
            <NoTrackMessage />
          )}
        </div>
        <div className="h-2/5 pt-10">
          <h2 className={`${lusitana.className} py-2 text-xl md:text-2xl`}>
            Participants
          </h2>
          <PagedTrackView
            tracks={tracks}
            activeTrackSid={toRenderTrack?.publication?.trackSid}
            onTrackClick={(track) => {
              handleTrackSelection(track.publication?.trackSid!);
            }}
          />
        </div>
      </div>
    </div>
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
      <div className={'pt-5 self-center'}>
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
      <p className={'text-sm text-white hover:underline'}>
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

function VideoMutedIndicator({
  trackRef,
}: {
  trackRef: TrackReferenceOrPlaceholder;
}) {
  return (
    <div className={'flex h-full flex-col items-center justify-center'}>
      <p className={'text-lg text-black md:text-2xl'}>
        {getHelpText(trackRef)}
      </p>
      <p className={'text-lg text-black md:text-2xl'}>Video Muted</p>
    </div>
  );
}
