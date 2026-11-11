# F1 Telemetry Monitor

Real-time F1 2025 game telemetry monitoring via UDP. Built with Tauri 2.0 (React 18 + TypeScript frontend, Rust backend).

---

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Supported Games](#supported-games)
  - [F1 2025](#f1-2025)
  - [Assetto Corsa / ACC](#assetto-corsa--acc)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Data Flow](#data-flow)
- [UDP Packets](#udp-packets)
- [Game Detection](#game-detection)
- [Persistence](#persistence)
- [Development](#development)
- [Testing](#testing)
- [Project Structure](#project-structure)
- [License](#license)

---

## Overview

F1 Telemetry Monitor captures real-time UDP telemetry from the F1 2025 game (and, on Windows, Assetto Corsa / ACC via shared memory) and displays it in a live dashboard. The backend parses binary packets with `nom` and forwards them to a React frontend over Tauri events.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Game (UDP / Shared Memory)                  │
│                    F1 2025 · Assetto Corsa · ACC                    │
└──────────────────────────┬──────────────────────────────────────────┘
                           │ UDP packets / shared memory sections
                           ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      Rust Backend (src-tauri/)                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────────┐  │
│  │ UDP Listener │──▶│   Parser     │──▶│   Broadcaster /        │  │
│  │   (tokio)    │   │    (nom)     │   │   SessionManager       │  │
│  └──────────────┘  └──────────────┘  └──────────────┬────────────┘  │
│                                                     │ Tauri Events   │
│  ┌─────────────────────────────────────────────────┘                 │
│  │  Tauri Commands (start/stop, settings, sessions, shared memory) │  │
│  └──────────────────────────────────────────────────────────────────┘  │
└──────────────────────────┬──────────────────────────────────────────┘
                           │ Tauri Events / Commands
                           ▼
┌─────────────────────────────────────────────────────────────────────┐
│                     React Frontend (src/)                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────────┐  │
│  │ useTelemetry │──▶│ Zustand Store│◀──│  Components            │  │
│  │   Hook       │   │  (Reactive)  │   │  (Dashboard, Gauges,   │  │
│  └──────────────┘  └──────────────┘  │   Tyres, Lap Timing,… )  │  │
│                                        └──────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

## Supported Games

### F1 2025

- **Protocol:** UDP
- **Default port:** 20777
- **Detection:** Automatic via `packet_format == 2025` in the 29-byte header
- **Telemetry setting:** Enable UDP in the game's telemetry options and set the target IP/port to this machine

### Assetto Corsa / ACC

- **Protocol:** Windows shared memory (Content Manager / Kunos Simulioni convention)
- **Platform:** Windows only
- **Detection:** Not via UDP — started manually through the "Start Assetto Corsa shared memory" command
- **Shared memory sections:**
  - `Local\acpmf_physics` — physics data (~300 Hz)
  - `Local\acpmf_graphics` — graphics data (~60 Hz)
  - `Local\acpmf_static` — static car/track info (read once)

> **Note:** On non-Windows platforms (macOS, Linux), the shared memory reader is a compile-safe stub that logs a warning and does nothing. This is expected — AC/ACC shared memory is a Windows-only feature.

## Installation

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/) (stable)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites) (system dependencies for your OS)

### Frontend

```bash
npm install
```

### Backend

No separate install needed — Rust dependencies are fetched with `cargo`.

```bash
cargo check --offline
```

## Configuration

Settings are persisted to the OS data directory:

- **macOS:** `~/Library/Application Support/f1-telemetry/settings.json`
- **Linux:** `~/.local/share/f1-telemetry/settings.json`
- **Windows:** `%APPDATA%\f1-telemetry\settings.json`

Configurable options:

| Setting | Default | Description |
|---------|---------|-------------|
| `udpPort` | `20777` | UDP listen port (F1 25) |
| `units` | `metric` | `metric` or `imperial` |
| `theme` | `f1-dark` | UI theme |
| `autoConnect` | `true` | Auto-start UDP listener on launch |
| `logLevel` | `info` | Logging verbosity |
| `sourceIpOverride` | `""` | Override detected source IP |
| `targetIp` | `""` | Forwarding target IP |
| `targetPort` | `20777` | Forwarding target port |
| `enableForwarding` | `false` | Enable UDP packet forwarding |
| `acShmEnabled` | `false` | Enable AC/ACC shared memory reader (Windows) |
| `acShmPath` | `""` | Shared memory section path (Content Manager default) |
| `cmConnection` | `""` | Content Manager HTTP address (optional) |

### Assetto Corsa / ACC (Windows shared memory)

The app can read AC/ACC telemetry via Windows shared memory sections instead of UDP:

| Section | Rate | Contents |
|---------|------|----------|
| `Local\acpmf_physics` | ~300 Hz | Physics (speed, RPM, gear, suspension, tyres, …) |
| `Local\acpmf_graphics` | ~60 Hz | Graphics (car position, lap, session, …) |
| `Local\acpmf_static` | Once | Static car/track info |

**To use:**
1. Launch AC or ACC through **Content Manager**.
2. In the app Settings → Assetto Corsa / ACC, enable **shared memory reader**.
3. Optionally set the **Content Manager Connection** HTTP address (e.g. `http://localhost:8080`) so the app can read the active session/car from Content Manager.
4. Shared memory path defaults to `Local\acpmf_*`; change **Shared Memory Path** only if you use a custom layout.

> **Note:** Shared memory is Windows-only. On macOS/Linux the reader is a compile-safe stub that logs a warning and does nothing.

Sessions are stored as JSON files under:

- **macOS:** `~/Library/Application Support/race-telemetry/sessions/`
- **Linux:** `~/.local/share/race-telemetry/sessions/`
- **Windows:** `%APPDATA%\race-telemetry\sessions\`

## Usage

```bash
# Start frontend dev server (port 1420)
npm run dev

# Start full Tauri app (frontend + Rust backend)
npm run tauri:dev

# Build frontend for production
npm run build

# Build Tauri application binary
cargo tauri build

# Lint frontend
npm run lint

# Check Rust compilation
cargo check --offline
```

Once the app is running:

1. Enable UDP telemetry in F1 2025 game settings (port 20777, target IP = this machine)
2. Start the app — the UDP listener binds to port 20777 automatically
3. Telemetry events begin flowing once the game sends packets
4. Use the Settings panel to change port, units, theme, or enable forwarding
5. Sessions are auto-persisted when the game session changes; use "Save current session" to save manually

## Data Flow

```
F1 2025 game (UDP 20777)
        │
        ▼
Rust UDP listener (tokio, port 20777)
        │
        ▼
nom parser (16 packet types → TelemetryEvent)
        │
        ▼
Broadcaster (~60 FPS telemetry, 1 Hz connection status)
        │
        ▼
Tauri event: "telemetry-event"
        │
        ▼
useTelemetry hook → Zustand store
        │
        ▼
React components (selectors, no full re-renders)
```

## UDP Packets

F1 2025 sends 16 packet types. The following are parsed and used by the UI:

| ID | Packet | Size | UI Usage |
|----|--------|------|----------|
| 1 | Session | 753 B | Track, weather, session type, time |
| 2 | LapData | 1285 B | Lap times, sectors, position |
| 3 | Event | variable | Session start/end, flags |
| 4 | Participants | 1284 B | Driver names, teams, colors |
| 6 | CarTelemetry | 1352 B | RPM, speed, throttle, brake, gear, DRS |
| 7 | CarStatus | 1239 B | Fuel, ERS, engine mode, brake bias |
| 10 | CarDamage | 1041 B | Tyre wear, damage indicators |
| 12 | TyreSets | 231 B | Available tyre sets |
| 13 | MotionEx | 273 B | Suspension data |
| 11 | SessionHistory | 1460 B | Lap history, tyre stints |

Unparsed packet types (0, 5, 8, 9, 14, 15) are accepted but not displayed.

## Game Detection

The backend detects which game is sending data on every UDP packet:

- **F1 2025:** `packet_format == 2025` (u16 LE at header offset 0)
- **Other games:** No UDP detection (shared memory only)

When the detected game changes:

1. The previous session is persisted to disk (if one existed)
2. Session state is reset: lap history, best lap, current lap, session UID
3. `is_new_session` is set to `true`
4. Frontend receives a `GameDetected` event and resets its UI state

Session boundaries within the same game are detected via `m_session_uid` changes.

## Persistence

Completed sessions are auto-saved when:

- The game session changes (`m_session_uid` differs)
- The user clicks "Save current session"

Each session is stored as a JSON file named `{session_uid}_{started_at_ms}.json` containing:

- Session metadata (track, type, weather, laps, timestamps)
- Participants (driver names, teams, colors)
- Lap history with sector times and validity
- Best lap and sector times

Commands: `list_sessions`, `get_session`, `delete_session`, `save_current_session`.

## Development

### Adding a new packet type

1. Add the struct to `src-tauri/src/telemetry/packets.rs`
2. Add the parser function to `src-tauri/src/telemetry/parser.rs`
3. Register the dispatch in `parse_packet()`
4. Add a variant to the `TelemetryEvent` enum if needed
5. Handle the event in `src/hooks/useTelemetry.ts`
6. Update `src/types/telemetry.ts` and `src/stores/telemetryStore.ts`

### Adding a new Tauri command

1. Add the function with `#[tauri::command]` in `src-tauri/src/commands.rs`
2. Register it in `src-tauri/src/main.rs` `invoke_handler`
3. Call from frontend with `invoke('command_name', { args })`

### Code style

**Rust:** `#[repr(C, packed)]` for wire-format structs, `nom` for parsing, `thiserror` for errors, `tracing` for logging, `serde` for serialization.

**TypeScript/React:** Functional components + hooks, Zustand for state, CSS variables from `src/styles/f1-theme.css`, selectors for store subscriptions, `uPlot` for charts.

## Testing

```bash
# Rust unit tests
cargo test --offline

# TypeScript type check
npx tsc --noEmit
```

There is no end-to-end test with a real F1 2025 game (no game available in this environment). Tests cover the parser, game detection, and shared memory stub.

## Project Structure

```
F1Telemetry/
├── README.md
├── package.json
├── tsconfig.json
├── vite.config.ts
├── eslint.config.js
├── index.html
├── public/
├── src/                          # React frontend
│   ├── components/               # Dashboard, gauges, tyres, timing, status
│   ├── hooks/                    # useTelemetry, useSessions
│   ├── stores/                   # telemetryStore (Zustand)
│   ├── types/                    # telemetry.ts (TypeScript interfaces)
│   ├── utils/                    # unitConversion, formatLap
│   ├── styles/                   # f1-theme.css
│   └── App.tsx
├── src-tauri/                    # Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands.rs           # Tauri commands
│   │   └── telemetry/
│   │       ├── mod.rs
│   │       ├── packets.rs        # F1 25 packet structs
│   │       ├── parser.rs         # nom parser
│   │       ├── udp_listener.rs   # Tokio UDP socket
│   │       ├── broadcaster.rs    # SessionManager, throttling
│   │       ├── persistence.rs    # JSON session storage
│   │       ├── game_detection.rs # Multi-game detection
│   │       └── ac_shared_memory.rs # AC/ACC shared memory (Windows)
│   └── target/                   # Build output (ignored)
└── .gitignore
```

## License

MIT

---

*Generated for F1 Telemetry Monitor v0.1.0*
