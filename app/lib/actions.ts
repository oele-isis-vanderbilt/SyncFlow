'use server';

import { z } from 'zod';
import { liveKitService } from './livekit';
import { redirect } from 'next/navigation';

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

export async function createRoom(prevState: State, formData: FormData) {
  const validatedFields = CreateRoom.safeParse(formData);
  // console.log('validatedFields', validatedFields.error.flatten().fieldErrors);

  if (!validatedFields.success) {
    return {
      errors: validatedFields.error.flatten().fieldErrors,
      message: 'Missing Name. Failed to create room',
    };
  }

  const { roomName } = validatedFields.data;

  try {
    const room = await liveKitService.createRoom(roomName);
    console.log('room', room);
  } catch (err) {
    return {
      message: 'Failed to create room',
    };
  }

  redirect('/');
}
