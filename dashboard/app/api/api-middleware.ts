// Verify API Bearer Token

// Path: app/api/api-middleware.ts

import { NextRequest, NextResponse } from 'next/server';

export default async function middleware(req: NextRequest, res: NextResponse) {
  if (
    req.nextUrl.pathname.startsWith('/api') &&
    !req.nextUrl.pathname.includes('/webhooks')
  ) {
    const token = req.headers.get('Authorization')?.replace('Bearer ', '');
    if (token !== process.env.API_BEARER_TOKEN) {
      return new NextResponse(
        JSON.stringify({
          error: 'Unauthorized',
        }),
        { status: 401 },
      );
    }
    return NextResponse.next();
  }
}
