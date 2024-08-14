'use client';
import Image from 'next/image';
import GoogleLogo from '@/app/ui/oauth-logos/google.svg';
import GitHubLogo from '@/app/ui/oauth-logos/github.svg';
import { Button } from './button';
import { providerLogin } from '../lib/actions';

export default function OAuthForm() {
  return (
    <div className="flex flex-1 flex-row justify-between rounded-lg bg-gray-700 py-2 dark:bg-gray-900">
      <Button className="bg-gray-700 hover:bg-gray-700 dark:bg-gray-900 dark:hover:bg-gray-900">
        <Image
          src={GoogleLogo}
          alt="Continue with Google"
          style={{ height: '2rem' }}
        />
        Google
      </Button>
      <Button
        className="bg-gray-700 text-black hover:bg-gray-700 dark:bg-gray-900 dark:hover:bg-gray-900"
        onClick={() => providerLogin('github')}
      >
        <Image
          src={GitHubLogo}
          alt="Continue with GitHub"
          style={{ height: '2rem' }}
          className="mr-2 bg-white"
        />
        GitHub
      </Button>
    </div>
  );
}
