import { Room, RoomServiceClient } from 'livekit-server-sdk';

class LiveKitService {
  roomService: RoomServiceClient;

  constructor() {
    this.roomService = new RoomServiceClient(
      process.env.LIVEKIT_SERVER_URL,
      process.env.LIVEKIT_API_KEY,
      process.env.LIVEKIT_API_SECRET,
    );
  }

  async createRoom(name: string): Promise<Room> {
    return await this.roomService.createRoom({ name });
  }

  async listRooms(): Promise<Room[]> {
    return await this.roomService.listRooms();
  }
}

export const liveKitService = new LiveKitService();
