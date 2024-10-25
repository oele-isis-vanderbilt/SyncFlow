## Examples

- This [`example`](./example_client.rs) uses SyncFlow project client and creates a new session in `SyncFlow`, publish camera for 1 minutes, leaves and delete the session. To run this example, create an `.env` file with the following:
```{sh}
SYNCFLOW_PROJECT_ID="PROJECT_ID"
SYNCFLOW_BASE_URL="SYNCFLOW_API_URL"
SYNCFLOW_API_KEY="SYNCFLOW_API_KEY"
SYNCFLOW_API_SECRET="SYNCFLOW_API_SECRET"
```

And run the example:
```{sh}
$ cargo run --example example_client
```

- This [`example`](./devices.rs) uses the created SyncFlow project client and registers and deregisters an IOT device. To run this example, create an `.env` file with the following:
```{sh}
SYNCFLOW_PROJECT_ID="PROJECT_ID"
SYNCFLOW_BASE_URL="SYNCFLOW_API_URL"
SYNCFLOW_API_KEY="SYNCFLOW_API_KEY"
SYNCFLOW_API_SECRET="SYNCFLOW_API_SECRET"
```

And run the example:
```{sh}
$ cargo run --example devices
```
