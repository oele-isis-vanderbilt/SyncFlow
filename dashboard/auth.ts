import NextAuth from 'next-auth';
import Credentials from 'next-auth/providers/credentials';
import { authConfig } from './auth.config';
import { z } from 'zod';
import { jwtDecode } from 'jwt-decode';
import type { SessionUser } from '@/types/next-auth';
import deploymentConfig from '@/deployment-config';

async function apiSignIn(
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
      id: decoded_jwt.userName,
      name: decoded_jwt.userName,
      email: decoded_jwt.userName,
      role: decoded_jwt.role,
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
  trustHost: true,
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
        session.user ? (session.user.id = token.id) : null;
        session.user ? (session.user.role = token.role) : null;
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

          let user = await apiSignIn(email, password);
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
