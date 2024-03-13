import { useState } from 'react';
import { Profile } from '@/types';
import { getProfile } from '@/utils/auth';

export default function NavBar() {
  const [profile, setProfile] = useState<Profile | null>(null);

  getProfile(setProfile);

  const goProfile = () => {
    console.log("go profile");
  }
  const logOut = () => {
    console.log("log out");
  }
  const logIn = () => {
    console.log("log in");
  }
  const signUp = () => {
    console.log("sign up");
  }

  return (
    <div className="navbar bg-base-100 border-b border-neutral">
      <div className="flex-none">
        <button className="btn btn-square btn-ghost">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" className="inline-block w-5 h-5 stroke-current"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
        </button>
      </div>
      <div className="flex-1">
        <a className="btn btn-ghost text-xl">LiveKitMMLA Client</a>
      </div>

      {/* <div className="flex-none">
        <button className="btn btn-square btn-ghost">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" className="inline-block w-5 h-5 stroke-current"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"></path></svg>
        </button>
      </div> */}
      <div className="dropdown dropdown-end">
        <div tabIndex={0} role="button" className="btn btn-ghost btn-circle border-neutral border avatar">
          <div className="relative w-10 h-10 overflow-hidden bg-gray-100 rounded-full dark:bg-gray-600">
            <svg className="absolute w-12 h-12 text-gray-400 -left-1" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z"></path></svg>
          </div>
        </div>
        <ul tabIndex={0} className="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow border-neutral bg-base-300 rounded-box w-52">
          {profile
            ? <>
              <li onClick={goProfile}><a>Profile</a></li>
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