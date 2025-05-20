import type { EgressInfo, ParticipantInfo } from 'livekit-server-sdk';

export interface Project {
  id: string;
  name: string;
  description: string;
  livekitServerUrl: string;
  endpoint: string;
  bucketName: string;
  storageType: string;
  lastUpdated: number;
}

export interface ProjectsSummary {
  numProjects: number;
  numSessions: number;
  numActiveSessions: number;
}

export interface ProjectSummary {
  numSessions: number;
  numActiveSessions: number;
  numParticipants: number;
  numRecordings: number;
}

export interface ProjectSession {
  id: string;
  name: string;
  startedAt: number;
  comments: string;
  empty_timeout: number;
  maxParticipants: number;
  livekitRoomName: string;
  projectId: string;
  status: string;
  numParticipants: number;
  numRecordings: number;
  participants?: SessionParticipant[];
  recordings?: SessionEgress[];
}

export interface SessionParticipant {
  id: string;
  identity: string;
  name?: string;
  joinedAt: number;
  leftAt: number;
  sessionId: string;
  tracks: SessionTrack[];
}

export interface SessionTrack {
  id: string;
  sid: String;
  name?: string;
  kind: string;
  source: string;
  participantId: string;
  multimediaDetails?: MultimediaDetails;
}

export interface ProjectDevice {
  id: string;
  name: string;
  group: string;
  comments: string;
  registeredAt: number;
  registeredBy: number;
  projectId: string;
}

export interface SessionTokenResponse {
  token: string;
  identity: string;
  livekitServerUrl: string;
}

export interface SessionEgress {
  id: string;
  trackId: string;
  egressId: string;
  startedAt: number;
  egressType: string;
  status: string;
  destination: string;
  roomName: string;
  sessionId: string;
}

export interface EgressMediaPath {
  path: string;
}

export interface EgressMediaDownloadResponse {
  mediaPath: string;
  mediaUrl: string;
  bucketName: string;
  expiresIn: number;
}

export interface MultimediaDetails {
  destination: string;
  fileName: string;
  presignedUrl: string;
  presignedUrlExpires: number;
  publisher: string;
  recordingStartTime: number;
  trackId: string;
}
