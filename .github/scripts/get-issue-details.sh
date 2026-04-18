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

# Parse optional YAML frontmatter block at top of body:
#   ---
#   timeout: 30              # minutes (default 10)
#   model: claude-opus-4-6   # default: claude-haiku-4-5
#   ---
# Extract YAML between --- markers using sed (portable, no grep -P)
YAML_PART=$(printf '%s\n' "$BODY" | sed -n '/^---$/,/^---$/p' | sed '1d;$d' || true)
if [ -n "$YAML_PART" ]; then
  echo "DEBUG: Found YAML: ${#YAML_PART} chars" >&2
else
  echo "DEBUG: No YAML frontmatter found" >&2
fi
TIMEOUT_MINS=""
MODEL=""
BASE_BRANCH=""
PR_BRANCH=""

if [ -n "$YAML_PART" ]; then
  echo "DEBUG: Found YAML frontmatter, parsing..." >&2
  # Remove YAML block: delete from start to first ---, then start to second ---
  BODY=$(printf '%s\n' "$BODY" | sed '0,/^---\s*$/d' | sed '0,/^---\s*$/d')

  if [ -z "$(which yq)" ]; then
    echo "::warning::yq not installed, skipping YAML parsing" >&2
  else
    TIMEOUT_MINS=$(yq -r '.timeout // ""' <<< "$YAML_PART" 2>/dev/null || true)
    MODEL=$(yq -r '.model // ""' <<< "$YAML_PART" 2>/dev/null || true)
    BASE_BRANCH=$(yq -r '.base_branch // ""' <<< "$YAML_PART" 2>/dev/null || true)
    PR_BRANCH=$(yq -r '.pr_branch // ""' <<< "$YAML_PART" 2>/dev/null || true)
    echo "DEBUG: timeout=$TIMEOUT_MINS model=$MODEL base_branch=$BASE_BRANCH pr_branch=$PR_BRANCH" >&2
  fi

  # yq may emit the literal string "null" when a key resolves to null
  [ "$TIMEOUT_MINS" = "null" ] && TIMEOUT_MINS=""
  [ "$MODEL" = "null" ] && MODEL=""
  [ "$BASE_BRANCH" = "null" ] && BASE_BRANCH=""
  [ "$PR_BRANCH" = "null" ] && PR_BRANCH=""
fi

echo "title=${TITLE}" >> $GITHUB_OUTPUT

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

TIMEOUT_SECS=$(( ${TIMEOUT_MINS:-10} * 60 ))
FINAL_MODEL="${MODEL:-claude-haiku-4-5}"

echo "timeout_secs=${TIMEOUT_SECS}" >> $GITHUB_OUTPUT
echo "model=${FINAL_MODEL}" >> $GITHUB_OUTPUT
echo "base_branch=${BASE_BRANCH}" >> $GITHUB_OUTPUT
echo "pr_branch=${PR_BRANCH}" >> $GITHUB_OUTPUT

# Export prompt message to environment (body only — title is metadata for branch/PR naming)
{
  echo "ONESHOT_PROMPT_MESSAGE<<EOF"
  echo "$BODY"
  echo "EOF"
} >> $GITHUB_ENV

# Print summary
echo ""
echo "::group::Issue Details Summary"
echo "Issue #${ISSUE_NUMBER}: $TITLE"
if [ -n "$PR_BRANCH" ]; then
  echo "PR branch (merge to): $PR_BRANCH"
fi
echo "Work branch: agent/${ISSUE_NUMBER}-${SLUG}"
if [ -n "$BASE_BRANCH" ]; then
  echo "Base branch (create from): $BASE_BRANCH"
fi
echo "Timeout: ${TIMEOUT_MINS:-10} minutes ($TIMEOUT_SECS seconds)"
echo "Model: $FINAL_MODEL"
echo "::endgroup::"

echo ""
echo "::group::ONESHOT_PROMPT_MESSAGE (Claude will receive this)"
cat <<EOF
$BODY
EOF
echo "::endgroup::"
