import { projectClient } from '@/app/lib/project-client';

export async function GET(
  request: Request,
  { params }: { params: Promise<{ projectId: string; sessionId: string }> },
) {
  const { projectId, sessionId } = await params;
  const result = await projectClient.getSession(projectId, sessionId);
  return result
    .map((p) => Response.json(p, { status: 200 }))
    .unwrapOrElse((err) => Response.json(err, { status: 500 }));
}
