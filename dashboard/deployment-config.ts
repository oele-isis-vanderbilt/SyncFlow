import type { DeploymentConfig } from '@/types/deployment-config-models';
import { UserPermissions } from '@/types/deployment-config-models';

const config = {
  tagLine:
    '<strong>Welcome to LiveKitELP.</strong> Use the dashboard to manage embodied learning [<code>GEM-STEP</code>] data collection with LiveKit.',
  logoPath: '/livekit-elp.svg',
  userPermissions: UserPermissions.publishOnly,
  mmla_api_url: 'http://localhost:8081',
} as DeploymentConfig;

export default config;
