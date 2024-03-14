import { useState } from 'react';
import { Profile } from '@/types';
import { getProfile, loggingOut } from '@/utils/auth';
import { useRouter } from 'next/navigation';

export default function NavBar() {
  const [profile, setProfile] = useState<Profile | null>(null);
  const router = useRouter()

  getProfile(setProfile);

  const logOut = async () => {
    await loggingOut()
    router.push("/")
  }
  const logIn = () => {
    router.push("/session/login")
  }
  const signUp = () => {
    router.push("/session/sign-up")
  }

  const iconButton = () => {
    router.push("/")
  }

  return (
    <div className="navbar bg-base-100 border-b border-neutral">
      {profile &&
        <div className="flex-none">
          <button className="btn btn-square btn-ghost">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" className="inline-block w-5 h-5 stroke-current"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
          </button>
        </div>
      }
      <div className="flex-1">
        <button className="btn btn-ghost text-xl" onClick={iconButton}>LiveKitMMLA Client</button>
      </div>

      <div className="dropdown dropdown-end">
        <div tabIndex={0} role="button" className="btn btn-ghost btn-circle border-neutral border avatar">
          <div className="relative w-10 h-10 overflow-hidden bg-gray-100 rounded-full dark:bg-gray-600">
            <svg className="absolute w-12 h-12 text-gray-400 -left-1" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z"></path></svg>
          </div>
        </div>
        <ul tabIndex={0} className="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow border-neutral bg-base-300 rounded-box w-52">
          {profile
            ? <>
              <li onClick={logOut}><a>Logout</a></li>
            </>
            : <>
              <li onClick={logIn}><a>Log In</a></li>
              <li onClick={signUp}><a>Sign Up</a></li>
            </>
          }
        </ul>
      </div>
    </div>
  )
}