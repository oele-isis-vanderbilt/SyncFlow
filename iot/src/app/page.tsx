"use client";

import NavBar from "@/components/NavBar";

export default function Page() {

  return (
    <div className="min-h-screen flex flex-col">
      <NavBar />
      <div className="h-full w-full flex flex-col justify-center items-center">
        <p>
          Welcome to LiveKit-MMLA
        </p>
      </div>
    </div>
  );
}
