import { projectClient } from '@/app/lib/project-client';
import DataSharer from '@/app/ui/dashboard/project/data-sharing-page';
import ErrorComponent from '@/app/ui/dashboard/project/error-component';
import { auth } from '@/auth';
import { VideoGrant } from 'livekit-server-sdk';
import type { AudioPresets, videoCodecs, VideoPresets } from 'livekit-client';

export default async function Page({
  params,
  searchParams,
}: {
  params: {
    project_id: string;
    session_id: string;
  };
  searchParams?: { [key: string]: string | string[] | undefined };
}) {
  const { project_id: projectId, session_id: sessionId } = params;
  let session = await auth();
  const user = session?.user;
  // const audioCo

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

  const identity = searchParams?.identity as string;
  const videoCodec = (searchParams?.videoCodec ||
    'h264') as (typeof videoCodecs)[number];
  const audioPreset = (searchParams?.audioPreset ||
    'speech') as keyof typeof AudioPresets;
  const videoPreset = (searchParams?.videoPreset ||
    'h1080') as keyof typeof VideoPresets;

  if (!identity) {
    return (
      <ErrorComponent
        title="Error"
        error="Identity not provided"
        projectId={projectId}
      />
    );
  }

  const joinOptions = {
    videoPreset: videoPreset,
    audioPreset: audioPreset,
    videoCodec: videoCodec,
  };

  return (
    await (
      await projectClient.getSession(projectId, sessionId)
    ).mapAsync(async (sessionInfo) => {
      if (sessionInfo.status !== 'Started') {
        return (
          <ErrorComponent
            title="Error"
            error={`Session ${sessionInfo.name} is not started`}
            projectId={projectId}
          />
        );
      }

      const tokenRequest: VideoGrant = {
        room: sessionInfo.name,
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
          identity,
          tokenRequest,
        )
      )
        .map((sessionToken) => {
          return (
            <DataSharer
              key={`${projectId}-${sessionId}`}
              token={sessionToken.token}
              sessionName={sessionInfo.name}
              joinOptions={joinOptions}
              user={user.name || 'Anon'}
              lkServerUrl={sessionToken.livekitServerUrl}
              disconnectRedirectUrl={`/dashboard/projects/${projectId}/sessions/`}
              settingsUrl={`/dashboard/projects/${projectId}/sessions/`}
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
