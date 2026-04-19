# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

AWB (Autonomous Web Browser) is a headless browser automation server built in Rust. It exposes a REST API (Rocket framework) that controls a Chromium instance via the `eoka` CDP wrapper library (vendored locally at `vendor/eoka`).

## Commands

```bash
# Build
cargo build
cargo build --release

# Run (server starts on http://0.0.0.0:8000)
cargo run

# Docker
docker-compose up        # recommended — allocates 1GB shm for Chrome stability
docker build -t awb .
make all                 # build + run Docker image

# Lint/Format
cargo clippy
cargo fmt
```

No automated tests exist — test manually via HTTP (e.g. `curl http://localhost:8000/health-check`).

## Architecture

```
REST Request → Rocket (src/main.rs) → Handler (src/requests/) → AutonomousWebBrowser (src/awb.rs)
                                                                         ↓
                                                               eoka (vendor/) → Chromium
```

**`src/awb.rs`** — Singleton `AutonomousWebBrowser` struct wrapping a `Browser` + `Page` from eoka. Uses `tokio::sync::OnceCell` for lazy init. Initialized at server launch.

**`src/main.rs`** — Rocket `#[launch]` entry point, mounts all route modules, initializes the browser.

**`src/requests/`** — One file per concern, all returning `ApiResponse<T>` (defined in `mod.rs`):
- `general.rs` — navigation, page title/URL/content/screenshot
- `input.rs` — click, hover, fill, type keys
- `find.rs` — find elements by selector or text (some endpoints WIP)
- `wait.rs` — delay and wait-for-network-idle

## Key Details

- `eoka` is patched via `[patch.crates-io]` in `Cargo.toml` to use `vendor/eoka` — do not update the crate version without also updating the vendor source.
- Browser runs with `StealthConfig` and `headless: false` (stealth mode, not truly headless).
- Chrome profile is persisted at `chrome/profile` (gitignored).
- Rocket config: address/port override via `ROCKET_ADDRESS` / `ROCKET_PORT` env vars.
