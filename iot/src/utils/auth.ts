import { useEffect } from "react";
import { Profile } from "@/types";
import { Store } from "tauri-plugin-store-api";

const store = new Store(".settings.json");

export function getProfile(setProfile: (profile: Profile) => void) {
  useEffect(() => {

    const getProfile = async () => {
      const newProfile: Profile | null = await store.get("profile");
      if (newProfile) {
        setProfile(newProfile);
      }
    }
    getProfile();

  }, [])
}

export async function loggingOut() {
  await store.clear()
}