import { AuthHttpClient } from './auth-http-client';
import type {
  ApiKeyRequest,
  ApiKeyResponse,
  ApiKeyResponseWithoutSecret,
} from '@/types/api';
import getConfig from '@/config';
const PREFIXES = {
  CREATE_API_KEY: '/users/api-key',
  LIST_API_KEYS: '/users/api-keys',
  DELETE_API_KEY: '/users/api-key',
};

class ApiClient extends AuthHttpClient {
  async listApiKeys() {
    return await this.authenticatedGet<ApiKeyResponseWithoutSecret[]>(
      PREFIXES.LIST_API_KEYS,
    );
  }

  async deleteApiKey(key: string) {
    return await this.authenticatedDelete<ApiKeyResponseWithoutSecret>(
      `${PREFIXES.DELETE_API_KEY}/${key}`,
    );
  }

  async createApiKey(req: ApiKeyRequest) {
    return await this.authenticatedPost<ApiKeyResponse, ApiKeyRequest>(
      PREFIXES.CREATE_API_KEY,
      req,
    );
  }
}

const deploymentConfig = getConfig();

export const apiClient = new ApiClient(deploymentConfig.syncFlowApiUrl);
