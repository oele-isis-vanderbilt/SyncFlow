import Image from 'next/image';
import deploymentConfig from '@/deployment-config';

export default function AppLogo() {
  return (
    <>
      <Image
        className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70] dark:invert"
        src={deploymentConfig.logoPath}
        alt="App Logo"
        width={400}
        height={400}
        priority
      />
    </>
  );
}
