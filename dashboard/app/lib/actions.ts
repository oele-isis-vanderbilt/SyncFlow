'use server';

import { z } from 'zod';
import { redirect } from 'next/navigation';
import { revalidatePath } from 'next/cache';
import { signIn, signOut as authSignOut } from '@/auth';

import { AuthError } from 'next-auth';
import { authClient, SignUpSchema } from './auth-client';

const APP_NAME = 'SyncFlow';
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

export type SignUpState = {
  errors?: string[];
  success: boolean;
};

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

export async function signOut() {
  await authSignOut();
}

export const providerLogin = async (provider: 'google' | 'github') => {
  await signIn(provider);
};

export async function redirectToDashboard() {
  revalidatePath('/dashboard');
  redirect('/dashboard');
}

export async function redirectTo(url: string) {
  revalidatePath(url);
  redirect(url);
}

export async function apiSignInWithGithub() {
  await signIn('github');
}

export async function signUp(
  prevState: SignUpState | undefined,
  formData: FormData,
): Promise<SignUpState> {
  const formDataObj = Object.fromEntries(formData.entries());
  const data = SignUpSchema.extend({
    confirmPassword: z.string().min(8),
  })
    .refine((data) => data.password === data.confirmPassword, {
      message: 'Passwords do not match',
      path: ['confirmPassword'],
    })
    .safeParse(formDataObj);

  if (!data.success) {
    return {
      success: false,
      errors: data.error.issues.map((issue) => issue.message),
    };
  }
  try {
    const signUpRequest = SignUpSchema.parse(data.data);
    const response = await authClient.signUp(signUpRequest);
    if (response.status === 200) {
      return { success: true };
    }
    const json = await response.json();
    return {
      success: false,
      errors: [json.message],
    };
  } catch (error) {
    return {
      success: false,
      errors: ['An error occurred while signing up. Please try again.'],
    };
  }
}
