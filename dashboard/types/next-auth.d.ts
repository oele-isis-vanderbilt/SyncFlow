import NextAuth from 'next-auth';

declare module 'next-auth' {
  interface Session {
    user: SessionUser;
  }
}

export type SessionUser = {
  email: string;
  name: string;
  accessToken: string;
  refreshToken: string;
  accessTokenExpires: number;
};
