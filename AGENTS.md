# F1 Telemetry Monitor - Agent Guide

> **Project:** F1 2025 Telemetry Monitor
> **Stack:** Tauri 2.0 (React 18 + TypeScript frontend, Rust backend)
> **Purpose:** Real-time F1 2025 game telemetry monitoring via UDP

---

## 1. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                      F1 2025 Game                               │
│                    (UDP Port 20777)                             │
└──────────────────────────┬──────────────────────────────────────┘
                           │ UDP Packets (16 types)
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  Rust Backend (src-tauri/)                                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│  │ UDP Listener │──▶│   Parser     │──▶│   Broadcaster        │  │
│  │ (tokio)      │   │ (nom)        │   │ (SessionManager)     │  │
│  └──────────────┘  └──────────────┘  └──────────┬─────────────┘  │
│                                                  │ Tauri Events    │
│  ┌──────────────────────────────────────────────┘                 │
│  │ Tauri Commands (start/stop, settings, export, session info)  │
│  └──────────────────────────────────────────────────────────────┘  │
└──────────────────────────┬──────────────────────────────────────┘
                           │ Tauri Events / Commands
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  React Frontend (src/)                                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│  │ useTelemetry │──▶│ Zustand Store│◀──│  Components          │  │
│  │ Hook         │   │ (Reactive)   │   │ (Dashboard, Gauges,  │  │
│  └──────────────┘  └──────────────┘   │  Timing, Tyres, etc) │  │
│                                        └──────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. Key Directories & Files

### Rust Backend (`src-tauri/src/`)
| File | Purpose |
|------|---------|
| `main.rs` | App entry, Tauri setup, command registration, auto-starts UDP listener |
| `commands.rs` | Tauri commands: `start_telemetry`, `get_session_info`, `export_session`, `get_settings`, `save_settings`, etc. |
| `telemetry/mod.rs` | Module exports |
| `telemetry/packets.rs` | **F1 25 UDP packet structs** (16 packet types, enums, constants) |
| `telemetry/parser.rs` | **nom-based parser** for all 16 packet types → `TelemetryEvent` enum |
| `telemetry/udp_listener.rs` | Tokio UDP socket on port 20777, parses packets, sends to broadcaster |
| `telemetry/broadcaster.rs` | `SessionManager` (global state), throttled event emission to frontend |

### Frontend (`src/`)
| File | Purpose |
|------|---------|
| `types/telemetry.ts` | TypeScript interfaces mirroring Rust structs (`CarTelemetryDataFrontend`, `CarStatusDataFrontend`, `SessionStateFrontend`) |
| `stores/telemetryStore.ts` | **Zustand store** - single source of truth for UI state |
| `hooks/useTelemetry.ts` | Connects Tauri events → store, handles all packet types |
| `components/Dashboard.tsx` | Main layout, composes all gauge/timing/tyre/status components |
| `components/TelemetryGauges/*.tsx` | RPM, Speed/Gear, PedalBars, DRS/ERS gauges |
| `components/TyreMonitor/*.tsx` | 4-wheel tyre cards (temp, pressure, wear, compound) |
| `components/LapTiming/LapTiming.tsx` | uPlot chart + sector times + delta to best |
| `components/CarStatus/CarStatus.tsx` | Fuel, ERS, engine mode, brake bias, tyre strategy |
| `components/SessionInfo/SessionInfo.tsx` | Track, weather, session time, driver selector |
| `components/SettingsPanel/SettingsPanel.tsx` | UDP port, units, theme, logging, export/clear data |
| `styles/f1-theme.css` | CSS variables for F1 dark theme (red/green/yellow zones) |

---

## 3. F1 25 UDP Packet Specification

**Port:** `20777` (default, configurable)

**Packet Format:** Little-endian, 29-byte header + payload

**Header (29 bytes):**
```rust
struct PacketHeader {
    m_packet_format: u16,      // 2025
    m_game_year: u8,           // 25
    m_game_major_version: u8,
    m_game_minor_version: u8,
    m_packet_version: u8,
    m_packet_id: u8,           // 0-15 (see PacketId enum)
    m_session_uid: u64,
    m_session_time: f32,
    m_frame_identifier: u32,
    m_overall_frame_identifier: u32,
    m_player_car_index: u8,    // Critical for player data extraction
    m_secondary_player_car_index: u8,
}
```

