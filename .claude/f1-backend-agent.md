---
name: f1-backend-agent
description: Specialized agent for Rust backend development on the F1 Telemetry Monitor (Tauri 2.0, tokio, nom, serde)
metadata:
  type: reference
---

# F1 Telemetry Backend Agent

This agent handles Rust backend work for the F1 2025 Telemetry Monitor (Tauri 2.0).

## Scope

- **UDP listener** (`src-tauri/src/telemetry/udp_listener.rs`): Tokio UDP socket on port 20777, listener handle management (start/stop)
- **Parser** (`src-tauri/src/telemetry/parser.rs`): nom-based parser for all 16 F1 25 packet types → `TelemetryEvent` enum
- **Packets** (`src-tauri/src/telemetry/packets.rs`): F1 25 UDP packet structs (16 packet types, enums, constants, `#[repr(C, packed)]`)
- **Broadcaster** (`src-tauri/src/telemetry/broadcaster.rs`): `SessionManager` global state, throttled event emission (~60 FPS telemetry, 1 Hz connection), session boundary detection via `m_session_uid`
- **Persistence** (`src-tauri/src/telemetry/persistence.rs`): Local JSON session persistence under `dirs::data_dir()/f1-telemetry/sessions/`
- **Commands** (`src-tauri/src/commands.rs`): Tauri command handlers (start/stop telemetry, session info, drivers, settings, export, list/get/delete/save sessions)
- **main.rs**: App entry, Tauri setup, command registration

## Key Patterns

- Packet structs use `#[repr(C, packed)]` matching wire format; little-endian parsing with `nom`
- All per-car arrays are 22 elements (`MAX_CARS`); use `m_player_car_index` from header for player data
- Session boundary detected when `m_session_uid` changes → auto-persist previous session via `persistence::persist_current_session`
- Throttling: telemetry ~60 FPS (16ms), connection status 1 Hz (1000ms)
- `tracing` for logging, `thiserror` for errors, `serde` for Tauri command serialization
- Settings persisted as `settings.json` in `dirs::data_dir()/f1-telemetry/`

## Current State

- All 16 packet IDs parsed (0-15); CarDamage (10), TyreSets (12), MotionEx (13), SessionHistory (11) wired to frontend
- `stop_telemetry` aborts the UDP listener handle; `clear_session_data` clears `SessionManager` state
- `list_sessions`/`get_session`/`delete_session`/`save_current_session` commands implemented
- Backend compiles clean via `cargo check` (only pre-existing warnings)

## When to invoke

Use this agent for: adding new packet types, modifying parsers, changing Tauri commands, updating persistence layer, fixing Rust compile errors, adjusting broadcaster throttling/session logic.
