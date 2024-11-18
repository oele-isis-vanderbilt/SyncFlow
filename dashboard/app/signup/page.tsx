import SignUpForm from '@/app/ui/signup-form';
import OAuthForm from '@/app/ui/oauth-form';
import Header from '@/app/landing-page/Header';
import FooterComp from '@/app/landing-page/Footer';

export default function LoginPage() {
  return (
    <div className="flex h-screen w-full flex-col dark:bg-gray-800">
      <Header />
      <main className="flex h-full w-full flex-1 flex-col items-center justify-center border-b-2 dark:bg-gray-800">
        <div className="relative mx-auto flex flex-col space-y-2.5 p-4">
          <SignUpForm />
          <div className="relative my-4">
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
