import Greet from "./greet"
import NavBar from "@/components/NavBar";

export default function Page() {
  return (
    <div className="min-h-screen flex flex-col justify-between">
      <NavBar />
      <Greet />
    </div>
  );
}
