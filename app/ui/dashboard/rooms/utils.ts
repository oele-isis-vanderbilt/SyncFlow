import { TrackReferenceOrPlaceholder } from '@livekit/components-react';
import { Track } from 'livekit-client';

export function getHelpText(track: TrackReferenceOrPlaceholder) {
  if (track.publication?.source === Track.Source.Camera) {
    return track.participant.identity + "'s Video";
  }
  if (track.publication?.source === Track.Source.Microphone) {
    return track.participant.identity + "'s Audio";
  }
  if (track.publication?.source === Track.Source.ScreenShare) {
    return track.participant.identity + "'s Screen";
  }
  if (track.publication?.source === Track.Source.ScreenShareAudio) {
    return track.participant.identity + "'s Screen Audio";
  }
  return 'Unknown';
}
