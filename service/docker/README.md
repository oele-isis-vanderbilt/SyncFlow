# Docker
This directory contains deployment files for running the service using docker compose

## Prerequisites
- Docker: https://docs.docker.com/get-docker/ 
- Docker Compose: https://docs.docker.com/compose/install/

## Environment Variables and Secrets
The following environment variables are required to run/configure the service:

```shell
LIVEKIT_SERVER_URL="YOUR_LIVEKIT_SERVER_URL"
LIVEKIT_API_KEY="YOUR_LIVEKIT_API_KEY"
LIVEKIT_API_SECRET="YOUR_LIVEKIT_API_SECRET"
APP_PORT="APP_HOST" # The port the service will listen on (hardcode to 8082 to use the compose files in this directory)
APP_HOST="APP_HOST" # The host the service will listen on (hardcode to 
DATABASE_URL="DATABASE_URL" # The database connection string
POSTGRES_USER="POSTGRES_USER" # The postgres user
POSTGRES_PASSWORD="POSTGRES_PASSWORD" # The postgres password
POSTGRES_DB="POSTGRES_DB" # The postgres database
PGADMIN_DEFAULT_EMAIL="PGADMIN_DEFAULT_EMAIL" # The pgadmin default email
PGADMIN_DEFAULT_PASSWORD="PGADMIN_DEFAULT_PASSWORD" # The pgadmin default password
PGADMIN_LISTEN_PORT="PGADMIN_LISTEN_PORT" # The pgadmin listen port
NUM_ACTIX_WORKERS="NUM_ACTIX_WORKERS" # The number of actix workers
JWT_SECRET="JWT_SECRET" # The jwt secret
```

The environment variables should be self-explanatory. For using the compose files in this directory, you can create an `.env.prod`(in the [`service`](../../service) directory) and set the environment variables there.  

## Images used for Deployment
Postgres and pgAdmin are used for the database and database management respectively. The service is built and run using the [`Dockerfile`](./Dockerfile). The database migrations are run using the [`Dockerfile.migrations`](./Dockerfile.migrations) file.


## Deployment Commands
To build and run the service using docker compose, run the following commands :

## Initial Deployment
> [!IMPORTANT]  
> Make sure you run these commands from the [`service`](../../service) directory.

```shell
$ docker-compose -p livekit-mmla-prod --file docker/docker-compose.prod.yaml up -d
```

## Update Deployment
Unless you have to stop the database and pgadmin for maintenance purposes, you can update the deployment by re-building the service/migration and running the following command:

```shell
$ docker-compose -p livekit-mmla-prod --file docker/docker-compose.prod.yaml down api-livekit-mmla-prod
$ docker-compose -p livekit-mmla-prod --file docker/docker-compose.prod.yaml build api-livekit-mmla-prod migrations-prod
$ docker-compose -p livekit-mmla-prod --file docker/docker-compose.prod.yaml up -d
```

## Stopping the Deployment
To stop the deployment, run the following command:

```shell
$ docker-compose -p livekit-mmla-prod --file docker/docker-compose.prod.yaml down
```

## Development Database
For development purposes, you can use the `docker-compose.postgres.yaml` file to run a postgres instance locally ( in this case use `.env` file name to set the environment variables):

```shell
$ docker-compose -p livekit-mmla-dev --file docker/docker-compose.postgres.yaml up -d
```
