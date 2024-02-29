# LiveKit API/ Service Docker

This is a docker image for the LiveKit API/ Service. This provides a docker package for the LiveKit API/ Service.

## Usage

To use this image, you will need to provide a configuration file. The configuration file should be named `config.yml` and should be placed in a directory. This directory should be mounted to the `/config` directory in the container.

```bash
$ docker compose --file docker/docker-compose.yml up --build 
```
