import AppLogo from '@/app/ui/app-logo';
import { ArrowRightIcon } from '@heroicons/react/24/outline';
import Link from 'next/link';
import { lusitana } from '@/app/ui/fonts';
import getConfig from '@/config';
import Nav from '@/app/ui/nav';
import Hero from './landing-page/Hero';
import Features from './landing-page/Features';
import Header from './landing-page/Header';
import About from './landing-page/About';
import FooterComp from './landing-page/Footer';

export default function Page() {
  const deploymentConfig = getConfig();
  return (
    <div className="relative flex w-full flex-col">
      <Header />
      <main className="min-w-0 flex-1 divide-y dark:divide-gray-700">
        <Hero />
        <Features />
        <About />
      </main>
      <FooterComp />
    </div>
  );
}
