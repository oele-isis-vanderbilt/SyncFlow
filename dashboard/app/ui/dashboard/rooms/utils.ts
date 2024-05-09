import { TrackReferenceOrPlaceholder } from '@livekit/components-react';
import { Track } from 'livekit-client';

export function shortenText(text: string, maxLength: number = 10) {
  if (text.length <= maxLength) {
    return text;
  }

  return text.substring(0, maxLength) + '...';

}

export function getHelpText(track: TrackReferenceOrPlaceholder, long=true) {
  if (track.publication?.trackName) {
    return track.publication.trackName;
  }

  let participantText = long ? track.participant.identity:  shortenText(track.participant.identity)

  if (track.publication?.source === Track.Source.Camera) {
    return  participantText + "'s Video";
  }
  if (track.publication?.source === Track.Source.Microphone) {
    return participantText + "'s Audio";
  }
  if (track.publication?.source === Track.Source.ScreenShare) {
    return participantText + "'s Screen";
  }
  if (track.publication?.source === Track.Source.ScreenShareAudio) {
    return participantText + "'s Screen Audio";
  }
  return 'Unknown';
}
