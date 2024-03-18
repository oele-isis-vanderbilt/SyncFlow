import { SessionUser } from '@/types/next-auth';
import type { Role } from '@/types/next-auth';

export function isAdmin(user: SessionUser | undefined) {
  return user?.role === 'ADMIN';
}
