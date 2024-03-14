"use client";

import React from 'react';
import NavBar from '@/components/NavBar';
import Link from "next/link"

const UpdatePassword = () => {
  return (
    <div>
      <NavBar/>
      <div className="flex justify-center items-center h-screen">
        <div className="p-6 max-w-sm w-full rounded-2xl border border-neutral overflow-hidden">
          <h2 className="mb-4 text-xl font-bold">Update Password</h2>
            <form>
              <div className="mb-4">
                <label htmlFor="password" className="block mb-2 text-sm">New Password</label>
                <input type="password" id="password" className=" border border-neutral overflow-hidden bg-base-100 text-sm rounded-2xl block w-full p-2.5" required />
              </div>
              <button type="submit" className="w-full btn btn-primary">Update Password</button>
            </form>

            <div className="flex justify-between items-center mt-4">
              <Link href="login">
                <button className="text-sm btn btn-ghost text-primary hover:underline">Remember? Sign In</button>
              </Link>
            </div>
        </div>
      </div>
   </div>
  );
};

export default UpdatePassword;