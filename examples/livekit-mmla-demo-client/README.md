## Livekit MMLA Demo Client

This NextJS application uses the Livekit MMLA API to create an example browser-based video streaming application for the Livekit MMLA service.

### Features

- Uses signed JSON web tokens for sending requests to the Livekit MMLA API
- Uses NextJS and Livekit React components for building the UI

### Getting Started

First, install the dependencies, from the root of the repository, run the following commands:

```bash
cd examples/livekit-mmla-demo-client
npm install
```

Create an `.env` file with the following content:

```bash
LIVEKIT_MMLA_API_KEY="YOUR API KEY"
LIVEKIT_MMLA_API_SECRET="YOUR API SECRET
LIVEKIT_MMLA_API_BASE_URL="THE BASE URL OF THE LIVEKIT MMLA API"
LIVEKIT_MMLA_SERVER_URL="THE WS URL OF THE LIVEKIT SERVER"
NEXT_PUBLIC_LIVEKIT_SERVER_URL="THE WS URL OF THE LIVEKIT SERVER"
```

Then, run the development server:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
# or
bun dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.
