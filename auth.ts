import NextAuth from 'next-auth';
import Credentials from 'next-auth/providers/credentials';
import bcrypt from 'bcrypt';
import prisma from '@/app/lib/prisma';
import type { User } from '@prisma/client';
import { authConfig } from './auth.config';
import { z } from 'zod';
import { PrismaAdapter } from '@auth/prisma-adapter';

async function getUser(email: string): Promise<User> {
  try {
    return await prisma.user.findUniqueOrThrow({
      where: {
        email: email,
      },
    });
  } catch (error) {
    console.error('Failed to fetch user', error);
    throw new Error('Failed to fetch user');
  }
}

export const { auth, signIn, signOut } = NextAuth({
  ...authConfig,
  adapter: PrismaAdapter(prisma),
  session: {strategy: 'jwt'},
  providers: [
    Credentials({
      async authorize(credentials) {
        const parsedCredentials = z
          .object({
            email: z.string().email(),
            password: z.string(),
          })
          .safeParse(credentials);

        if (parsedCredentials.success) {
          const { email, password } = parsedCredentials.data;
          const user = await getUser(email);
          if (user) {
            const passwordMatch = await bcrypt.compare(password, user.password);

            if (passwordMatch) {
              console.log(user);
              return user;
            }
          }
        }
        return null;
      },
    }),
  ],
});
