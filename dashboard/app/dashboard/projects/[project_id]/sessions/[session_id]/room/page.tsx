import { projectClient } from '@/app/lib/project-client';
import ErrorComponent from '@/app/ui/dashboard/project/error-component';
import Room from '@/app/ui/dashboard/rooms/room';
import { auth } from '@/auth';
import { VideoGrant } from 'livekit-server-sdk';

export default async function Page({
  params,
}: {
  params: {
    project_id: string;
    session_id: string;
  };
}) {
  const { project_id: projectId, session_id: sessionId } = params;
  let session = await auth();
  const user = session?.user;

  if (!user) {
    return (
      <ErrorComponent
        title="Error"
        error="Not authenticated"
        projectId={projectId}
      />
    );
  }
  const projectRes = await projectClient.getProject(projectId);

  let projectName = projectRes
    .map((project) => project.name)
    .unwrapOr('Unknown Project');

  return (
    await (
      await projectClient.getSession(projectId, sessionId)
    ).mapAsync(async (sessionInfo) => {
      const tokenRequest: VideoGrant = {
        room: sessionInfo.livekitRoomName,
        canPublish: true,
        canSubscribe: true,
        canPublishSources: [],
        canPublishData: true,
        canUpdateOwnMetadata: true,
        hidden: false,
        ingressAdmin: true,
        recorder: true,
        roomAdmin: true,
        roomCreate: true,
        roomJoin: true,
        roomList: true,
        roomRecord: true,
      };

      return (
        await projectClient.getSessionToken(
          projectId,
          sessionId,
          user.name || 'Anon',
          tokenRequest,
        )
      )
        .map((sessionToken) => {
          return (
            <Room
              key={`${projectId}-${sessionId}`}
              token={sessionToken.token}
              user={session?.user}
              lkServerUrl={sessionToken.livekitServerUrl}
              disconnectRedirectUrl={`/dashboard/projects/${projectId}/sessions/`}
            />
          );
        })
        .unwrapOrElse((error) => {
          return (
            <ErrorComponent
              title={`Error fetching session token for project ${projectName}`}
              error={error}
              projectId={projectId}
            />
          );
        });
    })
  ).unwrapOrElse((error) => {
    return (
      <ErrorComponent
        title={`Error fetching session for project ${projectName}`}
        error={error}
        projectId={projectId}
      />
    );
  });
}
