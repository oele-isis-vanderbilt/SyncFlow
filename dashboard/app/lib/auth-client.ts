import type { Account, User } from 'next-auth';
import type { JWT } from 'next-auth/jwt';
import type { SessionUser } from '@/types/next-auth';
import { jwtDecode } from 'jwt-decode';
import { z } from 'zod';
import getConfig from '@/config';

export const SignUpSchema = z.object({
  username: z.string(),
  email: z.string().email({
    message: 'Invalid email address',
  }),
  password: z.string().min(4),
  firstName: z.string().optional(),
  middleName: z.string().optional(),
  lastName: z.string().optional(),
  organization: z.string().optional(),
  jobRole: z.string().optional(),
});

export type SignUpRequest = z.infer<typeof SignUpSchema>;
export class AuthClient {
  auth_url: string;

  constructor(auth_url: string) {
    this.auth_url = auth_url;
  }

  async loginWithGithub(token: JWT, user: User, account: Account) {
    const serverUrl = this.auth_url;
    const githubToken = account.access_token;

    const payload = {
      email: user.email,
      avatar_url: user.image,
      login: user.login,
    };

    const response = await fetch(`${serverUrl}/oauth/github/login`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${githubToken}`,
      },
      body: JSON.stringify(payload),
    });

    if (response.ok) {
      const data = await response.json();
      const token = data.accessToken;
      const decoded_jwt = jwtDecode(token);
      return {
        id: decoded_jwt.userName,
        name: decoded_jwt.userName,
        email: decoded_jwt.email,
        role: decoded_jwt.role,
        accessToken: token,
        refreshToken: data.refreshToken,
        accessTokenExpires: decoded_jwt.exp * 1000,
      } as SessionUser;
    }
    return null;
  }

  async apiSignIn(id: string, password: string): Promise<SessionUser | null> {
    const server_url = this.auth_url;
    const credentials = {
      username_or_email: id,
      password: password,
    };

    const response = await fetch(`${server_url}/users/login`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(credentials),
    });

    if (response.ok) {
      const data = await response.json();
      const token = data.accessToken;
      const decoded_jwt = jwtDecode(token);
      return {
        id: decoded_jwt.userName,
        name: decoded_jwt.userName,
        email: decoded_jwt.email,
        role: decoded_jwt.role,
        accessToken: token,
        refreshToken: data.refreshToken,
        accessTokenExpires: decoded_jwt.exp * 1000,
      } as SessionUser;
    }

    return null;
  }

  async refreshAccessToken(token) {
    const serverUrl = this.auth_url;

    const refreshToken = token.refreshToken;

    const response = await fetch(`${serverUrl}/users/refresh-token`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ refresh_token: refreshToken }),
    });

    if (response.ok) {
      const data = await response.json();
      const decoded_jwt = jwtDecode(data.accessToken);
      const refreshedTokens = {
        ...token,
        jwt: data.accessToken,
        id: decoded_jwt.userName,
        name: decoded_jwt.userName,
        email: decoded_jwt.userName,
        role: decoded_jwt.role,
        accessToken: data.accessToken,
        refreshToken: data.refreshToken,
        accessTokenExpires: decoded_jwt.exp * 1000,
      };
      return refreshedTokens;
    }
    return null;
  }

  async signOut(token: string) {
    const serverUrl = this.auth_url;
    const response = await fetch(`${serverUrl}/users/logout`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${token}`,
      },
    });

    if (response.ok) {
      return null;
    }
    return null;
  }

  async signUp(userDetails: SignUpRequest): Promise<Response> {
    const server_url = this.auth_url;
    const response = await fetch(`${server_url}/users/signup`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(userDetails),
    });

    if (response.status >= 500) {
      throw new Error('Internal Server error, please try again later');
    }

    return response;
  }
}

const deploymentConfig = getConfig();
export const authClient = new AuthClient(deploymentConfig.syncFlowApiUrl);
