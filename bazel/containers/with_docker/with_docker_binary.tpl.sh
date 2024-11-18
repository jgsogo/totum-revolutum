
set -euo pipefail

echo ">>>> Run the docker compose %COMPOSE_FILE% (project name %PROJECT_NAME%)"
docker compose --project-name %PROJECT_NAME% --env-file %ENV_FILE% --file %COMPOSE_FILE% up --wait


echo ">>>> Just sleep for a few seconds"
sleep 5


echo ">>>> Stop and remove the docker compose %COMPOSE_FILE% (project name %PROJECT_NAME%)"
docker compose --project-name %PROJECT_NAME% --env-file %ENV_FILE% --file %COMPOSE_FILE% down --volumes --timeout=30
