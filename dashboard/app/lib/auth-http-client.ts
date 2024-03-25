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
    return (await auth())?.jwt;
  }

  async authenticatedGet<T>(url: string): Promise<Result<T, ClientError>> {
    const sessionToken = await this.getAuthToken();
    if (sessionToken === null) {
      return new Err({ message: 'Not authenticated', code: 401 });
    } else {
      let response = await fetch(this.base_url + url, {
        headers: {
          Authorization: `Bearer ${sessionToken}`,
          'Content-Type': 'application/json',
        },
      });

      if (response.ok) {
        let data = await response.json();
        return new Ok<T>(data);
      } else {
        try {
          let data = await response.json();
          return new Err<ClientError>({
            message: data,
            code: response.status,
          });
        } catch (e) {
          return new Err<ClientError>({
            message: 'Unknown error',
            code: response.status,
          });
        }
      }
    }
  }

  async authenticatedPost<T, U>(
    url: string,
    body: U,
  ): Promise<Result<T, ClientError>> {
    const sessionToken = await this.getAuthToken();
    if (sessionToken === null) {
      return new Err({ message: 'Not authenticated', code: 401 });
    } else {
      let response = await fetch(this.base_url + url, {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${sessionToken}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(body),
      });

      if (response.ok) {
        let data = await response.json();
        return new Ok<T>(data);
      } else {
        try {
          let data = await response.json();
          return new Err<ClientError>({
            message: data,
            code: response.status,
          });
        } catch (e) {
          return new Err<ClientError>({
            message: 'Unknown error',
            code: response.status,
          });
        }
      }
    }
  }

  async authenticatedDelete<T>(url: string): Promise<Result<T, ClientError>> {
    const sessionToken = await this.getAuthToken();
    if (sessionToken === null) {
      return new Err({ message: 'Not authenticated', code: 401 });
    } else {
      let response = await fetch(this.base_url + url, {
        method: 'DELETE',
        headers: {
          Authorization: `Bearer ${sessionToken}`,
          'Content-Type': 'application/json',
        },
      });

      if (response.ok) {
        let data = await response.json();
        return new Ok<T>(data);
      } else {
        try {
          let data = await response.json();
          return new Err<ClientError>({
            message: data,
            code: response.status,
          });
        } catch (e) {
          return new Err<ClientError>({
            message: 'Unknown error',
            code: response.status,
          });
        }
      }
    }
  }
}
