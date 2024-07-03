import type { DeploymentConfig } from '@/types/deployment-config-models';
import { UserPermissions } from '@/types/deployment-config-models';

const environment = process.env.NODE_ENV || 'development';
let config: DeploymentConfig | null = null;

export default function getConfig(): DeploymentConfig {
  if (config) {
    return config;
  } else {
    try {
      config = require(`./${environment}.json`) as DeploymentConfig;
      // Convert to enum
      if (config.userPermissions) {
        config.userPermissions = UserPermissions[config.userPermissions];
      }
    } catch (e) {
      throw new Error(`Unknown environment: ${environment}`);
    }
  }
  return config as DeploymentConfig;
}
