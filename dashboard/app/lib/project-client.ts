import getConfig from '@/config';
import { AuthHttpClient } from './auth-http-client';
import type { Project } from '@/types/project';

const PREFIXES = {
  CREATE_PROJECT: '/projects/create',
  LIST_PROJECTS: '/projects/list',
  DELETE_PROJECT: '/projects',
  GET_PROJECT: '/projects',
};

import { z } from 'zod';

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

export type NewProject = z.infer<typeof NewProjectSchema>;

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

  async startSession(projectId: string) {}

  async stopSession(projectId: string) {}

  async getSessions(projectId: string) {}
}

const deploymentConfig = getConfig();
export const projectClient = new ProjectClient(deploymentConfig.mmla_api_url);
