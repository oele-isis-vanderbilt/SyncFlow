'use client';
import { useFormState } from 'react-dom';
import { createRoom } from '@/app/lib/actions';

export default function CreateRoom() {
  const initialState = {
    message: null,
    errors: {},
  };

  const [state, dispatch] = useFormState(createRoom, initialState);

  return (
    <form action={dispatch} aria-describedby="room-status">
      <div className="rounded-md p-4 md:p-6 flex flex-row justify-between">
        <label htmlFor="roomName" className="mb-2 block text-sm font-medium" />
        <input
          id="roomName"
          name="roomName"
          type="text"
          placeholder="Enter Room Name"
          className="peer block w-full rounded-md border border-gray-200 py-2 pl-10 text-sm text-gray-900 outline-2"
          aria-describedby="name-error"
        />
        <button type="submit" className="text-white bg-blue-700 hover:bg-blue-800 focus:outline-none focus:ring-4 focus:ring-blue-300 font-medium rounded-full text-sm px-5 py-2.5 text-center me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 ml-5 dark:focus:ring-blue-800">Create</button>
      </div>
      <div id="name-error" aria-live="polite" aria-atomic="true" className="pl-4 md:pl-6">
        {state.errors?.roomName &&
          state.errors?.roomName.map((error: string) => (
            <p className="mt-2 text-sm text-red-600" key={error}>
              {error}
            </p>
          ))}
      </div>
    </form>
  );
}
