"use client";

import Link from "next/link";
import { useState } from "react";
import { useRouter } from "next/navigation";
import NavBar from "@/components/NavBar";

const Page = () => {
  const [email, setEmail] = useState<string>('');
  const [password, setPassword] = useState<string>('');
  const [error, setError] = useState<string>('');
  const router = useRouter();

  function validateInput() {
    if (email === '' || password === '' || password.length < 8) {
      setError("Please fill out all fields.");
      return false;
    }
    return true;
  }

  async function signInWithEmail(e: any) {
    e.preventDefault();

    if (!validateInput()) {
      return;
    }
  }

  return (
    <div>
      <NavBar />
      <div className="flex justify-center items-center h-screen">
        <div className="p-6 max-w-sm w-full rounded-2xl border border-neutral overflow-hidden">
          <h2 className="mb-4 text-xl font-bold">Login</h2>
            <form>
              <div className="mb-4">
                <label htmlFor="email" className="mb-2 text-sm">Email</label>
                <input type="email" id="email" className="border border-neutral text-sm rounded-2xl overflow-hidden w-full p-2.5 bg-base-100" placeholder="name@example.com" required onChange={(e) => setEmail(e.target.value)}/>
              </div>
              <div className="mb-4">
                <label htmlFor="password" className="mb-2 text-sm">Password</label>
                <input type="password" id="password" className="border border-neutral text-sm rounded-2xl overflow-hidden w-full p-2.5 bg-base-100" required onChange={(e) => setPassword(e.target.value)}/>
              </div>
              <button type="button" className="w-full btn btn-primary " onClick={signInWithEmail}>Login</button>
            </form>
            {error && <div className="mt-4 text-error">{error}</div>}
            <div className="flex justify-between items-center mt-4">
            <Link href="sign-up">
              <button className="text-sm btn btn-ghost text-primary hover:underline">Sign Up</button>
            </Link>
            <Link href="forgot-password">
              <button className="text-sm btn btn-ghost text-primary hover:underline">Forgot Password?</button>
            </Link>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Page;