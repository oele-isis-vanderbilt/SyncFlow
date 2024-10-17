import SyncFlowLogo from '@/app/ui/app-logo';
import LoginForm from '@/app/ui/login-form';
import OAuthForm from '@/app/ui/oauth-form';
import { signIn } from '@/auth';
import FooterComp from '../landing-page/Footer';
import Header from '../landing-page/Header';

export default function LoginPage() {
  return (
    <div className="flex h-full w-full flex-col dark:bg-gray-800">
      <Header />
      <main className="flex flex-1 flex-col items-center justify-center border-b-2 dark:bg-gray-800">
        <div className="md:-mt-32 relative mx-auto flex flex-col space-y-2.5 p-4">
          <div className="flex h-20 w-full items-center rounded-lg p-3 md:h-36">
            <SyncFlowLogo />
          </div>
          <LoginForm />
          <div className="relative my-4">
            <div className="absolute inset-0 flex items-center">
              <div className="w-full border-gray-300 border-t"></div>
            </div>
            <div className="relative flex justify-center text-sm">
              <span className="bg-white px-2 text-black dark:bg-gray-900 dark:text-white">
                OR Continue With
              </span>
            </div>
          </div>
          <OAuthForm />
        </div>
      </main>
      <FooterComp />
    </div>
  );
}
