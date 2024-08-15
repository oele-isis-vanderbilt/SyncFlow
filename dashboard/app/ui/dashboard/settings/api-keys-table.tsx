import { apiClient } from '@/app/lib/api-client';
import { ApiKeyResponseWithoutSecret } from '@/types/api';
import ApiKeyActions from '@/app/ui/dashboard/settings/api-key-actions';

export default async function ApiKeysTable() {
  let allApiKeys: ApiKeyResponseWithoutSecret[] = [];
  let apiKeysResult = await apiClient.listApiKeys();
  apiKeysResult
    .map((apiKeys) => {
      allApiKeys = apiKeys;
    })
    .mapError((error) => {
      allApiKeys = [];
    });

  return (
    <>
      {allApiKeys.length === 0 ? (
        <p className="dark:text-white">No Api Keys</p>
      ) : (
        <table className="w-full text-left text-sm rtl:text-right">
          <thead className="text-gray bg-gray-100 text-xs uppercase dark:bg-gray-900 dark:text-white ">
            <tr>
              <th scope="col" className="px-6 py-3">
                Key
              </th>
              <th scope="col" className="px-6 py-3">
                Description
              </th>
              <th scope="col" className="px-6 py-3">
                Created At
              </th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {allApiKeys.map((apiKey, index: number) => {
              return (
                <tr
                  key={index}
                  className="border-5 border-indigo-200 bg-gray-200 dark:bg-black dark:text-white"
                >
                  <td className="whitespace-nowrap px-6 py-4">
                    <div className="text-blue text-sm hover:text-blue-400 hover:underline">
                      {apiKey.key.trim()}
                    </div>
                  </td>
                  <td className="whitespace-nowrap px-6 py-4">
                    <div className="text-blue text-sm hover:text-blue-400 hover:underline">
                      {apiKey.comment.trim()}
                    </div>
                  </td>
                  <td className="whitespace-nowrap px-6 py-4">
                    <div className="text-blue text-sm hover:text-blue-400 hover:underline">
                      {new Date(apiKey.createdAt * 1000).toISOString()}
                    </div>
                  </td>
                  <td className="whitespace-nowrap px-6 py-4">
                    <ApiKeyActions apiKey={apiKey.key} />
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      )}
    </>
  );
}
