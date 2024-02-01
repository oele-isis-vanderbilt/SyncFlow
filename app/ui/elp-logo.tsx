import { GlobeAltIcon } from '@heroicons/react/24/outline';
import { lusitana } from '@/app/ui/fonts';
import Image from 'next/image';

export default function ElpLogo() {
  return (
    <>
      <Image
        className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70] dark:invert"
        src="/livekit-elp.svg"
        alt="LiveKit ELP Logo"
        width={400}
        height={400}
        priority
      />
    </>
  );
}
