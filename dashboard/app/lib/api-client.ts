import { AuthHttpClient } from "./auth-http-client";
const PREFIXES = {
    CREATE_API_KEYS: '/users/api-key',
    LIST_API_KEYS: '/users/api-keys',
};

class ApiClient extends AuthHttpClient {
    constructor(base_url: string) {
        super(base_url);
    }

    async createApiKey(options) {
        return await this.authenticatedPost(PREFIXES.CREATE_API_KEYS, options);
    }

    async listApiKeys() {
        return await this.authenticatedGet(PREFIXES.LIST_API_KEYS);
    }
  
}