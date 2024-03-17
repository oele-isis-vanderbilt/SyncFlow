'use server';

import { z } from 'zod';
import { liveKitService } from './livekit';
import { redirect } from 'next/navigation';
import { revalidatePath } from 'next/cache';
import { signIn } from '@/auth';
import { mmlaClient } from '@/app/lib/mmlaClient';

import type { CreateOptions, EgressInfo, VideoGrant } from 'livekit-server-sdk';
import { AuthError } from 'next-auth';
import { Egress } from 'livekit-server-sdk/dist/proto/livekit_egress';

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
  await mmlaClient.createRoom(options);
  revalidatePath('/dashboard');
  redirect('/dashboard');
}

export async function deleteRoom(roomName: string) {
  await mmlaClient.deleteRoom(roomName);
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

export async function getRoomRecordings(roomName: string) {
  try {
    return await liveKitService.getRoomEgresses(roomName);
  } catch (error) {
    console.error('Error getting room recordings', error);
    return [];
  }
}

export async function redirectToRoomRecording(roomName: string) {
  revalidatePath(`/dashboard/recordings/${roomName}`);
  redirect(`/dashboard/recordings/${roomName}`);
}

export async function beginTrackEgress(roomName: string, trackId: string) {
  try {
    const egressInfo = await liveKitService.startTrackEgress(
      roomName,
      {
        filepath:
          'tracks/{room_name}/{publisher_identity}/{track_type}-{track_source}-{track_id}-{time}',
        s3: {
          bucket: process.env.S3_BUCKET_NAME!,
          region: process.env.S3_REGION!,
          accessKey: process.env.S3_ACCESS_KEY!,
          secret: process.env.S3_SECRET_KEY!,
          endpoint: process.env.S3_ENDPOINT!,
        },
      },
      trackId,
    );
    return egressInfo;
  } catch (error) {
    console.error('Error beginning track egress', error);
    revalidatePath(`/dashboard/recordings/${roomName}`);
    redirect(`/dashboard/recordings/${roomName}`);
  }
}

export async function stopEgress(roomName: string, egressId: string) {
  try {
    const egressInfo = await liveKitService.stopEgress(egressId);
    return egressInfo;
  } catch (error) {
    console.error('Error stopping track egress', error);
    revalidatePath(`/dashboard/recordings/${roomName}`);
    redirect(`/dashboard/recordings/${roomName}`);
  }
}

export async function beginTracksEgress(trackIds: string[], roomName: string) {
  const egresses = await Promise.all(
    trackIds.map((trackId) => {
      return beginTrackEgress(roomName, trackId);
    }),
  );

  revalidatePath(`/dashboard/recordings/${roomName}`);
  redirect(`/dashboard/recordings/${roomName}`);
}

export async function stopTracksEgress(egressIds: string[], roomName: string) {
  const egresses = await Promise.all(
    egressIds.map(liveKitService.stopEgress.bind(liveKitService)),
  );

  await new Promise((resolve) => setTimeout(resolve, 1000));

  revalidatePath(`/dashboard/recordings/${roomName}`);
  redirect(`/dashboard/recordings/${roomName}`);
}
