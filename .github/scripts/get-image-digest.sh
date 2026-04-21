#!/bin/bash
# Extract docker image digest and emit a digest-pinned image ref.
# Outputs (to $GITHUB_OUTPUT):
#   digest=sha256:...
#   image=ghcr.io/clockwork-pilot/rslite-ws@sha256:...

set -e

IMAGE_REPO="ghcr.io/clockwork-pilot/rslite-ws"

echo "::group::Image Digest Extraction"

if ! manifest=$(docker manifest inspect "${IMAGE_REPO}:latest" 2>&1); then
  echo "::error::docker manifest inspect ${IMAGE_REPO}:latest failed:"
  echo "$manifest"
  exit 1
fi
echo "Manifest received (length: ${#manifest})"

digest=$(echo "$manifest" | python3 -c '
import sys, json

try:
  m = json.load(sys.stdin)
  if "manifests" in m:
    digest = m["manifests"][0]["digest"]
  elif "config" in m:
    digest = m["config"]["digest"]
  else:
    raise ValueError("No digest found in manifest")
  if not digest:
    raise ValueError("Digest is empty")
  print(digest)
except Exception as e:
  print(f"ERROR: Failed to extract digest: {e}", file=sys.stderr)
  sys.exit(1)
')

if [ -z "$digest" ]; then
  echo "::error::Failed to extract image digest - digest is empty"
  exit 1
fi

pinned="${IMAGE_REPO}@${digest}"
echo "Extracted digest: $digest"
echo "Pinned image ref: $pinned"
echo "digest=$digest" >> "$GITHUB_OUTPUT"
echo "image=$pinned" >> "$GITHUB_OUTPUT"
echo "::endgroup::"
