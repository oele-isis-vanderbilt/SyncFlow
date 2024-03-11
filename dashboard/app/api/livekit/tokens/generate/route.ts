import { NextRequest, NextResponse } from 'next/server';
import { liveKitService } from '@/app/lib/livekit';
import type { VideoGrant } from 'livekit-server-sdk';

export async function POST(request: NextRequest) {
  const requestBody = await request.json();
  const identity = requestBody.identity;
  const name = requestBody.name;
  const tokenOptions = (requestBody.options as VideoGrant) || {};

  const token = await liveKitService.generateToken(
    identity,
    name,
    tokenOptions,
  );
  return new NextResponse(JSON.stringify({ token }), {
    headers: {
      'content-type': 'application/json',
    },
    status: 200,
  });
}
