import type { Metadata } from 'next';
import { Inter } from 'next/font/google';
import './globals.css';
import '@livekit/components-styles';

const inter = Inter({ subsets: ['latin'] });

export const metadata: Metadata = {
  title: 'SyncFlow Demo Client',
  description: 'A demo client for SyncFlow',
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={`${inter.className} bg-[#111]`}>{children}</body>
    </html>
  );
}
