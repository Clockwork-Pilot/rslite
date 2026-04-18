#!/bin/bash
# Docker image cache management: load or save
# Usage: docker-cache.sh [load|save]

set -e

ACTION="${1:-}"

if [ -z "$ACTION" ]; then
  echo "::error::Usage: docker-cache.sh [load|save]"
  exit 1
fi

case "$ACTION" in
  load)
    echo "::group::Loading Cached Image"
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
    echo "::group::Pulling Image and Saving to Cache"
    echo "Cache miss - fetching fresh image"
    docker pull ghcr.io/clockwork-pilot/rslite-ws:latest || {
      echo "::error::Failed to pull docker image"
      exit 1
    }
    docker save ghcr.io/clockwork-pilot/rslite-ws:latest > /tmp/rslite-ws.tar || {
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
