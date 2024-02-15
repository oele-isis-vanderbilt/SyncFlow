# LiveKit Local Deployment

## Overview

This guide provides instructions for setting up a local LiveKit deployment using Docker Compose.  This setup includes LiveKit server, Redis for messaging and coordination, and LiveKit's Egress service for handling media recording and streaming outside of real-time sessions.

### Warning on Secrets Management

**Important**: This configuration includes sensitive secrets (API keys, API secrets) directly within the configuration files. Exposing secrets in this manner is not recommended for production environments. Consider using environment variables or secure secrets management solutions for handling sensitive information.

## Prerequisites

- Docker and Docker Compose installed on your machine.

## Setup Instructions

1. **Clone Repository**: Ensure all configuration files (`livekit.yaml`, `egress.yaml`) are placed in the same directory as your `docker-compose.yml` file.

2. **Configuration Files**: Review and modify the configuration files (`livekit.yaml`, `egress.yaml`) as needed to fit your deployment scenario. Be especially mindful of the secrets (API keys and secrets) and network settings.

3. **Docker Compose**: Navigate to the directory containing your `docker-compose.local.yaml` and configuration files, then start your services:

    ```bash
    cd docker
    docker-compose --file docker-compose.local.yaml up -d
    ```

    This command will pull the necessary Docker images and start the services defined in your `docker-compose.yml` file.

4. **Verify Operation**: After starting the services, ensure that they are running correctly. You can check the logs of each service using Docker Compose:

    ```bash
    docker-compose logs [service_name]
    ```

    Replace `[service_name]` with the name of the service you wish to inspect (e.g., `livekit`, `redis`, `egress`).


## Support

For issues specific to LiveKit or its components, consult the [LiveKit documentation](https://docs.livekit.io/) or their support channels. For Docker or Docker Compose related queries, the Docker documentation and community forums are excellent resources.

