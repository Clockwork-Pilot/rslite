#!/bin/bash
# Docker image cache management: load or save.
# Usage: AGENT_IMAGE=<digest-pinned ref> docker-cache.sh [load|save]

set -e

ACTION="${1:-}"
: "${AGENT_IMAGE:?AGENT_IMAGE required (use a digest-pinned ref like repo@sha256:...)}"

if [ -z "$ACTION" ]; then
  echo "::error::Usage: docker-cache.sh [load|save]"
  exit 1
fi

case "$ACTION" in
  load)
    echo "::group::Loading Cached Image ($AGENT_IMAGE)"
    if [ ! -f /tmp/rslite-ws.tar ]; then
      echo "::error::Cache hit reported but /tmp/rslite-ws.tar not found"
      exit 1
    fi
    tar_size=$(stat -f%z /tmp/rslite-ws.tar 2>/dev/null || stat -c%s /tmp/rslite-ws.tar 2>/dev/null || echo "unknown")
    echo "Loading cached image (size: $tar_size bytes)"
    docker load < /tmp/rslite-ws.tar || {
      echo "::error::Failed to load cached docker image from /tmp/rslite-ws.tar"
      exit 1
    }
    echo "Image loaded successfully"
    echo "::endgroup::"
    ;;

  save)
    echo "::group::Pulling Image and Saving to Cache ($AGENT_IMAGE)"
    echo "Cache miss - fetching image by digest"
    docker pull "$AGENT_IMAGE" || {
      echo "::error::Failed to pull docker image $AGENT_IMAGE"
      exit 1
    }
    docker save "$AGENT_IMAGE" > /tmp/rslite-ws.tar || {
      echo "::error::Failed to save docker image to cache"
      exit 1
    }
    tar_size=$(stat -f%z /tmp/rslite-ws.tar 2>/dev/null || stat -c%s /tmp/rslite-ws.tar 2>/dev/null || echo "unknown")
    echo "Image saved to cache (size: $tar_size bytes)"
    echo "::endgroup::"
    ;;

  *)
    echo "::error::Unknown action '$ACTION'. Use 'load' or 'save'"
    exit 1
    ;;
esac
