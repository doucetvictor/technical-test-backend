#!/bin/bash

set -e

docker compose down -v --rmi local
docker image prune -f
