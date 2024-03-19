import { auth } from '@/auth';
import deploymentConfig from '@/deployment-config';
import { Ok, Err } from 'ts-monads';
import type { Result } from 'ts-monads/lib/Result';
import {
  EgressInfo,
  ParticipantInfo,
  Room,
  VideoGrant,
} from 'livekit-server-sdk';
import { CreateRoomRequest, TokenResponse } from '@/types/mmla';

interface MMLAClientError {
  message: any;
  code: number;
}

const PREFIXES = {
  LIST_ROOMS: '/livekit/list-rooms',
  GENERATE_TOKEN: '/livekit/token',
  CREATE_ROOM: '/livekit/create-room',
  DELETE_ROOM: '/livekit/delete-room',
  LIST_PARTICIPANTS: '/livekit/list-participants',
  LIST_EGRESSES: '/livekit/list-egresses',
  RECORD_ROOM: '/livekit/record-room',
};

export class MMLAClient {
  private base_url: string;

  constructor(base_url: string) {
    this.base_url = base_url;
  }

  private async getAuthToken(): Promise<string | null> {
    return (await auth())?.jwt;
  }

  private async authenticatedGet<T>(
    url: string,
  ): Promise<Result<T, MMLAClientError>> {
    const sessionToken = await this.getAuthToken();
    if (sessionToken === null) {
      return new Err({ message: 'Not authenticated', code: 401 });
    } else {
      let response = await fetch(this.base_url + url, {
        headers: {
          Authorization: `Bearer ${sessionToken}`,
          'Content-Type': 'application/json',
        },
      });

      if (response.ok) {
        let data = await response.json();
        return new Ok<T>(data);
      } else {
        try {
          let data = await response.json();
          return new Err<MMLAClientError>({
            message: data,
            code: response.status,
          });
        } catch (e) {
          return new Err<MMLAClientError>({
            message: 'Unknown error',
            code: response.status,
          });
        }
      }
    }
  }

  private async authenticatedPost<T, U>(
    url: string,
    body: U,
  ): Promise<Result<T, MMLAClientError>> {
    const sessionToken = await this.getAuthToken();
    if (sessionToken === null) {
      return new Err({ message: 'Not authenticated', code: 401 });
    } else {
      let response = await fetch(this.base_url + url, {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${sessionToken}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(body),
      });

      if (response.ok) {
        let data = await response.json();
        return new Ok<T>(data);
      } else {
        try {
          let data = await response.json();
          return new Err<MMLAClientError>({
            message: data,
            code: response.status,
          });
        } catch (e) {
          return new Err<MMLAClientError>({
            message: 'Unknown error',
            code: response.status,
          });
        }
      }
    }
  }

  private async authenticatedDelete<T>(
    url: string,
  ): Promise<Result<T, MMLAClientError>> {
    const sessionToken = await this.getAuthToken();
    if (sessionToken === null) {
      return new Err({ message: 'Not authenticated', code: 401 });
    } else {
      let response = await fetch(this.base_url + url, {
        method: 'DELETE',
        headers: {
          Authorization: `Bearer ${sessionToken}`,
          'Content-Type': 'application/json',
        },
      });

      if (response.ok) {
        let data = await response.json();
        return new Ok<T>(data);
      } else {
        try {
          let data = await response.json();
          return new Err<MMLAClientError>({
            message: data,
            code: response.status,
          });
        } catch (e) {
          return new Err<MMLAClientError>({
            message: 'Unknown error',
            code: response.status,
          });
        }
      }
    }
  }

  async createRoom(options: CreateRoomRequest) {
    return await this.authenticatedPost<Room, CreateRoomRequest>(
      PREFIXES.CREATE_ROOM,
      options,
    );
  }

  async deleteRoom(room: string) {
    return await this.authenticatedDelete<void>(
      PREFIXES.DELETE_ROOM + '/' + room,
    );
  }

  async listRooms() {
    return await this.authenticatedGet<Room[]>(PREFIXES.LIST_ROOMS);
  }

  async generateToken(req: { identity: string; videoGrants: VideoGrant }) {
    return await this.authenticatedPost<
      TokenResponse,
      { identity: string; videoGrants: VideoGrant }
    >(PREFIXES.GENERATE_TOKEN, req);
  }

  async listParticipants(roomName: string) {
    let response = await this.authenticatedGet<any[]>(
      PREFIXES.LIST_PARTICIPANTS + '/' + roomName,
    );
    let parsed = response.map((data) =>
      data.map((p: any) => ParticipantInfo.fromJSON(p)),
    );
    return parsed;
  }

  async listEgresses(roomName: string) {
    let response = await this.authenticatedGet<any[]>(
      PREFIXES.LIST_EGRESSES + '/' + roomName,
    );

    let parsed = response.map((data) =>
      data.map((p: any) => EgressInfo.fromJSON(p)),
    );

    return parsed;
  }

  async recordRoom(roomName: string) {
    let response = await this.authenticatedPost(
      PREFIXES.RECORD_ROOM + '/' + roomName,
      {},
    );
  }
}

export const mmlaClient = new MMLAClient(deploymentConfig.mmla_api_url);
