import type { DeploymentConfig } from '@/types/deployment-config-models';
import { UserPermissions } from '@/types/deployment-config-models';
import * as fs from 'fs';
import * as path from 'path';

const environment = process.env.NODE_ENV || 'development';
let config: DeploymentConfig | null = null;

export default function getConfig(): DeploymentConfig {
  if (config) {
    return config;
  } else {
    try {
      config = JSON.parse(
        fs.readFileSync(
          path.join(process.cwd(), `/config/${environment}.json`),
          'utf-8',
        ),
      );
      // Convert to enum
      if (config.userPermissions) {
        config.userPermissions = UserPermissions[config.userPermissions];
      }
    } catch (e) {
      throw new Error(`Unknown environment: ${environment}: ${e}`);
    }
  }
  return config as DeploymentConfig;
}
