'use client';

import {LiveKitRoom, useToken, VideoConference} from "@livekit/components-react";
import {LocalUserChoices} from "@livekit/components-core";
import {useMemo} from "react";
import {useRouter} from "next/navigation";

export default function LkRoom({roomName}: {roomName: string}) {
    const preJoinChoices: LocalUserChoices = JSON.parse(
        localStorage.getItem('preJoinChoices') || '{}',
    );
    let tokenOptions = useMemo(() => {
        return {
            userInfo: {
                identity: preJoinChoices.username,
                name: preJoinChoices.username,
            },
        };
    }, [preJoinChoices.username]);
    const token = useToken('/api/token', roomName, tokenOptions);
    let lkUrl = process.env.NEXT_PUBLIC_LIVEKIT_SERVER_URL;
    const router = useRouter();

    return (
        <div data-lk-theme="default">
            <LiveKitRoom
                className={'h-screen w-screen'}
                serverUrl={lkUrl}
                token={token}
                video={preJoinChoices.videoEnabled}
                audio={preJoinChoices.audioEnabled}
                onDisconnected={() => {
                    router.push('/');
                }}
            >
                <VideoConference/>
            </LiveKitRoom>
        </div>
    )
}