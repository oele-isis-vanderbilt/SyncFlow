import FooterComp from '@/app/landing-page/Footer';
import Header from '@/app/landing-page/Header';

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex h-full w-full flex-col dark:bg-gray-800">
      <Header />
      <main className="flex flex-1 flex-col items-center justify-center border-b-2 dark:bg-gray-800">
        {children}
      </main>
      <FooterComp />
    </div>
  );
}
