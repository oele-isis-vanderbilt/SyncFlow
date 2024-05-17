import type { DeploymentConfig } from '@/types/deployment-config-models';
import { UserPermissions } from '@/types/deployment-config-models';

const config = {
  tagLine:
    '<strong>Welcome to SyncFlow.</strong> Use the dashboard to manage multimodal data collection with SyncFlow.',
  logoPath: '/syncflow.png',
  userPermissions: UserPermissions.publishOnly,
  mmla_api_url:
    process.env.MMLA_API_URL ||
    process.env.NEXT_PUBLIC_MMLA_API_URL ||
    'http://localhost:8081',
} as DeploymentConfig;

export default config;
