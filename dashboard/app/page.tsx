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
    <main className="relative">
      <Header />
      <section className="border-b-2">
        <Hero />
      </section>
      <section className="border-b-2">
        <Features />
      </section>
      <section className="border-b-2">
        <About />
      </section>
      <FooterComp />
    </main>
  );
}
