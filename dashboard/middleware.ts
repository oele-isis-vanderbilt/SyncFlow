import NextAuth from 'next-auth';
import { authConfig } from './auth.config';
import { NextRequest, NextResponse } from 'next/server';

const auth = NextAuth(authConfig).auth;

export default async function middleware(req: NextRequest, res: NextResponse) {
  return auth(req, res);
}

export const config = {
  // https://nextjs.org/docs/app/building-your-application/routing/middleware#matcher
  matcher: [
    '/api/:path*',
    '/((?!_next/static|_next/image|.*\\.png$|.*\\.svg$).*)',
  ],
};
