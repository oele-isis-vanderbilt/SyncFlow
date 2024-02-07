import { WebhookReceiver } from 'livekit-server-sdk';
import { NextRequest } from 'next/server'
import { revalidatePath } from "next/cache";

const receiver = new WebhookReceiver(
    process.env.LIVEKIT_API_KEY!,
    process.env.LIVEKIT_API_SECRET!
);

export async function POST(request: NextRequest) {
  const bodyString = await request.text();
  const event = receiver.receive(bodyString, request.headers.get('Authorization')!);

  if (['room_started', 'room_finished'].includes(event.event!)) {
    revalidatePath('/dashboard');
  }

  return new Response('ok');
}

export async function GET(request: NextRequest) {
  const responseBody = {
    status: 'ok',
    message: 'LiveKit Webhook Receiver',
  };

    return new Response(JSON.stringify(responseBody), {
        headers: {
        'content-type': 'application/json',
        },
        status: 200,
    });
}
