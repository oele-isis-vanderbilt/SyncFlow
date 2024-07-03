import { auth } from '@/auth';
import getConfig from '@/config';
import { Ok, Err } from 'ts-monads';
import type { Result } from 'ts-monads/lib/Result';
import { EgressInfo, ParticipantInfo, Room } from '@livekit/protocol';

import { VideoGrant } from 'livekit-server-sdk';
import { CreateRoomRequest, TokenResponse } from '@/types/mmla';
import { AuthHttpClient } from './auth-http-client';

const PREFIXES = {
  LIST_ROOMS: '/livekit/list-rooms',
  GENERATE_TOKEN: '/livekit/token',
  CREATE_ROOM: '/livekit/create-room',
  DELETE_ROOM: '/livekit/delete-room',
  LIST_PARTICIPANTS: '/livekit/list-participants',
  LIST_EGRESSES: '/livekit/list-egresses',
  BEGIN_TRACK_EGRESS: '/livekit/begin-track-egress',
  STOP_EGRESS: '/livekit/stop-recording',
};

export class MMLAClient extends AuthHttpClient {
  constructor(base_url: string) {
    super(base_url);
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
      data.map((p: any) => ParticipantInfo.fromJson(p)),
    );
    return parsed;
  }

  async listEgresses(roomName: string) {
    let response = await this.authenticatedGet<any[]>(
      PREFIXES.LIST_EGRESSES + '/' + roomName,
    );

    let parsed = response.map((data) =>
      data.map((p: any) => EgressInfo.fromJson(p)),
    );

    return parsed;
  }

  async recordTrack(roomName: string, trackSid: string) {
    let responseResult = await this.authenticatedPost<any, {}>(
      PREFIXES.BEGIN_TRACK_EGRESS + '/' + roomName + '/' + trackSid,
      {},
    );

    let egressInfo = responseResult.map((data) => EgressInfo.fromJson(data));

    return egressInfo;
  }

  async stopEgress(roomName: string, egressId: string) {
    let responseResult = await this.authenticatedPost<any, {}>(
      PREFIXES.STOP_EGRESS + '/' + roomName + '/' + egressId,
      {},
    );

    let egressInfo = responseResult.map((data) => EgressInfo.fromJson(data));

    return egressInfo;
  }
}

const deploymentConfig = getConfig();
export const mmlaClient = new MMLAClient(deploymentConfig.mmla_api_url);
