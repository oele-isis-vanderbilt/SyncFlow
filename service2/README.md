## LiveKitMMLA API
`actix-web` API for LiveKit to generate tokens/ manage rooms.

## Installation
Installation requires rust. This project uses `actix-web` and `actix-rt` for async handling.

```shell
$ git clone git@github.com:umesh-timalsina/LiveKitMMLAAPI.git
$ cd LiveKitMMLAAPI
```

After cloning the repository, create an environement file `.env` in the root directory of the project. The file should contain the following environment variables:

```shell
LIVEKIT_SERVER_URL="YOUR_LIVEKIT_SERVER_URL"
LIVEKIT_API_KEY="YOUR_LIVEKIT_API_KEY"
LIVEKIT_API_SECRET="YOUR_LIVEKIT_API_SECRET"
PORT = "8081"
```

Run the following command to start the server:

```shell
$ cargo run
```

API docs will be available `http://localhost:8081/redoc`.

## Docker

ToDo
