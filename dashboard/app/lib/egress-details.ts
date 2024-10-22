import {
  EgressInfo,
  type TrackEgressRequest,
  EgressStatus,
} from '@livekit/protocol';
import type { JsonValue } from '@bufbuild/protobuf';

export class EgressDetails {
  private egressInfo: EgressInfo;
  constructor(egressInfo: EgressInfo) {
    this.egressInfo = egressInfo;
  }

  static fromJson(json: JsonValue) {
    return new EgressDetails(EgressInfo.fromJson(json));
  }

  get egressId() {
    return this.egressInfo.egressId;
  }

  get trackSid() {
    if (this.isTrackEgress()) {
      const egressRequest = this.egressInfo.request.value as TrackEgressRequest;
      return egressRequest.trackId;
    }
    return undefined;
  }

  isEgressActive() {
    return [EgressStatus.EGRESS_ACTIVE, EgressStatus.EGRESS_STARTING].includes(
      // biome-ignore lint/style/noNonNullAssertion: <explanation>
      this.egressInfo.status!,
    );
  }

  isEgressCompleted() {
    return this.egressInfo.status === EgressStatus.EGRESS_COMPLETE;
  }

  isRoomCompositeEgress() {
    return this.egressInfo.request.case === 'roomComposite';
  }

  isTrackEgress() {
    return this.egressInfo.request.case === 'track';
  }

  isTrackCompositeEgress() {
    return this.egressInfo.request.case === 'trackComposite';
  }
}
