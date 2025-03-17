#!/bin/bash

# Script to rebuild and restart Docker containers

# Ensure secrets are set up
./setup-secrets.sh

# Export the password from the secret file for Docker Compose
export POSTGRES_PASSWORD=$(cat secrets/postgres_password.txt)

# Rebuild and restart containers
echo "Rebuilding and restarting Docker containers..."
docker compose down
docker compose build
docker compose up -d

# Show container status
echo "Container status:"
docker compose ps

# Show logs from the backend
echo "Backend logs:"
docker compose logs backend | tail -n 20

echo ""
echo "Rebuild complete. Use 'docker compose logs -f' to follow logs." 