"use client";

import Greet from "./greet"
import NavBar from "@/components/NavBar";
import { Store } from "tauri-plugin-store-api";
import { useEffect, useState } from "react";
import { Profile } from "@/types";
import { useRouter } from "next/navigation";
import { getProfile } from "@/utils/auth";

export default function Page() {
  const store = new Store(".settings.json");
  const [ profile, setProfile ]= useState<Profile>();
  const router = useRouter();

  getProfile(setProfile);

  useEffect(() => {
    if (profile) {
      router.push("/access");
    }
  }, [profile])

  return (
    <div className="min-h-screen flex flex-col justify-between">
      <NavBar />
      <Greet />
    </div>
  );
}