**Packet IDs (16 types):**
| ID | Packet | Size | Parsed | Frontend Use |
|----|--------|------|--------|--------------|
| 0 | Motion | 1349 | ❌ | — |
| 1 | **Session** | 753 | ✅ | Track, weather, session type, time |
| 2 | **LapData** | 1285 | ✅ | Lap times, sectors, position |
| 3 | Event | variable | ✅ | Session start/end, flags |
| 4 | **Participants** | 1284 | ✅ | Driver names, teams, colors |
| 5 | CarSetups | 1133 | ❌ | — |
| 6 | **CarTelemetry** | 1352 | ✅ | **RPM, speed, throttle, brake, gear, DRS, tyre temps/pressures** |
| 7 | **CarStatus** | 1239 | ✅ | **Fuel, ERS, engine mode, brake bias, tyre compound/age** |
| 8 | FinalClassification | 1042 | ❌ | — |
| 9 | LobbyInfo | 954 | ❌ | — |
| 10 | **CarDamage** | 1041 | ✅ | Tyre wear (not yet used in UI) |
| 11 | SessionHistory | 1460 | ❌ | — |
| 12 | **TyreSets** | 231 | ✅ | Available tyre sets (not yet used in UI) |
| 13 | **MotionEx** | 273 | ✅ | Suspension data (not yet used in UI) |
| 14 | TimeTrial | variable | ❌ | — |
| 15 | LapPositions | variable | ✅ | Lap distance, sector (not yet used in UI) |

> **Key Packets for UI:** 1 (Session), 2 (LapData), 4 (Participants), 6 (CarTelemetry), 7 (CarStatus)

---

## 4. Data Flow

### Backend → Frontend (Events)
```rust
// broadcaster.rs emits via Tauri:
app_handle.emit_all("telemetry-event", &event)

// Event types (TelemetryEvent enum):
TelemetryEvent::Session { data: PacketSessionData }
TelemetryEvent::LapData { header, data: [LapData; 22], player_index }
TelemetryEvent::Participants { data: PacketParticipantsData }
TelemetryEvent::CarTelemetry { header, data: [CarTelemetryData; 22], player_index }
TelemetryEvent::CarStatus { header, data: [CarStatusData; 22], player_index }
TelemetryEvent::CarDamage { ... }
TelemetryEvent::TyreSets { data: PacketTyreSetsData }
TelemetryEvent::MotionEx { ... }
TelemetryEvent::LapPositions { ... }
TelemetryEvent::ConnectionStatus { connected, packet_rate, last_packet_time }
```

**Throttling (broadcaster.rs):**
- Telemetry events: **~60 FPS** (16ms)
- Connection status: **1 Hz** (1000ms)

### Frontend Event Handling (`useTelemetry.ts`)
```typescript
listen<TelemetryEventData>('telemetry-event').then((event) => {
    switch (event.type) {
        case 'Session': updateSession({...})
        case 'Participants': setParticipants([...])
        case 'CarTelemetry': setCarTelemetry({...})
        case 'CarStatus': setCarStatus({...})
        case 'LapData': updateSession({ currentLap, lastLapTime, sectorTimes })
        case 'ConnectionStatus': updateConnection({...})
    }
})
```

### Zustand Store (`telemetryStore.ts`)
```typescript
interface TelemetryState {
    session: SessionStateFrontend
    carTelemetry: CarTelemetryDataFrontend | null
    carStatus: CarStatusDataFrontend | null
    connection: { packetRate, lastPacketTime, connected }
    // Actions: updateSession, setCarTelemetry, setCarStatus, addLapHistory, setBestLap, etc.
}
```

---

## 5. Component Architecture

```
Dashboard (3-col grid)
├── Row 1: RPMGauge | SpeedGear | PedalBars | DRSERS
├── Row 2: TyreSet (4 TyreCards: RL, RR, FL, FR)
├── Row 3: LapTiming (uPlot chart + sector times + delta)
├── Row 4: CarStatus (fuel, ERS, engine, brakes, tyres, flags)
├── Row 5: SessionInfo (track, weather, time, driver selector)
└── Row 6: SettingsPanel (network, display, logging, export)
```

**Data Access Pattern:** Components subscribe to store slices via selectors:
```typescript
const rpm = useTelemetryStore(state => state.carTelemetry?.engineRpm || 0)
const carStatus = useTelemetryStore(state => state.carStatus)
```

---

## 6. Development Commands

```bash
# Frontend dev server (port 1420)
npm run dev

# Full Tauri dev (frontend + Rust backend)
npm run tauri:dev

# Build frontend
npm run build

# Build Tauri app
cargo tauri build

# Lint
npm run lint
```

