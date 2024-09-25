import { VideoGrant } from 'livekit-server-sdk';

export interface CreateRoomRequest {
  name: string;
  options?: CreateRoomOptions;
  autoRecording?: boolean;
}

export interface CreateRoomOptions {
  maxParticipants: number;
  emptyTimeout: number;
  metadata: string;
}

export interface TokenResponse {
  token: string;
  identity: string;
}
