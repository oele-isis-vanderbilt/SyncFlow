import { lusitana } from '@/app/ui/fonts';

export function NoPermission() {
  return (
    <div className={'flex h-full w-full flex-col'}>
      <h1 className={`${lusitana.className} mb-4 text-xl md:text-2xl`}>
        You do not have permission to view this page
      </h1>
    </div>
  );
}
