import { jwtClient } from '../../lib/services';
import { NextRequest, NextResponse } from 'next/server';

export async function GET(req: NextRequest) {
  try {
    const roomName = req.nextUrl.searchParams.get('roomName');
    const identity = req.nextUrl.searchParams.get('identity');

    // Ensure both roomName and identity are provided and are strings
    if (typeof roomName !== 'string' || typeof identity !== 'string') {
      return new NextResponse('Bad Request', {
        status: 400,
      });
    }

    // Generate the LiveKit token
    const token = await jwtClient.generateLivekitToken(identity, roomName);

    // Prepare and return the result
    const result = {
      identity: identity,
      accessToken: token,
    };
    return new NextResponse(JSON.stringify(result, null, 2), {
      status: 200, // OK
      headers: {
        'Content-Type': 'application/json',
      },
    });
  } catch (e) {
    // Return the error response from the catch block
    return new NextResponse('Internal Server Error', {
      status: 500, // Internal Server Error
    });
  }
}
