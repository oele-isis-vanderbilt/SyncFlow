import {
  AccessToken,
  CreateOptions,
  DirectFileOutput,
  EgressClient,
  EncodedFileType,
  Room,
  RoomServiceClient,
  VideoGrant,
} from 'livekit-server-sdk';
import { unstable_noStore as noStore } from 'next/cache';

class LiveKitService {
  roomService: RoomServiceClient;
  egressService: EgressClient;

  constructor() {
    this.roomService = new RoomServiceClient(
      process.env.LIVEKIT_SERVER_URL!,
      process.env.LIVEKIT_API_KEY!,
      process.env.LIVEKIT_API_SECRET!,
    );
    this.egressService = new EgressClient(
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

  async startRoomCompositeEgress(roomName: string) {
    noStore();
    const egressInfo = await this.egressService.startRoomCompositeEgress(
      roomName,
      {
        fileType: EncodedFileType.MP4,
        filepath: `roomComposite/${roomName}.mp4`,
        s3: {
          bucket: process.env.S3_BUCKET_NAME!,
          region: process.env.S3_REGION!,
          accessKey: process.env.S3_ACCESS_KEY!,
          secret: process.env.S3_SECRET_KEY!,
          endpoint: process.env.S3_ENDPOINT!,
        },
      },
    );
    console.log('Egress started', egressInfo);
    return egressInfo;
  }

  async stopEgress(egressId: string) {
    noStore();
    return await this.egressService.stopEgress(egressId);
  }

  async getRoomEgresses(roomName: string) {
    noStore();
    return await this.egressService.listEgress({ roomName });
  }

  async listParticipants(roomName: string) {
    noStore();
    return await this.roomService.listParticipants(roomName);
  }

  async startTrackEgress(
    roomName: string,
    output: DirectFileOutput | string,
    trackId: string,
  ) {
    noStore();
    return await this.egressService.startTrackEgress(roomName, output, trackId);
  }
}

export const liveKitService = new LiveKitService();
