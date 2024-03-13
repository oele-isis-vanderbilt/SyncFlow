# Installation

When using a Tauri application, you will need to install both Rust and JS dependencies.

## Rust Prerequisites

Follow the [Tauri documentation](https://tauri.app/v1/guides/getting-started/prerequisites/) in regards to how to install Tauri for your own development setup. This includes Rust plus some additionally dependencies depending on your development OS.

## Front-End Dependencies

Within the ``iot`` subfolder, run the following command to install the necessary dependencies:

```
npm install
```

More information in regards to using [NextJS+Rust is provided in Tauri's documentation](https://tauri.app/v1/guides/getting-started/setup/next-js).

# Running

For running the desktop application, run the following command:

```bash
npm run tauri dev
```

It will printout in stdout the standard NextJS message with ```http://localhost:3000```, ignore this. A native window should open with the application.