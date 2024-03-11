import NextAuth from 'next-auth';

import type { Role } from '@prisma/client';

declare module 'next-auth' {
  interface Session {
    user: SessionUser;
  }
}

export type SessionUser = {
  role: Role;
  email: string;
  name: string;
};
