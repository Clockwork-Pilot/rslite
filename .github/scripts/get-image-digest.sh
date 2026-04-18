#!/bin/bash
# Extract docker image digest with error handling

set -e

echo "::group::Image Digest Extraction"

manifest=$(docker manifest inspect ghcr.io/clockwork-pilot/rslite-ws:latest 2>&1)
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

echo "Extracted digest: $digest"
echo "digest=$digest" >> $GITHUB_OUTPUT
echo "::endgroup::"
