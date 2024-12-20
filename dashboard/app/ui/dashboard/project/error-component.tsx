export default function ErrorComponent({
  title,
  error,
  projectId,
}: {
  title: string;
  error: any;
  projectId: string;
}) {
  return (
    <div className="mt-2 min-h-20 bg-gray-200 p-4 dark:bg-indigo-950 dark:text-white">
      <span>{title}</span>
      <br />
      <div className="mt-2 min-h-20 overflow-auto bg-gray-200 p-4 dark:bg-indigo-950">
        <pre>
          {JSON.stringify(
            {
              projectId: projectId,
              error: error,
            },
            null,
            2,
          )}
        </pre>
      </div>
    </div>
  );
}
