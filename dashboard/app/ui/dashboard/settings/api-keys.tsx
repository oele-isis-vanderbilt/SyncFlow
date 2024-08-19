import { lusitana } from '@/app/ui/fonts';
import CreateApiKeys from '@/app/ui/dashboard/settings/create-api-key';
import { auth } from '@/auth';

export default async function ApiKeys() {
  let session = await auth();
  return (
    <div className="mt-8 flex items-center dark:text-white">
      <div>
        <h1 className={`${lusitana.className} mb-4 mt-4 text-xl md:text-2xl`}>
          Api Keys
        </h1>
      </div>
      <CreateApiKeys />
    </div>
  );
}
