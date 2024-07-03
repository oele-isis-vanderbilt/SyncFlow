import NextAuth from 'next-auth';
import Credentials from 'next-auth/providers/credentials';
import { authConfig } from './auth.config';
import { z } from 'zod';
import getConfig from '@/config';
import { redirect } from 'next/navigation';
import Github from 'next-auth/providers/github';
import { AuthClient } from './app/lib/auth-client';

const deploymentConfig = getConfig();
const authClient = new AuthClient(deploymentConfig.mmla_api_url);

export const { handlers, auth, signOut, signIn } = NextAuth({
  ...authConfig,
  trustHost: true,
  session: { strategy: 'jwt' },
  callbacks: {
    async jwt({ token, user, account }) {
      if (token && user && account && account.provider === 'github') {
        let apiUser = await authClient.loginWithGithub(token, user, account);

        if (apiUser) {
          return {
            ...token,
            id: apiUser.id,
            role: apiUser.role,
            accessToken: apiUser.accessToken,
            refreshToken: apiUser.refreshToken,
            accessTokenExpires: apiUser.accessTokenExpires,
          };
        }
        return null;
      }
      if (user) {
        return {
          ...token,
          id: user.id,
          role: user.role,
          accessToken: user.accessToken,
          refreshToken: user.refreshToken,
          accessTokenExpires: user.accessTokenExpires,
        };
      }

      if (Date.now() < token.accessTokenExpires) {
        return token;
      }

      const refreshedTokens = await refreshAccessToken(token);
      if (refreshedTokens === null) {
        redirect('/login');
      }
    },
    async session({ session, token }) {
      if (token) {
        session.jwt = token.jwt;
        session.accessToken = token.accessToken;
        session.refreshToken = token.refreshToken;
        session.accessTokenExpires = token.accessTokenExpires;
        session.user ? (session.user.id = token.id) : null;
        session.user ? (session.user.role = token.role) : null;
        session.user ? (session.user.userName = token.userName) : null;
      }

      return session;
    },
  },
  events: {
    signOut: async ({ token, session }) => {
      await authClient.signOut(token?.accessToken as string);
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

          let user = await authClient.apiSignIn(email, password);
          if (user) {
            return user;
          }
          return null;
        }
        return null;
      },
    }),
    Github({
      clientId: process.env.GITHUB_CLIENT_ID,
      clientSecret: process.env.GITHUB_CLIENT_SECRET,
      profile(profile) {
        return {
          id: profile.id,
          name: profile.name,
          email: profile.email,
          image: profile.avatar_url,
          login: profile.login,
        };
      },
    }),
  ],
});
