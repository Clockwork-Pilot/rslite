#!/usr/bin/env bash
# Follow hooks.log written by the Claude agent and print concise per-event
# summaries. Used by the Coding Agent workflow so the agent run isn't a
# black box while `claude -p` runs silently inside Docker.
#
# Launch with `setsid` so cleanup via `kill -- -<pid>` only signals this
# pipeline, not the parent workflow shell.
set -uo pipefail
LOG="${1:?usage: $0 <hooks.log path>}"

trap 'kill 0 2>/dev/null || true' EXIT INT TERM

tail -F -n 0 "$LOG" 2>/dev/null \
  | grep --line-buffered -E '"event": "(Edit|Write|MultiEdit|UserPromptSubmit|SessionStart|SessionEnd|Notification|Stop)"' \
  | sed -u -E '
      s#.*"event": "(Edit|Write|MultiEdit)".*"file_path": "([^"]+)".*#[hook] \1 \2#; t;
      s#.*"event": "UserPromptSubmit".*"prompt": "([^"]{0,200}).*#[hook] prompt: \1#; t;
      s#.*"event": "([^"]+)".*#[hook] \1#;
    '
