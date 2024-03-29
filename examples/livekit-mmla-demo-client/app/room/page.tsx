/* eslint-disable */
import LkRoom from "../lk-room";

export default async function Page(  {searchParams}: { searchParams?: { [key: string]: string | string[] | undefined }}) {
  let roomName = searchParams?.name;
  if (!roomName) {
    return (
      <div style={{ display: 'grid', placeItems: 'center', height: '100%' }}>
        <p>Invalid Room</p>
      </div>
    );
  }



  return (
      <LkRoom roomName={roomName as string}></LkRoom>
  );
}
