'use server';

import { z } from 'zod';
import { liveKitService } from './livekit';
import { redirect } from 'next/navigation';

import type { CreateOptions, VideoGrant } from 'livekit-server-sdk';

const APP_NAME = 'LiveKitELP';
const USER_NAME = 'admin';

const RoomSchema = z.object({
  roomName: z.string(),
});

const CreateRoom = RoomSchema;

export type State = {
  errors?: {
    roomName?: string;
  };
  message?: string | null;
};

const randomRoomName = () => {
  return APP_NAME + '_' + Math.random().toString(36).substring(7);
};

export async function createRoom() {
  const options: CreateOptions = {
    name: randomRoomName(),
    emptyTimeout: 60 * 10,
    maxParticipants: 10,
    metadata: 'LiveKit ELP Room',
  };
  const room = await liveKitService.createRoom(options);
  redirect('/dashboard');
}

export async function deleteRoom(roomName: string) {
  await liveKitService.deleteRoom(roomName);
  redirect('/dashboard');
}

export async function generateToken(roomName: string) {
  const token = await liveKitService.generateToken(USER_NAME, USER_NAME, {
    canPublish: true,
    canSubscribe: false,
    canUpdateOwnMetadata: true,
    roomJoin: true,
    roomCreate: false,
  } as VideoGrant);
  return token;
}