**Key Config Files:**
- `vite.config.ts` - Port 1420, strictPort, esbuild minify
- `tsconfig.json` / `tsconfig.node.json` - TypeScript config
- `src-tauri/tauri.conf.json` - Window size (1600x900), CSP disabled, allowlist
- `src-tauri/Cargo.toml` - Dependencies: `tauri`, `tokio`, `nom`, `serde`, `bytes`, `tracing`

---

## 7. Common Tasks & Patterns

### Adding a New Packet Type to UI

1. **Parser** (`parser.rs`): Add `parse_packet_*` function, register in `parse_packet()`
2. **Event Enum** (`packets.rs`): Add variant to `TelemetryEvent`
3. **Broadcaster** (`broadcaster.rs`): Update `SessionManager` if state needed
4. **Frontend Types** (`types/telemetry.ts`): Add TypeScript interface
5. **Hook** (`useTelemetry.ts`): Add `case` in switch, call store action
6. **Store** (`telemetryStore.ts`): Add state field + action
7. **Component**: Create new component or extend existing

### Adding a New Gauge/Component

1. Create component in `components/` (follow existing patterns)
2. Subscribe to store via selector hooks
3. Use CSS variables from `f1-theme.css` (e.g., `var(--color-zone-green)`)
4. Add to `Dashboard.tsx` grid layout

### Modifying Packet Parsing

- **Packets definition:** `src-tauri/src/telemetry/packets.rs`
- **Parser logic:** `src-tauri/src/telemetry/parser.rs`
- Use `nom` combinators (`le_u8`, `le_u16`, `le_f32`, `le_i8`, etc.)
- Always check buffer size before parsing

### Adding Tauri Commands

1. Add function in `commands.rs` with `#[tauri::command]`
2. Register in `main.rs` invoke_handler
3. Call from frontend: `invoke('command_name', { args })`

---

## 8. Important Implementation Details

### Player Car Index
- **Critical:** All per-car data arrays are 22 elements; use `m_player_car_index` from header to get player data
- Frontend receives `player_index` in events, uses it to extract player car data

### Session Detection
- New session detected when `m_session_uid` changes (broadcaster.rs:89)
- Resets: lap history, best lap, current lap

### Lap History Logic
- **Backend** (`broadcaster.rs:132-173`): Detects lap completion via `m_current_lap_num` increase
- **Frontend** (`useTelemetry.ts:140-176`): Also detects new laps, adds to history, updates best lap
- Both exist; frontend is authoritative for UI

### Tyre Temperature Colors (TyreCard.tsx)
```typescript
< 70°C  → Blue (cold)
70-85°C → Green (cool)
85-100°C → Yellow (optimal)
100-115°C → Orange (hot)
> 115°C → Red (overheating)
```

### ERS Energy
- Max: **4 MJ** (4,000,000 J)
- Displayed as percentage with color coding

### RPM Gauge Zones (RPMGauge.tsx)
- 0-70% max RPM: Green
- 70-85%: Yellow
- 85-100%: Red
- Needle: 0-270° rotation, 0.05s transition

### Settings Persistence
- Stored in OS data dir: `~/Library/Application Support/f1-telemetry/settings.json` (macOS)
- Loaded via `get_settings` command, saved via `save_settings`

---

## 9. Known Gaps / TODOs

| Area | Status | Notes |
|------|--------|-------|
| CarDamage tyre wear → UI | ❌ | Parsed but not displayed |
| TyreSets data → UI | ❌ | Parsed but not displayed |
| MotionEx suspension → UI | ❌ | Parsed but not displayed |
| SessionHistory → UI | ❌ | Not parsed |
| CarSetups → UI | ❌ | Not parsed |
| Stop telemetry command | 🟡 | Stub only (needs handle management) |
| Clear session data | 🟡 | Stub only |
| Multi-driver telemetry | 🟡 | Driver selector exists but backend only returns current player |
| Unit conversion (metric/imperial) | 🟡 | Settings exist but not applied in components |

---

## 10. Debugging Tips

**Check UDP packets arriving:**
```bash
# macOS/Linux
sudo tcpdump -i any -n udp port 20777 -X
```

**Frontend console:**
- Open DevTools (Cmd+Option+I in Tauri dev)
- Watch for `telemetry-event` logs (commented out in useTelemetry.ts:36)

**Rust logs:**
```bash
RUST_LOG=debug cargo tauri dev
```

