'use-client';
import Image from 'next/image';
import GoogleLogo from '@/app/ui/oauth-logos/google.svg';
import GitHubLogo from '@/app/ui/oauth-logos/github.svg';
import { Button } from './button';

export default function OAuthForm() {
    return (
        <div className="flex-1 flex flex-row justify-center gap-10 rounded-lg py-2 bg-gray-900">
            <Button className='bg-gray-900 hover:bg-gray-900'>
                <Image
                    src={GoogleLogo}
                    alt="Continue with Google"
                    style={{ height: "2rem" }}
                />
                Google
            </Button>
            <Button className='bg-gray-900 hover:bg-gray-900 text-black'>
                <Image
                    src={GitHubLogo}
                    alt="Continue with GitHub"
                    style={{ height: "2rem" }}
                    className='bg-white mr-2'
                />
                GitHub
            </Button>
        </div>
    );
    
}