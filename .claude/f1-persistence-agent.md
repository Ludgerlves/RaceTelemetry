---
name: f1-persistence-agent
description: Specialized agent for session persistence layer on the F1 Telemetry Monitor (local JSON storage, dirs crate, serde_json)
metadata:
  type: reference
---

# F1 Telemetry Persistence Agent

This agent handles local session persistence for the F1 2025 Telemetry Monitor.

## Scope

- **Persistence** (`src-tauri/src/telemetry/persistence.rs`): SessionRecord/SessionListItem structs, JSON file storage under `dirs::data_dir()/f1-telemetry/sessions/`
- **Commands** (`src-tauri/src/commands.rs`): `list_sessions`, `get_session`, `delete_session`, `save_current_session` Tauri commands
- **Store** (`src/stores/telemetryStore.ts`): `sessions`, `selectedSession` state + `setSessions`/`setSelectedSession`/`deleteSession` actions
- **Hook** (`src/hooks/useSessions.ts`): Wraps persistence commands with store actions
- **UI** (`src/components/SessionHistory/`): Session history browser + lap history table

## Key Patterns

- Session records stored as JSON files, one per session, named `{session_uid}_{started_at_ms}.json`
- `sessions_dir()` creates `data_dir/f1-telemetry/sessions/` directory (mirrors existing `settings.json` pattern)
- `make_session_id(session_uid, started_at)` generates stable file id
- Auto-persist on session boundary: when `m_session_uid` changes, previous session is persisted via `persist_current_session` before state resets
- Manual save: `save_current_session` command persists current live state on demand
- Frontend ↔ Rust types: `SessionListItem` (lightweight listing) and `SessionRecord` (full record with participants, lap history, sector times)

## Data Structures

```rust
SessionListItem { id, session_uid, track_name, session_type, started_at, duration_ms, best_lap_time, total_laps, weather }
SessionRecord { id, session_uid, track_name, session_type, session_time_left, session_duration, weather, track_temp, air_temp, total_laps, player_car_index, active_driver_index, num_active_cars, started_at, participants, lap_history, best_lap_time, last_lap_time, sector_times, best_sector_times }
```

## Frontend Integration

- `useSessions` hook: `loadSessions()`, `loadSession(id)`, `removeSession(id)`, `saveCurrent()`
- Store actions: `setSessions(list)`, `setSelectedSession(record)`, `deleteSession(id)`
- `SessionHistory` component lists sessions with search; `LapHistoryTable` displays laps with sectors, delta to best, best-lap highlighting

## Current State

- Persistence layer complete; all commands registered in `main.rs` invoke_handler
- Auto-persist on session_uid change in `broadcaster.rs`
- Sessions UI: browser with search, delete, selection; lap table with delta-to-best and valid/invalid marking

## When to invoke

Use this agent for: changing the persistence format/storage location, adding new session fields, modifying persistence commands, updating the sessions/lap history UI, fixing serialization issues between Rust and TypeScript.
