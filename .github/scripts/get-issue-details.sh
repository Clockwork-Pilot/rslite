#!/bin/bash
# Extract and parse GitHub issue details
# Outputs: title, body, branch, timeout_secs, model, ONESHOT_PROMPT_MESSAGE

set -e

ISSUE_NUMBER="${1:?Issue number required}"
REPO="${2:?Repository required}"

echo "::group::Fetching issue #$ISSUE_NUMBER"

GH_TOKEN="${GH_TOKEN:?GH_TOKEN not set}"

ISSUE=$(gh api repos/$REPO/issues/$ISSUE_NUMBER)
TITLE=$(echo "$ISSUE" | jq -r '.title')
BODY=$(echo "$ISSUE" | jq -r '.body // ""')

echo "Title: $TITLE"
echo "Body length: ${#BODY} characters"
echo "::endgroup::"

echo "title=${TITLE}" >> $GITHUB_OUTPUT
echo "body=${BODY}" >> $GITHUB_OUTPUT

# Generate branch name from title
SLUG=$(echo "$TITLE" | tr '[:upper:]' '[:lower:]' | tr -cs 'a-z0-9' '-' | sed 's/-\+/-/g;s/^-//;s/-$//' | cut -c1-100)

# Check for existing branch
EXISTING=$(gh api "repos/$REPO/branches" --paginate -q ".[].name" \
  | grep -E "^agent/${ISSUE_NUMBER}(-|$)" | head -1 || true)

if [ -n "$EXISTING" ]; then
  echo "Reusing existing branch: $EXISTING"
  echo "branch=$EXISTING" >> $GITHUB_OUTPUT
else
  echo "branch=agent/${ISSUE_NUMBER}-${SLUG}" >> $GITHUB_OUTPUT
fi

# Parse optional YAML frontmatter block at top of body:
#   ---
#   timeout: 30              # minutes (default 10)
#   model: claude-opus-4-6   # default: claude-haiku-4-5
#   ---
YAML_PART=$(printf '%s\n' "$BODY" | grep -Pzo '(?s)^---\r?\n\K.*?(?=\r?\n---(\r?\n|$))' | tr -d '\0' || true)
TIMEOUT_MINS=""
MODEL=""

if [ -n "$YAML_PART" ]; then
  BODY=$(printf '%s\n' "$BODY" | sed '1,/^---\s*$/d; 1,/^---\s*$/d')
  TIMEOUT_MINS=$(yq -r '.timeout // ""' <<< "$YAML_PART" 2>/dev/null || true)
  MODEL=$(yq -r '.model // ""' <<< "$YAML_PART" 2>/dev/null || true)
  # yq may emit the literal string "null" when a key resolves to null
  [ "$TIMEOUT_MINS" = "null" ] && TIMEOUT_MINS=""
  [ "$MODEL" = "null" ] && MODEL=""
fi

TIMEOUT_SECS=$(( ${TIMEOUT_MINS:-10} * 60 ))
FINAL_MODEL="${MODEL:-claude-haiku-4-5}"

echo "timeout_secs=${TIMEOUT_SECS}" >> $GITHUB_OUTPUT
echo "model=${FINAL_MODEL}" >> $GITHUB_OUTPUT

# Export prompt message to environment
{
  echo "ONESHOT_PROMPT_MESSAGE<<EOF"
  echo "$TITLE"
  echo "$BODY"
  echo "EOF"
} >> $GITHUB_ENV

# Print summary
echo ""
echo "::group::Issue Details Summary"
echo "Issue #${ISSUE_NUMBER}: $TITLE"
echo "Branch: agent/${ISSUE_NUMBER}-${SLUG}"
echo "Timeout: ${TIMEOUT_MINS:-10} minutes ($TIMEOUT_SECS seconds)"
echo "Model: $FINAL_MODEL"
echo "::endgroup::"

echo ""
echo "::group::ONESHOT_PROMPT_MESSAGE (Claude will receive this)"
cat <<EOF
$TITLE
$BODY
EOF
echo "::endgroup::"
