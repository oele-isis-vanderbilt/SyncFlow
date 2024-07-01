import SyncFlowLogo from '@/app/ui/app-logo';
import LoginForm from '@/app/ui/login-form';
import OAuthForm from '@/app/ui/oauth-form';
import { signIn } from '@/auth';

export default function LoginPage() {
  return (
    <main className="flex items-center justify-center md:h-full">
      <div className="relative mx-auto flex w-full max-w-[400px] flex-col space-y-2.5 p-4 md:-mt-32">
        <div className="flex h-20 w-full items-center rounded-lg p-3 md:h-36">
          <SyncFlowLogo />
        </div>
        <LoginForm />
        <div className="relative my-4">
          <div className="absolute inset-0 flex items-center">
            <div className="w-full border-t border-gray-300"></div>
          </div>
          <div className="relative flex justify-center text-sm">
            <span className="bg-gray-900 px-2 text-white">
              OR Continue With
            </span>
          </div>
        </div>
        <OAuthForm />
      </div>
    </main>
  );
}
