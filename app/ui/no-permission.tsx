import { lusitana } from '@/app/ui/fonts';

export function NoPermission() {
  return (
    <div className={'flex h-full w-full flex-col items-center justify-center'}>
      <h1 className={`${lusitana.className} mb-4 text-xl md:text-4xl`}>
        You do not have permission to view this page.
      </h1>
    </div>
  );
}
