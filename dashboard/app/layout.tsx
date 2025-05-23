import type { Metadata } from 'next';
import './globals.css';
import { inter } from '@/app/ui/fonts';
import { ThemeModeScript } from 'flowbite-react';

export const metadata: Metadata = {
  title: {
    template: '%s | SyncFlow',
    default: 'SyncFlow',
  },
  description: 'SyncFlow Dashboard',
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html
      lang="en"
      className="scrollbar-thub-sky-700 scrollbar-track-sky-300 scrollbar-thin"
    >
      <head>
        <ThemeModeScript />
      </head>
      <body className={`${inter.className} dark:bg-gray-800`}>{children}</body>
    </html>
  );
}
