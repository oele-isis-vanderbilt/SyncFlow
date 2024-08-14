import Image from 'next/image';
import getConfig from '@/config';

export default function AppLogo({ w, h }: { w?: number; h?: number }) {
  const deploymentConfig = getConfig();
  return (
    <>
      <Image
        className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70] dark:invert"
        src={deploymentConfig.logoPath}
        alt="App Logo"
        width={w || 400}
        height={h || 400}
        priority
      />
    </>
  );
}
