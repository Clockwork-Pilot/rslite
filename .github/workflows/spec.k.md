# Specification

## Overview

Workflow structure constraints for .github/workflows

## Table of Contents

- [Overview](#overview)
- [Features](#features)
    - [Feature: coding_agent_steps](#feature-coding_agent_steps)
      - [dispatcher_fails_on_missing_step](#dispatcher_fails_on_missing_step)
      - [fetch_issue_job_shape](#fetch_issue_job_shape)
      - [parse_issue_consumes_fetch_issue](#parse_issue_consumes_fetch_issue)
      - [run_agent_consumes_parse_issue](#run_agent_consumes_parse_issue)
    - [Feature: docker_environment](#feature-docker_environment)
      - [docker_required](#docker_required)
    - [Feature: step_output_checks](#feature-step_output_checks)
      - [choose_branch_cases](#choose_branch_cases)
      - [parse_issue_cases](#parse_issue_cases)
    - [Feature: workflow_hygiene](#feature-workflow_hygiene)
      - [checkout_pinned_to_approved_sha](#checkout_pinned_to_approved_sha)
      - [no_checkout_v4_in_workflows](#no_checkout_v4_in_workflows)

## Features

### Feature: coding_agent_steps
**Coding agent steps for workflow automation**

**Goals:**
- Entry point called identically by workflow and constraint

#### dispatcher_fails_on_missing_step
**Description:** Negative: act-step-dispatch.sh must exit non-zero AND emit the missing-step error to stderr (not stdout) when the requested step does not exist

#### fetch_issue_job_shape
**Description:** Structural: fetch-issue job must exist with title and body_raw job-level outputs, and its run block must call gh api (it is the network-bound fetcher that feeds parse-issue).

#### parse_issue_consumes_fetch_issue
**Description:** Structural: parse-issue must declare needs: fetch-issue AND wire job-level env TITLE/BODY_RAW from needs.fetch-issue.outputs.* and ISSUE_NUMBER from inputs.issue_number. Env keys use natural names (no INPUT_ prefix) since the dispatcher resolves env expressions directly.

#### run_agent_consumes_parse_issue
**Description:** Structural: run-agent must depend on parse-issue; the legacy slug job must be gone; get-issue-details.sh must be deleted; no references to steps.issue.outputs.* may remain. Enforces the cutover to parse-issue as single source of parsed issue data.

### Feature: docker_environment
**Constraint checks must run inside Docker container**

**Goals:**
- Ensure constraint checker only runs in Docker environment, not on bare host

#### docker_required
**Description:** Verify constraints are running inside an existing Docker container (/.dockerenv present). We are already in Docker — do not nest another container layer.

### Feature: step_output_checks
**Behavioral checks: dispatch a workflow step with a fixture input and assert its $GITHUB_OUTPUT matches fixture.**

**Goals:**
- One behavioral constraint per dispatchable step, using mktemp -d for isolation

#### choose_branch_cases
**Description:** Behavioral: runs every fixture under fixtures/choose-branch/<case>/ and diffs emitted $GITHUB_OUTPUT. Adding a case is a folder drop — no spec edit needed.

#### parse_issue_cases
**Description:** Behavioral: runs every fixture under fixtures/parse-issue/<case>/ and diffs emitted $GITHUB_OUTPUT. Adding a case is a folder drop — no spec edit needed.

### Feature: workflow_hygiene
**Workflows use SHA-pinned actions**

**Goals:**
- All action references must be pinned to full commit SHAs for reproducibility and security

#### checkout_pinned_to_approved_sha
**Description:** Security: actions/checkout@v6 must be pinned to exactly de0fac2e4500dabe0009e67214ff5f5447ce83dd

#### no_checkout_v4_in_workflows
**Description:** Negative: no .github/workflows/*.yml file may reference actions/checkout@v4