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
}

export interface LivekitSessionInfo {
  roomName: string;
  roomSid: string;
  participants: ParticipantInfo[];
  recordings: EgressInfo[];
  duration: number;
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
