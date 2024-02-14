import type { RoomCompositeOptions, VideoGrant } from 'livekit-server-sdk';
import {
  AccessToken,
  CreateOptions,
  EgressClient,
  EncodedFileType,
  Room,
  RoomServiceClient,
  SegmentedFileOutput,
} from 'livekit-server-sdk';
import { unstable_noStore as noStore } from 'next/cache';

class LiveKitService {
  roomService: RoomServiceClient;
  egressClient: EgressClient;

  constructor() {
    this.roomService = new RoomServiceClient(
      process.env.LIVEKIT_SERVER_URL!,
      process.env.LIVEKIT_API_KEY!,
      process.env.LIVEKIT_API_SECRET!,
    );

    this.egressClient = new EgressClient(
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

  async beginRoomCompositeEgress(roomName: string) {
    noStore();
    const result = await this.egressClient.startRoomCompositeEgress(roomName, {
      fileType: EncodedFileType.MP4,
      filepath: `livekit-demo/room-${roomName}-test.mp4`,
      s3: {
        accessKey: process.env.S3_ACCESS_KEY!,
        secret: process.env.S3_SECRET_KEY!,
        bucket: process.env.S3_BUCKET_NAME!,
        endpoint: process.env.S3_ENDPOINT!,
        region: process.env.S3_REGION!,
      },
    });

    return result;
  }

  async stopRoomCompositeEgress(egressId: string) {
    noStore();
    await this.egressClient.stopEgress(egressId);
  }

  async getRoomEgresses(roomName: string) {
    noStore();
    return await this.egressClient.listEgress(roomName);
  }
}

export const liveKitService = new LiveKitService();
