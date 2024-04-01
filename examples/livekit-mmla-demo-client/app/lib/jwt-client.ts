import * as jwt from 'jsonwebtoken';

export interface Claims {
  iat: number;
  exp: number;
  iss: string;

  project?: string;
}

export class JWTClient {
  apiKey: string;
  secret: string;
  baseUrl: string;
  token: string | undefined;

  constructor(apiKey: string, secret: string, baseUrl: string) {
    this.apiKey = apiKey;
    this.secret = secret;
    this.baseUrl = baseUrl;
  }

  async getAccessToken(): Promise<string> {
    if (this.token && !this.hasExpired(this.token)) {
      return this.token;
    }

    const claims: Claims = {
      iat: Math.floor(Date.now() / 1000),
      exp: Math.floor(Date.now() / 1000) + 3600,
      iss: this.apiKey,
      project: 'client-demo',
    };

    this.token = jwt.sign(claims, this.secret, {
      algorithm: 'HS256',
    });
    return this.token as string;
  }

  async listRooms() {
    const url = `${this.baseUrl}/livekit/list-rooms`;
    const token = await this.getAccessToken();

    let response = await fetch(url, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${token}`,
      },
    });
    try {
      return await response.json();
    } catch (e) {
      return [];
    }
  }

  async generateLivekitToken(identity: string, roomName: string) {
    let baseUrl = `${this.baseUrl}/livekit/token`;
    let token = await this.getAccessToken();

    let requestData = {
      identity: identity,
      videoGrants: {
        room: roomName,
        canPublish: true,
        canSubscribe: false,
        canPublishSources: [],
        canPublishData: true,
        canUpdateOwnMetadata: false,
        hidden: false,
        ingressAdmin: false,
        recorder: false,
        roomAdmin: false,
        roomCreate: false,
        roomJoin: true,
        roomList: true,
        roomRecord: false,
      },
    };

    let response = await fetch(baseUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(requestData),
    });
    try {
      let json = await response.json();
      return json.token;
    } catch (e) {
      return '';
    }
  }

  hasExpired(token: string): boolean {
    const claims = jwt.decode(token) as Claims;
    return claims.exp < Math.floor(Date.now() / 1000);
  }
}
