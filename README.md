# F1 Telemetry Monitor

Real-time F1 2025 game telemetry monitoring via UDP. Tauri 2.0 (React 18 + TypeScript frontend, Rust backend).

## Stack
- Frontend: React 18, TypeScript, Zustand, uPlot, Vite
- Backend: Rust, Tauri 2.0, tokio, nom (UDP parser), serde

## Port
UDP 20777 (default F1 25 telemetry port, configurable)

## Commands
```bash
# Frontend dev (port 1420)
npm run dev

# Full Tauri dev
npm run tauri:dev

# Build
npm run build
cargo tauri build

# Lint
npm run lint

# Rust check
cargo check --offline
```

## Data flow
F1 25 game (UDP 20777) → Rust listener → nom parser → broadcaster (~60 FPS) → Tauri event → Zustand store → React components

## Key packets
Session (1), LapData (2), Participants (4), CarTelemetry (6), CarStatus (7), CarDamage (10), TyreSets (12), MotionEx (13), SessionHistory (11)

## Game detection
Parser tracks active game via `m_session_uid` and `packet_format`. On game change, session state resets (lap history, best lap, current lap) after persisting previous session.

## Notes
- `raceiq-analysis` is intentionally excluded (`.gitignore`, removed from git history).
- No end-to-end test with real F1 25 game (no game in this environment).
