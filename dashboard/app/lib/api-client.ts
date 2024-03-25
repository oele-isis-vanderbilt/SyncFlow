import { AuthHttpClient } from './auth-http-client';
import {
  ApiKeyRequest,
  ApiKeyResponse,
  ApiKeyResponseWithoutSecret,
} from '@/types/api';
import deploymentConfig from '@/deployment-config';
const PREFIXES = {
  CREATE_API_KEY: '/users/api-key',
  LIST_API_KEYS: '/users/api-keys',
  DELETE_API_KEY: '/users/api-key',
};

class ApiClient extends AuthHttpClient {
  constructor(base_url: string) {
    super(base_url);
  }

  async listApiKeys() {
    return await this.authenticatedGet<ApiKeyResponseWithoutSecret[]>(
      PREFIXES.LIST_API_KEYS,
    );
  }

  async deleteApiKey(key: string) {
    return await this.authenticatedDelete<ApiKeyResponseWithoutSecret>(
      PREFIXES.DELETE_API_KEY + `/${key}`,
    );
  }

  async createApiKey(req: ApiKeyRequest) {
    return await this.authenticatedPost<ApiKeyResponse, ApiKeyRequest>(
      PREFIXES.CREATE_API_KEY,
      req,
    );
  }
}

export const apiClient = new ApiClient(deploymentConfig.mmla_api_url);
