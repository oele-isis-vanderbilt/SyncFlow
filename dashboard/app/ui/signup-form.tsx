'use client';

import { useFormState, useFormStatus } from 'react-dom';
import { signUp, SignUpState } from '@/app/lib/actions';
import { lusitana } from '@/app/ui/fonts';
import {
  ArrowRightIcon,
  AtSymbolIcon,
  ExclamationCircleIcon,
  IdentificationIcon,
  KeyIcon,
} from '@heroicons/react/24/outline';
import { Button } from './button';
import Link from 'next/link';
import { Input as SignUpInput } from '@/app/ui/input';

export default function SignupForm() {
  const [messages, dispatch] = useFormState<SignUpState | undefined>(
    signUp,
    undefined,
  );

  if (messages && messages.success) {
    return (
      <div className="relative  mx-auto flex w-full flex-col p-4 md:w-1/3">
        <div className="relative text-xl dark:text-white">
          Signup successful! Please{' '}
          <Link href={'/login'} className="underline hover:text-blue-500">
            sign in
          </Link>{' '}
          to continue.
        </div>
      </div>
    );
  }

  return (
    <form action={dispatch} className="space-y-3">
      <div className="flex-1 rounded-lg border-2 px-6 pb-4 pt-8 shadow dark:bg-gray-900">
        <h1 className={`${lusitana.className} mb-3 text-2xl dark:text-white`}>
          Register to continue!
        </h1>
        <div className="w-full">
          <div className="w-full gap-2 md:flex">
            <div className="md:flex-1">
              <SignUpInput
                id="firstName"
                label="First Name"
                type="text"
                placeholder="Jane"
                required={false}
              >
                <IdentificationIcon className="pointer-events-none absolute left-3 top-1/2 h-[18px] w-[18px] -translate-y-1/2 text-gray-500 peer-focus:text-gray-900" />
              </SignUpInput>
            </div>
            <SignUpInput
              id="middleName"
              label="Middle Name"
              type="text"
              placeholder="M"
              required={false}
            >
              <IdentificationIcon className="pointer-events-none absolute left-3 top-1/2 h-[18px] w-[18px] -translate-y-1/2 text-gray-500 peer-focus:text-gray-900" />
            </SignUpInput>
          </div>

          <SignUpInput
            id="lastName"
            label="Last Name"
            type="text"
            placeholder="Doe"
            required={false}
          >
            <IdentificationIcon className="pointer-events-none absolute left-3 top-1/2 h-[18px] w-[18px] -translate-y-1/2 text-gray-500 peer-focus:text-gray-900" />
          </SignUpInput>
          <SignUpInput
            id="username"
            label="Username"
            type="text"
            placeholder="username"
            required={true}
          >
            <IdentificationIcon className="pointer-events-none absolute left-3 top-1/2 h-[18px] w-[18px] -translate-y-1/2 text-gray-500 peer-focus:text-gray-900" />
          </SignUpInput>
          <SignUpInput
            id="email"
            label="Email"
            type="text"
            placeholder="Doe"
            required={true}
          >
            <AtSymbolIcon className="pointer-events-none absolute left-3 top-1/2 h-[18px] w-[18px] -translate-y-1/2 text-gray-500 peer-focus:text-gray-900" />
          </SignUpInput>
          <SignUpInput
            id="password"
            label="Password"
            type="password"
            placeholder=""
            required={true}
          >
            <KeyIcon className="pointer-events-none absolute left-3 top-1/2 h-[18px] w-[18px] -translate-y-1/2 text-gray-500 peer-focus:text-gray-900" />
          </SignUpInput>
          <SignUpInput
            id="confirmPassword"
            label="Confirm Password"
            type="password"
            placeholder=""
            required={true}
          >
            <KeyIcon className="pointer-events-none absolute left-3 top-1/2 h-[18px] w-[18px] -translate-y-1/2 text-gray-500 peer-focus:text-gray-900" />
          </SignUpInput>
        </div>
        <SignUpButton />
        <div aria-live="polite" aria-atomic="true">
          {messages &&
            !messages.success &&
            messages.errors?.length &&
            messages.errors?.length > 0 &&
            messages.errors?.map((message, index) => (
              <p
                key={index}
                className="flex items-center gap-2 p-2 text-xs text-red-500"
              >
                <ExclamationCircleIcon className="h-5 w-5 text-red-500" />
                {message}
              </p>
            ))}
        </div>
        <div className="p-2 text-sm dark:text-white">
          Have an account?{' '}
          <Link href="/login" className="underline hover:text-blue-500">
            Sign In
          </Link>
          !
        </div>
      </div>
    </form>
  );
}

function SignUpButton() {
  const { pending } = useFormStatus();
  return (
    <Button className="mt-4 w-full bg-teal-900" aria-disabled={pending}>
      Register <ArrowRightIcon className="ml-auto h-5 w-5 text-gray-50" />
    </Button>
  );
}
