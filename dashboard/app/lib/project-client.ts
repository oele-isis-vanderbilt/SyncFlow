import getConfig from '@/config';
import { AuthHttpClient, ClientError } from './auth-http-client';
import type {
  LivekitSessionInfo,
  ProjectSession,
  Project,
  ProjectsSummary,
  ProjectSummary,
} from '@/types/project';

const PREFIXES = {
  CREATE_PROJECT: '/projects/create',
  LIST_PROJECTS: '/projects/list',
  DELETE_PROJECT: '/projects',
  GET_PROJECT: '/projects',
  SUMMARIZE_USER_PROJECTS: '/projects/summarize',
};

import { z } from 'zod';
import { TokenResponse } from '@/types/mmla';
import { VideoGrant } from 'livekit-server-sdk';

export const NewProjectSchema = z.object({
  name: z.string(),
  description: z.string().optional(),

  livekitServerUrl: z.string(),
  livekitServerApiKey: z.string(),
  livekitServerApiSecret: z.string(),

  accessKey: z.string(),
  secretKey: z.string(),
  bucketName: z.string(),
  endpoint: z.string(),
  region: z.string(),
  storageType: z.string().optional(),
});

export const NewSessionSchema = z.object({
  name: z.string().optional(),
  comments: z.string().optional(),
  maxParticipants: z
    .string()
    .optional()
    .default('100')
    .transform((v) => parseInt(v)),
  emptyTimeout: z
    .string()
    .optional()
    .default('600')
    .transform((v) => parseInt(v)),
  autoRecording: z
    .string()
    .optional()
    .default('off')
    .transform((v) => v === 'on'),
});

export type NewProject = z.infer<typeof NewProjectSchema>;
export type NewSession = z.infer<typeof NewSessionSchema>;

export class ProjectClient extends AuthHttpClient {
  constructor(base_url: string) {
    super(base_url);
  }

  async listProjects() {
    return await this.authenticatedGet<Project[]>(PREFIXES.LIST_PROJECTS);
  }

  async deleteProject(id: string) {
    return await this.authenticatedDelete(`${PREFIXES.DELETE_PROJECT}/${id}`);
  }

  async createProject(project: NewProject) {
    return await this.authenticatedPost<Project, NewProject>(
      PREFIXES.CREATE_PROJECT,
      project,
    );
  }

  async getProject(id: string) {
    return await this.authenticatedGet<Project>(
      `${PREFIXES.GET_PROJECT}/${id}`,
    );
  }

  async summarizeUserProjects() {
    return await this.authenticatedGet<ProjectsSummary>(
      PREFIXES.SUMMARIZE_USER_PROJECTS,
    );
  }

  async summarizeProject(projectId: string) {
    return await this.authenticatedGet<ProjectSummary>(
      `${PREFIXES.GET_PROJECT}/${projectId}/summarize`,
    );
  }

  async createSession(projectId: string, session: NewSession) {
    return await this.authenticatedPost<ProjectSession, NewSession>(
      `${PREFIXES.GET_PROJECT}/${projectId}/create-session`,
      session,
    );
  }

  async getLivekitSessionInfo(projectId: string, sessionId: string) {
    return await this.authenticatedGet<LivekitSessionInfo>(
      `${PREFIXES.GET_PROJECT}/${projectId}/sessions/${sessionId}/livekit-session-info`,
    );
  }

  async deleteSession(projectId: string, sessionId: string) {
    return await this.authenticatedDelete<ProjectSession>(
      `${PREFIXES.GET_PROJECT}/${projectId}/sessions/${sessionId}`,
    );
  }

  async stopSession(projectId: string, sessionId: string) {
    return await this.authenticatedPost<ProjectSession, {}>(
      `${PREFIXES.GET_PROJECT}/${projectId}/sessions/${sessionId}/stop`,
      {},
    );
  }

  async getSessions(projectId: string) {
    return await this.authenticatedGet<ProjectSession[]>(
      `${PREFIXES.GET_PROJECT}/${projectId}/sessions`,
    );
  }

  async getSession(projectId: string, sessionId: string) {
    return await this.authenticatedGet<ProjectSession>(
      `${PREFIXES.GET_PROJECT}/${projectId}/sessions/${sessionId}`,
    );
  }

  async getSessionToken(
    projectId: string,
    sessionId: string,
    identity: string,
    tokenRequest: VideoGrant,
  ) {
    return await this.authenticatedPost<
      TokenResponse,
      { identity: string; videoGrants: VideoGrant }
    >(`${PREFIXES.GET_PROJECT}/${projectId}/sessions/${sessionId}/token`, {
      identity: identity,
      videoGrants: tokenRequest,
    });
  }
}

const deploymentConfig = getConfig();
export const projectClient = new ProjectClient(deploymentConfig.mmla_api_url);
