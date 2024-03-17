import NextAuth from 'next-auth';
import Credentials from 'next-auth/providers/credentials';
import prisma from '@/app/lib/prisma';
import type { User, Role } from '@prisma/client';
import { authConfig } from './auth.config';
import { z } from 'zod';
import { PrismaAdapter } from '@auth/prisma-adapter';
import { jwtDecode } from 'jwt-decode';
import type { SessionUser } from '@/types/next-auth';
import deploymentConfig from '@/deployment-config';
import { Awaitable } from '@auth/core/src/types';

async function login(
  id: string,
  password: string,
): Promise<SessionUser | null> {
  let server_url = deploymentConfig.mmla_api_url;
  let credentials = {
    username_or_email: id,
    password: password,
  };

  let response = await fetch(server_url + '/users/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(credentials),
  });

  if (response.ok) {
    let data = await response.json();
    let token = data.token;
    let decoded_jwt = jwtDecode(token);
    return {
      id: decoded_jwt.user_name,
      name: decoded_jwt.user_name,
      email: decoded_jwt.user_name,
      role: decoded_jwt.role as Role,
      apiToken: token,
    } as SessionUser;
  }

  return null;
}

async function apiSignOut(token: string) {
  let server_url = deploymentConfig.mmla_api_url;
  let response = await fetch(server_url + '/users/logout', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
  });

  if (response.ok) {
    return null;
  }
  return null;
}

export const { auth, signIn, signOut } = NextAuth({
  ...authConfig,
  session: { strategy: 'jwt' },
  callbacks: {
    async jwt({ token, user }) {
      if (user) {
        return {
          ...token,
          id: user.id,
          role: user.role,
          jwt: user.apiToken,
        };
      }
      return token;
    },
    async session({ session, token }) {
      if (token) {
        session.jwt = token.jwt;
        session.user.id = token.id;
        session.user.role = token.role;
      }
      return session;
    },
  },
  events: {
    signOut: async ({ token, session }) => {
      await apiSignOut(token?.jwt as string);
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

          let user = await login(email, password);
          if (user) {
            return user;
          }
          return null;
        }
        return null;
      },
    }),
  ],
});