**Common Issues:**
- **No data:** Game not sending UDP (check F1 25 settings → Telemetry → UDP On, Port 20777)
- **Wrong port:** SettingsPanel allows port change, requires restart
- **Firewall:** Allow incoming UDP on port 20777
- **Packet format mismatch:** Game version must match `m_packet_format: 2025`

---

## 11. Code Style Guidelines

### Rust
- Use `#[repr(C, packed)]` for packet structs matching wire format
- `nom` for parsing, `thiserror` for errors
- `tracing` for logging (debug/info/warn/error)
- `serde` for Tauri command serialization

### TypeScript/React
- Functional components + hooks
- Zustand for global state (no Context API)
- CSS variables for theming (no hardcoded colors)
- Selectors for store subscriptions (avoid full state re-renders)
- `uPlot` for charts (lightweight, no React wrapper needed)

### F1 Theme Colors (from f1-theme.css)
```css
--color-accent-red: #e10600;      /* F1 Red */
--color-zone-green: #00ff00;      /* Optimal/Good */
--color-zone-yellow: #ffff00;     /* Warning/Caution */
--color-zone-red: #ff0000;        /* Danger/Bad */
--tyre-c0: #ffffff;  --tyre-c1: #fff200;  --tyre-c2: #ff3333;  /* etc */
```

---

## 12. Quick Reference: Key Types

### Frontend Interfaces (`types/telemetry.ts`)
```typescript
interface CarTelemetryDataFrontend {
    speed: number;           // km/h
    throttle: number;        // 0.0-1.0
    brake: number;           // 0.0-1.0
    steering: number;        // -1.0-1.0
    gear: number;            // -1=R, 0=N, 1-8
    engineRpm: number;
    drs: 'NotAvailable' | 'Available' | 'Active';
    revLightsPercent: number;
    brakesTemperature: number[];      // [RL, RR, FL, FR]
    tyresSurfaceTemperature: number[]; // [RL, RR, FL, FR]
    tyresInnerTemperature: number[];   // [RL, RR, FL, FR]
    engineTemperature: number;
    tyresPressure: number[];           // [RL, RR, FL, FR] PSI
    surfaceType: number[];
}

interface CarStatusDataFrontend {
    fuelInTank: number;         // kg
    fuelCapacity: number;       // kg
    fuelRemainingLaps: number;
    fuelMix: 'Lean' | 'Standard' | 'Rich';
    ersStoreEnergy: number;     // Joules
    ersDeployMode: 'None' | 'Medium' | 'Hotlap' | 'Overtake';
    ersHarvestedThisLapTotal: number;
    ersDeployedThisLapTotal: number;
    frontBrakeBias: number;     // percentage * 100
    actualTyreCompound: string; // e.g., "Hard (C0)"
    visualTyreCompound: string; // e.g., "Soft"
    tyresAgeLaps: number;
    maxRpm: number;
    idleRpm: number;
    drsAllowed: number;
    pitLimiterStatus: number;
    vehicleFiaFlags: number;
}

interface SessionStateFrontend {
    trackName: string;
    sessionType: string;
    sessionTimeLeft: number;
    weather: string;
    trackTemp: number;
    airTemp: number;
    currentLap: number;
    lastLapTime: number;
    bestLapTime: number;
    sectorTimes: [number, number, number];
    bestSectorTimes: [number, number, number];
    lapHistory: LapHistoryEntry[];
    participants: ParticipantInfo[];
    playerCarIndex: number;
    // ...
}
```

---

## 13. Agent Instructions

**When working on this project:**

1. **Read first** - Check existing patterns in similar files before adding new code
2. **Follow the data flow** - Backend parses → Broadcaster emits → Frontend hook receives → Store updates → Components re-render
3. **Use selectors** - Never subscribe to entire Zustand store in components
4. **CSS variables only** - No hardcoded colors; use `var(--color-zone-*)` etc.
5. **Packet IDs matter** - Always verify packet ID matches F1 25 spec before parsing
6. **Player index is critical** - All per-car arrays need `m_player_car_index` to get player data
7. **Test with real game** - UDP packets only come from running F1 25 with telemetry enabled
8. **Keep types in sync** - Rust structs ↔ TypeScript interfaces must match

**For questions about:**
- Packet parsing → `src-tauri/src/telemetry/parser.rs` + `packets.rs`
- UI state → `src/stores/telemetryStore.ts` + `src/hooks/useTelemetry.ts`
- Component structure → `src/components/Dashboard.tsx` (composition root)
- Tauri commands → `src-tauri/src/commands.rs`
- Styling → `src/styles/f1-theme.css`

---

*Generated for F1 Telemetry Monitor v0.1.0*