export interface ApiKeyRequest {
  comment: string;
}

export interface ApiKeyResponse {
  id: number;
  key: string;
  comment: string;
  createdAt: number;
  secret: string;
  projectId?: string;
}

export interface ApiKeyResponseWithoutSecret {
  key: string;
  comment: string;
  createdAt: number;
  projectId?: string;
}
