import { SessionUser } from '@/types/next-auth';
import { Role } from '@prisma/client';

export function isAdmin(user: SessionUser | undefined) {
  return user?.role === Role.ADMIN;
}
