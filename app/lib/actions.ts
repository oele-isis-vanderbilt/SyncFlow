'use server';

import { z } from 'zod';
import { liveKitService } from './livekit';
import { redirect } from 'next/navigation';
import { revalidatePath } from 'next/cache';
import { signIn } from '@/auth';

import type { CreateOptions, VideoGrant } from 'livekit-server-sdk';
import { AuthError } from 'next-auth';

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
  revalidatePath('/dashboard');
  redirect('/dashboard');
}

export async function deleteRoom(roomName: string) {
  await liveKitService.deleteRoom(roomName);
  revalidatePath('/dashboard');
  redirect('/dashboard');
}

// ToDo: Add a function to get all rooms
export async function generateToken(tokenOptions: VideoGrant = {}) {
  const grant = {
    ...{
      canPublish: true,
      canSubscribe: true,
      canUpdateOwnMetadata: true,
      roomJoin: true,
      roomCreate: false,
    },
    ...tokenOptions,
  };

  // ToDo: Add a function to get the user's identity after DB integration
  const token = await liveKitService.generateToken(USER_NAME, USER_NAME, grant);
  return token;
}

export async function authenticate(
  prevState: string | undefined,
  formData: FormData,
) {
  try {
    await signIn('credentials', formData);
  } catch (error) {
    if (error instanceof AuthError) {
      switch (error.type) {
        case 'CredentialsSignin':
          return 'Invalid credentials';
        default:
          return 'An error occurred';
      }
    }
    throw error;
  }
}

export async function redirectToDashboard() {
  revalidatePath('/dashboard');
  redirect('/dashboard');
}
