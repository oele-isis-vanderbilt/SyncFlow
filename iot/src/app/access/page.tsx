"use client";

import Greet from "../greet"
import NavBar from "@/components/NavBar";
import { Store } from "tauri-plugin-store-api";
import { useEffect, useState } from "react";
import { Profile } from "@/types";
import { useRouter } from "next/navigation";

export default function Page() {
  const store = new Store(".settings.json");
  const [ profile, setProfile ]= useState<Profile>();
  const router = useRouter();

  useEffect(() => {

    const getProfile = async () => {
      const newProfile: Profile | null = await store.get("profile");
      console.log(newProfile)
      if (newProfile) {
        setProfile(newProfile);
      }
    }
    getProfile();

  }, [])

  // useEffect(() => {
  //   if (!profile) {
  //     router.push("/session/login");
  //   }
  // }, [profile])

  return (
    <div className="min-h-screen flex flex-col">
      <NavBar />
      <div className="w-full h-full flex flex-col justify-center items-center">
        <Greet msg={profile?.email}/>
      </div>
    </div>
  );
}
