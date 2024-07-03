import type { DeploymentConfig } from '@/types/deployment-config-models';
import { UserPermissions } from '@/types/deployment-config-models';

const environment = process.env.NODE_ENV || 'development';

// Dynamically import the correct config file based on the environment
let config = null;

try {
  config = require(`./${environment}.json`);
} catch (e) {
  throw new Error(`Unknown environment: ${environment}`);
}

// Convert to enum
if (config.userPermissions) {
  config.userPermissions = UserPermissions[config.userPermissions];
}

export default config as any as DeploymentConfig;
