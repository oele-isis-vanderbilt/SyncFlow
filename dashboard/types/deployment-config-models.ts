export enum UserPermissions {
  publishOnly,
  publishSubscribe,
}

export interface DeploymentConfig {
  tagLine: string;
  logoPath: string;
  userPermissions: UserPermissions;
  syncFlowApiUrl: string;
}
