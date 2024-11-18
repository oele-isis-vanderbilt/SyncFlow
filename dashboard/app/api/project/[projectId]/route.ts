import { projectClient } from '@/app/lib/project-client';

export async function GET(
  request: Request,
  { params }: { params: Promise<{ projectId: string }> },
) {
  const { projectId } = await params;
  const result = await projectClient.getProject(projectId);

  return result
    .map((p) => Response.json(p, { status: 200 }))
    .unwrapOrElse((err) => Response.json(err, { status: 500 }));
}
