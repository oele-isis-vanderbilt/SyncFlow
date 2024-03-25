import { lusitana } from '@/app/ui/fonts';
import ApiKeys from '@/app/ui/dashboard/settings/api-keys';
import ApiKeysTable from '@/app/ui/dashboard/settings/api-keys-table';

export default function Page() {
  return (
    <main>
      <h1 className={`${lusitana.className} mb-4 text-xl md:text-2xl`}>
        Settings
      </h1>
      <ApiKeys />
      <div className="mt-8 flex items-center">
        <ApiKeysTable />
      </div>
    </main>
  );
}
