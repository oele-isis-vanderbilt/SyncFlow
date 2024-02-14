import type { VideoGrant } from 'livekit-server-sdk';
import {
  AccessToken,
  CreateOptions,
  EgressClient,
  Room,
  RoomServiceClient,
} from 'livekit-server-sdk';
import { unstable_noStore as noStore } from 'next/cache';

class LiveKitService {
  roomService: RoomServiceClient;

  constructor() {
    this.roomService = new RoomServiceClient(
      process.env.LIVEKIT_SERVER_URL!,
      process.env.LIVEKIT_API_KEY!,
      process.env.LIVEKIT_API_SECRET!,
    );
  }

  async createRoom(options: CreateOptions): Promise<Room> {
    noStore();
    return await this.roomService.createRoom(options);
  }

  async deleteRoom(room: string): Promise<void> {
    noStore();
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
    noStore();
    return (await this.roomService.listRooms()).sort(
      (r1, r2) => r1.creationTime - r2.creationTime,
    );
  }
}

export const liveKitService = new LiveKitService();
