#!/bin/bash

set -eu

COMPOSE_FILE=docker-compose.yaml
docker compose -f $COMPOSE_FILE down -v &&
docker-compose -f $COMPOSE_FILE up --build --abort-on-container-exit --exit-code-from

