'use client';

import { lusitana } from '@/app/ui/fonts';
import {
  AtSymbolIcon,
  KeyIcon,
  ExclamationCircleIcon,
} from '@heroicons/react/24/outline';
import { ArrowRightIcon } from '@heroicons/react/20/solid';
import { Button } from './button';

import { useFormState, useFormStatus } from 'react-dom';
import { authenticate } from '@/app/lib/actions';
import Link from 'next/link';

export default function LoginForm() {
  const [errorMessage, dispatch] = useFormState(authenticate, undefined);
  return (
    <form action={dispatch} className="space-y-3">
      <div className="flex-1 rounded-lg border-2 px-6 pt-8 pb-4 shadow dark:bg-gray-900">
        <h1 className={`${lusitana.className} mb-3 text-2xl dark:text-white`}>
          Please log in to continue.
        </h1>
        <div className="w-full">
          <div>
            <label
              className="mt-5 mb-3 block font-medium text-xs dark:text-white"
              htmlFor="email"
            >
              Email
            </label>
            <div className="relative">
              <input
                className="peer block w-full rounded-md border border-gray-200 py-[9px] pl-10 text-gray-900 text-sm outline-2 placeholder:text-gray-500"
                id="email"
                type="email"
                name="email"
                placeholder="Enter your email address"
                required
              />
              <AtSymbolIcon className="-translate-y-1/2 pointer-events-none absolute top-1/2 left-3 h-[18px] w-[18px] text-gray-500 peer-focus:text-gray-900" />
            </div>
          </div>
          <div className="mt-4">
            <label
              className="mt-5 mb-3 block font-medium text-xs dark:text-white"
              htmlFor="password"
            >
              Password
            </label>
            <div className="relative">
              <input
                className="peer block w-full rounded-md border border-gray-200 py-[9px] pl-10 text-gray-900 text-sm outline-2 placeholder:text-gray-500"
                id="password"
                type="password"
                name="password"
                placeholder="Enter password"
                required
                minLength={3}
              />
              <KeyIcon className="-translate-y-1/2 pointer-events-none absolute top-1/2 left-3 h-[18px] w-[18px] text-gray-500 peer-focus:text-gray-900" />
            </div>
          </div>
        </div>
        <LoginButton />
        <div
          className="flex h-8 items-end space-x-1"
          aria-live="polite"
          aria-atomic="true"
        >
          {errorMessage && (
            <>
              <ExclamationCircleIcon className="h-5 w-5 text-red-500" />
              <p className="text-red-500 text-xs">{errorMessage}</p>
            </>
          )}
        </div>
        <div className="text-sm dark:text-white">
          Don&apos;t have an account?{' '}
          <Link href="/signup" className="underline hover:text-blue-500">
            Sign Up
          </Link>
          !
        </div>
      </div>
    </form>
  );
}

function LoginButton() {
  const { pending } = useFormStatus();
  return (
    <Button className="mt-4 w-full bg-teal-900" aria-disabled={pending}>
      Log in <ArrowRightIcon className="ml-auto h-5 w-5 text-gray-50" />
    </Button>
  );
}
