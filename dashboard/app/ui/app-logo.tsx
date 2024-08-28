import Image from 'next/image';

export default function AppLogo({ w, h }: { w?: number; h?: number }) {
  return (
    <>
      <Image
        className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70] dark:invert"
        src={'/syncflow.png'}
        alt="App Logo"
        width={w || 400}
        height={h || 400}
        priority
      />
    </>
  );
}
