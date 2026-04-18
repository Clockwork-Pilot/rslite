#!/usr/bin/env bash
# Follow hooks.log written by the Claude agent and stream it to stdout so the
# agent run isn't a black box while `claude -p` runs silently inside Docker.
#
# Usage: $0 <hooks.log path> [--filtered]
#   default   — stream every line raw, prefixed with [hook]
#   --filtered — keep only known event types and rewrite them into concise
#                per-event summaries (compact but lossy; use when stable)
#
# Launch with `setsid` so cleanup via `kill -- -<pid>` only signals this
# pipeline, not the parent workflow shell.
set -uo pipefail
LOG="${1:?usage: $0 <hooks.log path> [--filtered]}"
MODE="${2:-}"

trap 'kill 0 2>/dev/null || true' EXIT INT TERM

if [ "$MODE" = "--filtered" ]; then
  tail -F -n 0 "$LOG" 2>/dev/null \
    | grep --line-buffered -E '"event": "(Edit|Write|MultiEdit|UserPromptSubmit|SessionStart|SessionEnd|Notification|Stop)"' \
    | sed -u -E '
        s#.*"event": "(Edit|Write|MultiEdit)".*"file_path": "([^"]+)".*#[hook] \1 \2#; t;
        s#.*"event": "UserPromptSubmit".*"prompt": "([^"]{0,1000}).*#[hook] prompt: \1#; t;
        s#.*"event": "SessionStart".*"model": "([^"]+)".*#[hook] SessionStart model=\1#; t;
        s#.*"event": "SessionEnd".*"reason": "([^"]+)".*#[hook] SessionEnd reason=\1#; t;
        s#.*"event": "Notification".*"message": "([^"]{0,1000}).*#[hook] Notification: \1#; t;
        s#.*"event": "Stop".*"last_assistant_message": "(([^"\\]|\\.){0,2000}).*#[hook] assistant: \1#; t;
        s#.*"event": "([^"]+)".*#[hook] \1#;
      '
else
  tail -F -n 0 "$LOG" 2>/dev/null | sed -u 's/^/[hook] /'
fi
