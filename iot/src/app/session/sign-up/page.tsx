"use client";

import React, { useState } from 'react';
import Link from 'next/link';
import NavBar from '@/components/NavBar';

export default function SignUp() {
  const [email, setEmail] = useState<string>('');
  const [password, setPassword] = useState<string>('');
  const [successMsg, setSuccessMsg] = useState<string>("");
  const [error, setError] = useState<string>("");

  async function signUpWithEmail(e: any) {
    e.preventDefault()
    
  }

  return (
    <div>
      <NavBar />
      <div className="flex justify-center items-center h-screen">
        <div className="p-6 max-w-sm w-full rounded-2xl border border-neutral">
          <h2 className="mb-4 text-xl font-bold">Sign Up</h2>
            <form>
              <div className="mb-4">
                <label htmlFor="email" className="mb-2 text-sm">Email</label>
                <input type="email" id="email" className="border border-neutral bg-base-100 text-sm rounded-2xl block w-full p-2.5" placeholder="name@example.com" required onChange={(e) => setEmail(e.target.value)}/>
              </div>
              <div className="mb-4">
                <label htmlFor="password" className="block mb-2 text-sm font-medium">Password</label>
                <input type="password" id="password" className="border border-neutral bg-base-100 text-sm rounded-2xl block w-full p-2.5" required onChange={(e) => setPassword(e.target.value)}/>
              </div>
              <button className="w-full btn btn-primary" onClick={signUpWithEmail}>Submit</button>
            </form>
            {error && <div className="mt-4 text-error">{error}</div>}
            {successMsg && <div className="mt-4 text-success">{successMsg}</div>}
            <div className="flex justify-between items-center mt-4">
              <Link href="login">
                <button className="text-sm btn btn-ghost text-primary hover:underline">Already have an account? Sign In</button>
              </Link>
            </div>
        </div>
      </div>
    </div>
  );
}