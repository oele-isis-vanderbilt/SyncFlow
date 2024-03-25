export interface ApiKeyRequest {
    comment: string;
}

export interface ApiKeyResponse {
    key: string;
    comment: string;
    createAt: number;
}