import ELPLogo from '@/app/ui/elp-logo';
import LoginForm from '@/app/ui/login-form';

export default function LoginPage() {
  return (
    <main className="flex items-center justify-center md:h-screen">
      <div className="relative mx-auto flex w-full max-w-[400px] flex-col space-y-2.5 p-4 md:-mt-32">
        <div className="flex h-20 w-full items-center rounded-lg p-3 md:h-36">
          <ELPLogo />
        </div>
        <LoginForm />
      </div>
    </main>
  );
}
