export interface ApiKeyRequest {
  comment: string;
}

export interface ApiKeyResponse {
  key: string;
  comment: string;
  createdAt: number;
  secret: string;
}

export interface ApiKeyResponseWithoutSecret {
  key: string;
  comment: string;
  createdAt: number;
}
