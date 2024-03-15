import NextAuth from 'next-auth';
import Credentials from 'next-auth/providers/credentials';
import bcrypt from 'bcrypt';
import prisma from '@/app/lib/prisma';
import type { User, Role } from '@prisma/client';
import { authConfig } from './auth.config';
import { z } from 'zod';
import { PrismaAdapter } from '@auth/prisma-adapter';
import {jwtDecode} from "jwt-decode";

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
  session: { strategy: 'jwt' },
  callbacks: {
    async jwt({ token, user }) {
      if (user) {
        token.id = user.id;
        // @ts-ignore
        token.role = user.role;
      }
      return token;
    },
    async session({ session, token }) {
      session.user.role = token.role as Role;
      return session;
    },
  },
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
          const resp = await fetch("http://localhost:8081/users/login", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({
              username_or_email: email,
              password: password,
            }),
          });

          const data = await resp.json();
          console.log(jwtDecode(data.token));

          if (user) {
            const passwordMatch = await bcrypt.compare(password, user.password);

            if (passwordMatch) {
              return {
                name: user.username,
                email: user.email,
                role: user.role,
              };
            }
          }
        }
        return null;
      },
    }),
  ],
});
