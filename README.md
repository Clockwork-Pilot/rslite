# RSLITE is a port of Sqlite3 on rust.

Our goal is seamless integration of SQLite into Rust applications, keeping it as close to the original SQLite architecture as possible and ensuring it passes existing sqlite tests.

We’re iteratively porting SQLite to Rust by first generating an unsafe Rust equivalent. Then we link the existing SQLite tests and make changes in small steps, verifying the tests at each iteration. This way, each modification is checked and validated through a continuous development loop.

At this point, we haven’t actually changed the interface, so it remains fully compatible with the original SQLite. But since we plan to change it in the future as the project evolves, we’ve already ported some of these tests to Rust. This way, when we modify the library, we’ll also be adjusting the tests suite accordingly.

Because the SQLite API is still unchanged, we continue to rely on the original test suite, which we run using `make c-tcl-tests`.
The tests in the c2rust directory are currently intended for future changes - specifically for when we start breaking the existing C-based sqlite3 API. At that point, we will begin migrating both the library and the tests to ensure we can continue using our testing approach.

# Build and test

For development and testing, use the Docker-based local development environment described in the [Local Development Environment](#local-development-environment) section below. This ensures you're using the same environment as the agentic workflow.

## Local Development Environment

The agentic workflow runs in a Docker container based on `autopilot-ws`. If you want to develop locally using this harness, follow this section — you don't need GitHub or automated workflows to use it.

### Build Docker Image

Clone autopilot-ws and build rslite's custom Docker image using the streamlined approach:

```bash
git clone git@github.com:Clockwork-Pilot/autopilot-ws.git
cd autopilot-ws
git submodule update --init --recursive

# Build rslite's custom image layered on autopilot-ws base
./build-docker-workspace.sh /path/to/rslite/Dockerfile.agent
```

The `build-docker-workspace.sh` script handles building the base `autopilot-ws` image and layering `Dockerfile.agent` on top with proper context. The result is a local `autopilot-ws` tag ready to use.

### Set Up Workspace

Return to rslite and set the environment:

```bash
cd /path/to/rslite
export PROJECT_ROOT=$(pwd)
```

### Run in Docker

Use `autopilot-ws/run-docker-workspace.sh` to execute commands in the Docker container with rslite mounted:

```bash
# Start dev flow in dockerized environment
/path/to/autopilot-ws/run-docker-workspace.sh
```

The `docker-files/` directory persists artifacts (Rust cache, credentials, Python venv) across runs. See [autopilot-ws README](https://github.com/Clockwork-Pilot/autopilot-ws#docker-artifacts-folder) for details.

## Customizing the Docker Image

The `Dockerfile.agent` customizes the base `autopilot-ws` image with project-specific dependencies:

```dockerfile
ARG BASE_IMAGE=ghcr.io/clockwork-pilot/autopilot-ws:latest
FROM ${BASE_IMAGE}

RUN apt-get update \
    && apt-get install -y --no-install-recommends <your-deps> \
    && rm -rf /var/lib/apt/lists/*
```

For details on how image building works and caching strategies, see [autopilot/README.md — Installing extra dependencies](https://github.com/Clockwork-Pilot/autopilot#installing-extra-dependencies).

## Optional: Automated Workflow via GitHub Actions

You can optionally set up GitHub Actions to run the agentic workflow automatically when issues are labeled. This is an additional feature, not required for local development.

### Setup

1. **Install Self-Hosted Runner** — Register a GitHub Actions runner on your machine with your GitHub username as the label. See [autopilot-ws ansible setup](https://github.com/Clockwork-Pilot/autopilot-ws#installation).

2. **Configure Secrets** — Add to repository settings:
   - `RUNNERS_PAT` — Fine-grained PAT with `Administration: Read-only`
   - Optional: `UPSTREAM_PR_TOKEN` — For opening PRs on upstream repos

### Usage

1. **Open an issue** describing work to do. Optional YAML frontmatter:
   ```yaml
   ---
   timeout: 20           # Minutes (default 10)
   model: claude-opus-4-6 # Model ID (default claude-haiku-4-5)
   ---
   <describe the work>
   ```

2. **Apply the `agent-run` label** to trigger the automated workflow.

3. **Workflow executes** — GitHub Actions runs the agentic workflow in Docker, commits changes, and opens a PR with results.
