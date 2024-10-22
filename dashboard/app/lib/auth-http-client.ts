import type { Result } from 'ts-monads/lib/Result';
import { Ok, Err } from 'ts-monads';
import { auth } from '@/auth';

export interface ClientError {
  message: any;
  code: number;
}

export class AuthHttpClient {
  private base_url: string;

  constructor(base_url: string) {
    this.base_url = base_url;
  }

  private async getAuthToken(): Promise<string | null> {
    // @ts-ignore
    return (await auth())?.accessToken;
  }

  async authenticatedGet<T>(url: string): Promise<Result<T, ClientError>> {
    const sessionToken = await this.getAuthToken();
    if (sessionToken === null) {
      return new Err({ message: 'Not authenticated', code: 401 });
    }
    const response = await fetch(this.base_url + url, {
      headers: {
        Authorization: `Bearer ${sessionToken}`,
        'Content-Type': 'application/json',
      },
    });

    if (response.ok) {
      const data = await response.json();
      return new Ok<T>(data);
    }
    try {
      const data = await response.json();
      // @ts-ignore
      return new Err<ClientError>({
        message: data,
        code: response.status,
      });
    } catch (e) {
      // @ts-ignore
      return new Err<ClientError>({
        message: 'Unknown error',
        code: response.status,
      });
    }
  }

  async authenticatedPost<T, U>(
    url: string,
    body: U,
  ): Promise<Result<T, ClientError>> {
    const sessionToken = await this.getAuthToken();
    if (sessionToken === null) {
      return new Err({ message: 'Not authenticated', code: 401 });
    }
    const response = await fetch(this.base_url + url, {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${sessionToken}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(body),
    });

    if (response.ok) {
      const data = await response.json();
      return new Ok<T>(data);
    }
    try {
      const data = await response.json();
      // @ts-ignore
      return new Err<ClientError>({
        message: data,
        code: response.status,
      });
    } catch (e) {
      // @ts-ignore
      return new Err<ClientError>({
        message: 'Unknown error',
        code: response.status,
      });
    }
  }

  async authenticatedDelete<T>(url: string): Promise<Result<T, ClientError>> {
    const sessionToken = await this.getAuthToken();
    if (sessionToken === null) {
      return new Err({ message: 'Not authenticated', code: 401 });
    }

    const response = await fetch(this.base_url + url, {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${sessionToken}`,
        'Content-Type': 'application/json',
      },
    });

    if (response.ok) {
      const data = await response.json();
      return new Ok<T>(data);
    }
    try {
      const data = await response.json();
      // @ts-ignore
      return new Err<ClientError>({
        message: data,
        code: response.status,
      });
    } catch (e) {
      // @ts-ignore
      return new Err<ClientError>({
        message: 'Unknown error',
        code: response.status,
      });
    }
  }
}
