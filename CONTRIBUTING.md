# SyncFlow Contributing Guide

SyncFlow is an open-source project and we welcome contributions from the community. This guide outlines the process for contributing to `SyncFlow`. 

# Preparing

The `SyncFlow` repository is hosted on Github and can be accessed [here](https://github.com/oele-isis-vanderbilt/SyncFlow.git). The mono-repo contains the following:

- `dashboard`: The frontend dashboard for managing projects, sessions and resources. This is a NextJS application.
- `service`: The backend service for managing projects, sessions and resources. Several rust crates consitute the monolith backend service.

Apart from the main repository components, you need a database (postgres) and a message broker (RabbitMQ) to run the service. The service is containerized using Docker and can be run locally or deployed to a cloud provider. In this guide, we will focus on setting up the development environment and running the service locally.

To contribute to the project, you need to set up your development environment by following these steps. You need to have the following tools installed on your local machine:

- [Node.js](https://nodejs.org/en/download/) and npm
- [Python](https://www.python.org/downloads/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

## Setting up the Development Environment
First, Clone the `SyncFlow` repository to your local machine using the following command:

   ```bash
   $ git clone https://github.com:oele-isis-vanderbilt/SyncFlow.git
   $ cd SyncFlow
   ```

## Configuration and Environment Variables
Create a deployment configuration file by using the following json structure and command to create the necessary environment files.
```json
{
    "dashboard": {
        "syncflow_api_url": "<URL for SyncFlow API>",
        "next_public_syncflow_api_url": "<Public URL for SyncFlow API>",
        "next_public_livekit_server_url": "<LiveKit Server Public URL>",
        "auth_secret": "<Authentication Secret Key>",
        "nextauth_url": "<NextAuth Service URL>",
        "nextauth_internal_url": "<NextAuth Internal Service URL>",
        "github_client_id": "<GitHub OAuth Client ID>",
        "github_client_secret": "<GitHub OAuth Client Secret>"
    },
    "service": {
        "app_host": "<Service Hostname or IP>",
        "app_port": "<Service Port Number>",
        "database_url": "<Database Connection URL>",
        "num_actix_workers": "<Number of Actix Workers>",
        "jwt_secret": "<JWT Secret Key>",
        "encryption_key": "<Encryption Key for Sensitive Data>",
        "jwt_expiration": "<JWT Expiration Time in Seconds>",
        "jwt_refresh_expiration": "<JWT Refresh Expiration Time in Seconds>",
        "rabbitmq_config": {
            "root_username": "<RabbitMQ Admin Username>",
            "root_password": "<RabbitMQ Admin Password>",
            "host": "<RabbitMQ Hostname>",
            "port": "<RabbitMQ Port Number>",
            "use_ssl": "<true/false>",
            "vhost_name": "<RabbitMQ Virtual Host Name>",
            "queue_name": "<Default Queue Name>",
            "exchange_name": "<Default Exchange Name>"
        },
        "github_client_id": "<GitHub OAuth Client ID (optional)>",
        "github_client_secret": "<GitHub OAuth Client Secret (optional)>",
        "root_user": {
            "username": "<Root User Username>",
            "email": "<Root User Email>",
            "password": "<Root User Password>"
        },
        "login_token": "<Temporary Login Token (optional)>",
        "test_user": "<Test User Username (optional)>",
        "test_password": "<Test User Password (optional)>"
    },
    "database_pgadmin": {
        "postgres_user": "<PostgreSQL Admin Username>",
        "postgres_password": "<PostgreSQL Admin Password>",
        "postgres_db": "<Database Name>",
        "pgadmin_default_email": "<PgAdmin Default Email>",
        "pgadmin_default_password": "<PgAdmin Default Password>",
        "pgadmin_listen_port": "<PgAdmin Service Port Number>"
    },
    "migration": {
        "database_url": "<Database Connection URL for Migrations>"
    }
}
```

Each of the configuration options are described below:

### Dashboard
There are different configuration variables for the dashboard. These include:

* `syncflow_api_url`: The URL for the SyncFlow API. In development, we use `http://localhost:8081`.

* `next_public_syncflow_api_url`: The public URL for the SyncFlow API. In development, we use `http://localhost:8081`.

* `next_public_livekit_server_url`: The public URL for the LiveKit server. In development, we use `ws://localhost:7880`. This is no longer used.

* `auth_secret`: The authentication secret key for the dashboard. This is used to encrypt sessions and user data. Please use openssl to generate a random secret key.

* `nextauth_url`: The URL for the NextAuth service. In development, we use `http://localhost:3000`.

* `nextauth_internal_url`: The internal URL for the NextAuth service. In development, we use `http://localhost:3000`.

* `github_client_id`: The GitHub OAuth client ID. This is used for authentication with GitHub OAuth. Optional.

* `github_client_secret`: The GitHub OAuth client secret. This is used for authentication with GitHub OAuth. Optional.

### PostgreSQL and PgAdmin
There are different configuration variables for PostgreSQL and PgAdmin. These include the secrets and connection details for the database to be used by the docker compose file for these containers.

* `postgres_user`: The PostgreSQL admin username.

* `postgres_password`: The PostgreSQL admin password. 

* `postgres_db`: The database name. For development, we use `syncflow`.

* `pgadmin_default_email`: The default email for the PgAdmin service.

* `pgadmin_default_password`: The default password for the PgAdmin service.

* `pgadmin_listen_port`: The port number for the PgAdmin service. For development, we use `80`.

The postgres database in this case is available via port `15432` and the pgadmin service is available via port `5050`, as can be seen in the [docker-compose](https://github.com/oele-isis-vanderbilt/SyncFlow/blob/main/docker/docker-compose.postgres.yaml) file.


### RabbitMQ
There are different configuration variables for RabbitMQ. These include the secrets and connection details for the message broker to be used by the docker compose file for these containers(management console and broker).

* `root_username`: The RabbitMQ admin username.
* `root_password`: The RabbitMQ admin password.
* `host`: The RabbitMQ hostname. For development, we use `localhost`.
* `port`: The RabbitMQ port number. For development, we use `5672`.
* `use_ssl`: Whether to use SSL for the connection. For development, we use `false`.
* `vhost_name`: The RabbitMQ virtual host name. For development, we use `syncflow`.
* `queue_name`: The default queue name. For development, we use `syncflow_session_notifier_queue`.
* `exchange_name`: The default exchange name. For development, we use `syncflow_session_notifier_exchange`.

Note that the RabbitMQ configuration is a nested object within the service configuration.

### Service
There are different configuration variables for the service. These include the secrets and connection details for the service to be used by the docker compose file for these containers.

* `app_host`: The service hostname or IP. For development, we use `localhost`.
* `app_port`: The service port number. For development, we use `8081`.
* `database_url`: The database connection URL. For development, we use `postgres://syncflow:syncflow@localhost:15432/syncflow`. Change this to the appropriate database URL that you have set up.
* `num_actix_workers`: The number of Actix workers. For development, we use `4`.
* `jwt_secret`: The JWT secret key. This is used to sign the JWT tokens. Please use openssl to generate a random secret key.
* `encryption_key`: The encryption key for sensitive data. This is used to encrypt sensitive data in the database. Please use openssl to generate a random encryption key. This should be kept secret and can be used to encrypt and decrypt sensitive data.
* `jwt_expiration`: The JWT expiration time in seconds. For development, we use `3600`.
* `jwt_refresh_expiration`: The JWT refresh expiration time in seconds. For development, we use `604800`.
* `github_client_id`: The GitHub OAuth client ID. This is used for authentication with GitHub OAuth. Optional.
* `github_client_secret`: The GitHub OAuth client secret. This is used for authentication with GitHub OAuth. Optional.
* `root_user`: The root user details. This is the default user that is created when the service is started. This is used for development purposes.
    * `username`: The root user username(This will be the first user created in the system).
    * `email`: The root user email.
    * `password`: The root user password.
* `login_token`: The temporary login token. This is used for development purposes. This is no longer used and/or used for testing.
* `test_user`: The test user username. This is used for development purposes. This is no longer used and/or used for testing.
* `test_password`: The test user password. This is used for development purposes. This is no longer used and/or used for testing.
* `rabbitmq_config`: The RabbitMQ configuration. This is a nested object that contains the RabbitMQ configuration details. This is used to connect to the RabbitMQ broker and send messages to the broker. See the RabbitMQ configuration above for more details.

### Migration
These are the configuration variables for the migrations to run for the database. 

* `database_url`: The database connection URL for the migrations. For development, we use `postgres://syncflow:syncflow@localhost:15432/syncflow`.

Once you have created the configuration file (config.json), you can create the necessary environment files by running the following command:

```bash
$ python docker/generate-prod-config --config-file config.json --outfile-name .env
```

This will create the necessary environment files for the dashboard, service, database and migrations. The environment files will be created in the root of necessary directories. Check and verify that the environment files have been created successfully.


## Starting PostgreSQL and PgAdmin
To start the PostgreSQL and PgAdmin services, run the following command:

```bash
$ docker-compose -f docker/docker-compose.postgres.yaml up -d
```

This will start the PostgreSQL and PgAdmin services in the background. You can access the PgAdmin service at `http://localhost:5050` and login with the default email and password that you set in the configuration file.

## Starting RabbitMQ
To start the RabbitMQ service, run the following command:

```bash
$ docker-compose -f docker/docker-compose.rmq.yaml up -d
```

This will start the RabbitMQ service in the background. You can access the RabbitMQ management console at `http://localhost:15672` and login with the admin username and password that you set in the configuration file.

## Starting the SyncFlow API
Once the containers are successfully started, you can start the SyncFlow API by running the following command:

```bash
$ cd crates 
$ cd infrastructure
$ diesel migration run # Run the migrations
$ cd ..
$ cargo run --bin api # Start the API
```

This will start the SyncFlow API on the specified host and port. You can access the API at `http://localhost:8081` or the specified host and port in the configuration file.

## Starting the SyncFlow Dashboard
To start the SyncFlow dashboard, run the following command:

```bash
$ cd dashboard
$ npm install
$ npm run dev
```

This will start the SyncFlow dashboard on the specified host and port. You can access the dashboard at `http://localhost:3000`.

# Contributing
Contributions to `SyncFlow` are welcome and we encourage you to contribute to the project. You can contribute to the project by:

1. Reporting issues and bugs at the [issue tracker](https://github.com/oele-isis-vanderbilt/SyncFlow/issues).
2. Contributing code by forking the repository and creating a pull request.