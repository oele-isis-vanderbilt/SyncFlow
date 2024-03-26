import { JWTClient } from '@/app/lib/jwt-client';

const apiKey = process.env.LIVEKIT_MMLA_API_KEY!;
const secret = process.env.LIVEKIT_MMLA_API_SECRET!;
const baseUrl = process.env.LIVEKIT_MMLA_API_BASE_URL!;
export const jwtClient = new JWTClient(apiKey, secret, baseUrl);
