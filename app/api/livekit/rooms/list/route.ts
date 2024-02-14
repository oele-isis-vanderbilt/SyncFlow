import { NextRequest, NextResponse } from 'next/server';
import { liveKitService } from '@/app/lib/livekit';

export async function GET(request: NextRequest, response: NextResponse) {
  const rooms = await liveKitService.listRooms();

  return new NextResponse(JSON.stringify(rooms), {
    headers: {
      'content-type': 'application/json',
    },
    status: 200,
  });
}
