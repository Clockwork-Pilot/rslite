#!/usr/bin/env bash
# Run an arbitrary bash command inside the agent docker image with the
# standard mount set. Thin wrapper — used by callers that need the mounts
# but not the claude-specific prompt/hooks wiring from run-in-docker-claude.sh.
#
# Required env: DOCKER_FILES, GITHUB_WORKSPACE
# Usage: run-in-docker-no-claude.sh <bash command string>
set -uo pipefail

: "${DOCKER_FILES:?DOCKER_FILES required}"
: "${GITHUB_WORKSPACE:?GITHUB_WORKSPACE required}"
CMD="${1:?bash command string required}"

docker run --rm \
  -e CLAUDE_FILE_RULES=/docker-scripts/y2-plugin-deny-file-rules.json \
  -e PROXY_WRAPPER_CONFIG=/docker-scripts/proxy_wrapper_config.json \
  -e DISABLE_STOP_HOOK= \
  -v "$DOCKER_FILES/.cargo:/home/node/.cargo:Z" \
  -v "$DOCKER_FILES/.credentials:/home/node/.claude:Z" \
  -v "$DOCKER_FILES/.claude.local.json:/home/node/.claude.json:Z" \
  -v "$DOCKER_FILES/.local:/home/node/.local:Z" \
  -v "$GITHUB_WORKSPACE:/workspace:Z" \
  ghcr.io/clockwork-pilot/rslite-ws:latest \
  bash -c "source /docker-scripts/user-entrypoint.sh; $CMD"
