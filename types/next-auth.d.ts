import NextAuth from 'next-auth';

import type { User, Role } from '@prisma/client';

declare module 'next-auth' {
    interface Session {
        user: {
            role: Role;
            email: string;
            name: string;
        };
    }
}
