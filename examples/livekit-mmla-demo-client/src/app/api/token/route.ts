/// The token endpoint for livekit server

import { jwtClient } from '@/app/lib/services';
import { NextRequest, NextResponse } from 'next/server';

export async function GET(req: NextRequest) {
  try {
    const roomName = req.nextUrl.searchParams.get('roomName');
    const identity = req.nextUrl.searchParams.get('identity');

    if (typeof roomName !== 'string' || typeof identity !== 'string') {
      return new NextResponse('Bad Request', {
        status: 403,
      });
    }

    const token = await jwtClient.generateLivekitToken(identity, roomName);

    const result = {
      identity: identity,
      accessToken: token,
    };
    return new NextResponse(JSON.stringify(result, null, 2), {
      status: 200,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  } catch (e) {
    new NextResponse('Internal Server Error', {
      status: 500,
    });
  }
}
