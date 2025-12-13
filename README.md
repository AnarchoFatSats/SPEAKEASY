# Speakeasy Privacy Suite (Blueprint + Starter Repo)

This repo is a **build-ready blueprint + code skeleton** for a cross-platform privacy app with:
- A configurable **Cover Mode** (e.g., Speakeasy bartender mini-game, notes, recipes)
- A **Private Room** behind a hidden unlock (gesture / sequence / "bouncer + secret agent")
- A Signal-style **end-to-end encrypted** messenger (inside-app, iOS + Android)
- A local **encrypted vault** for photos/videos/files/notes
- **Context privacy modes** (Car / Work / Public), implemented in platform-compliant ways
- Android-only optional: **Default SMS mode** with a public inbox + Private Inbox (per-number)

> IMPORTANT: This blueprint is designed to be **store-compliant** and does not attempt to intercept iMessage/SMS on iOS or bypass OS protections.

## Repo Layout
- `/docs/` - Product spec + CryptoSpec + ThreatModel + policies
- `/openapi/` - OpenAPI contract for the backend
- `/backend/` - Rust (Axum) relay backend + Postgres schema/migrations
- `/infra/` - Docker Compose dev stack (Postgres, Redis, MinIO)
- `/mobile_template/` - Flutter templates (copy into a Flutter project)
- `/android_sms_plugin_template/` - Kotlin plugin templates for Android default SMS mode
- `/scripts/` - bootstrap helpers for Cursor/IDE agents

## Quick Start (local dev)
1) Start infra:
```bash
cd infra
docker compose up -d
```

2) Start backend:
```bash
cd backend
cargo run
```

3) Create Flutter app and copy templates:
```bash
bash scripts/init_flutter_project.sh
```

Then open the repo in Cursor and follow `/docs/BUILD_PLAN.md`.
