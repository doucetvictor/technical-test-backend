#!/bin/bash

set -e

if [ ! -f ".env" ]; then
    touch .env
    chmod 600 .env

    echo "DATABASE_DB=price" >> .env
    echo "DATABASE_USER=price" >> .env
    echo "DATABASE_PASSWORD=price" >> .env
fi

docker compose build
docker compose up $@
