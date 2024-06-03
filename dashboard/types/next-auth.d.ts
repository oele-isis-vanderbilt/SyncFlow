import NextAuth from 'next-auth';

export type Role = 'ADMIN' | 'USER';

declare module 'next-auth' {
  interface Session {
    user: SessionUser;
  }
}

export type SessionUser = {
  role: Role;
  email: string;
  name: string;
  accessToken: string;
  refreshToken: string;
  accessTokenExpires: number;
};
