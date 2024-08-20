import SignUpForm from '@/app/ui/signup-form';
import OAuthForm from '@/app/ui/oauth-form';

export default function LoginPage() {
  return (
    <div className="relative  mx-auto flex w-full flex-col space-y-2.5 p-4 md:w-1/3">
      <SignUpForm />
      <div className="relative my-4">
        <div className="absolute inset-0 flex items-center">
          <div className="w-full border-t border-gray-300"></div>
        </div>
        <div className="relative flex justify-center text-sm">
          <span className="bg-white px-2 text-black dark:bg-gray-900 dark:text-white">
            OR Continue With
          </span>
        </div>
      </div>
      <OAuthForm />
    </div>
  );
}
