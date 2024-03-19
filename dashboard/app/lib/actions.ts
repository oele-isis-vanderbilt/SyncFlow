'use server';

import { z } from 'zod';
import { redirect } from 'next/navigation';
import { revalidatePath } from 'next/cache';
import { signIn } from '@/auth';
import { mmlaClient } from '@/app/lib/mmla-client';

import type { CreateRoomRequest } from '@/types/mmla';
import { AuthError } from 'next-auth';
import { EgressInfo } from '@livekit/protocol';
import { JsonValue } from '@bufbuild/protobuf';

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
  const options: CreateRoomRequest = {
    name: randomRoomName(),
    options: {
      emptyTimeout: 60 * 10,
      maxParticipants: 10,
      metadata: 'LiveKit ELP Room',
    },
  };
  await mmlaClient.createRoom(options);
  revalidatePath('/dashboard');
  redirect('/dashboard');
}

export async function deleteRoom(roomName: string) {
  await mmlaClient.deleteRoom(roomName);
  revalidatePath('/dashboard');
  redirect('/dashboard');
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

export async function redirectToRoomRecording(roomName: string) {
  revalidatePath(`/dashboard/recordings/${roomName}`);
  redirect(`/dashboard/recordings/${roomName}`);
}

export async function beginTrackEgress(roomName: string, trackId: string) {
  const egressResult = await mmlaClient.recordTrack(roomName, trackId);
  if (egressResult.ok()) {
    return egressResult.unwrap().toJson() as JsonValue;
  } else {
    revalidatePath(`/dashboard/recordings/${roomName}`);
    redirect(`/dashboard/recordings/${roomName}`);
  }
}

export async function stopEgress(roomName: string, egressId: string) {
  let egressResult = await mmlaClient.stopEgress(roomName, egressId);
  if (egressResult.ok()) {
    return egressResult.unwrap().toJson() as JsonValue;
  } else {
    revalidatePath(`/dashboard/recordings/${roomName}`);
    redirect(`/dashboard/recordings/${roomName}`);
  }
}

export async function beginTracksEgress(trackIds: string[], roomName: string) {
  const egresses = await Promise.all(
    trackIds.map((trackId) => {
      return mmlaClient.recordTrack(roomName, trackId);
    }),
  );

  revalidatePath(`/dashboard/recordings/${roomName}`);
  redirect(`/dashboard/recordings/${roomName}`);
}

export async function stopTracksEgress(egressIds: string[], roomName: string) {
  const egresses = await Promise.all(
    egressIds.map((egressId) => {
      return mmlaClient.stopEgress(roomName, egressId);
    }),
  );

  await new Promise((resolve) => setTimeout(resolve, 1000));

  revalidatePath(`/dashboard/recordings/${roomName}`);
  redirect(`/dashboard/recordings/${roomName}`);
}
