import {
  CreateOptions,
  Room,
  RoomServiceClient,
  AccessToken,
} from 'livekit-server-sdk';
import type { AccessTokenOptions, VideoGrant } from 'livekit-server-sdk';

class LiveKitService {
  roomService: RoomServiceClient;

  constructor() {
    this.roomService = new RoomServiceClient(
      process.env.LIVEKIT_SERVER_URL,
      process.env.LIVEKIT_API_KEY,
      process.env.LIVEKIT_API_SECRET,
    );
  }

  async createRoom(options: CreateOptions): Promise<Room> {
    return await this.roomService.createRoom(options);
  }

  async deleteRoom(room: string): Promise<void> {
    return await this.roomService.deleteRoom(room);
  }

  async generateToken(
    identity: string,
    name: string,
    grant: VideoGrant,
  ): Promise<string> {
    const token = new AccessToken(
      process.env.LIVEKIT_API_KEY,
      process.env.LIVEKIT_API_SECRET,
      {
        identity,
      },
    );
    token.addGrant(grant);
    return token.toJwt();
  }

  async listRooms(): Promise<Room[]> {
    return (await this.roomService.listRooms()).sort(
      (r1, r2) => r1.creationTime - r2.creationTime,
    );
  }
}

export const liveKitService = new LiveKitService();
