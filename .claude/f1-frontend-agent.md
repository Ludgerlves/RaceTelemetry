---
name: f1-frontend-agent
description: Specialized agent for React/TypeScript frontend development on the F1 Telemetry Monitor (React 18, Zustand, uPlot, Tauri bindings)
metadata:
  type: reference
---

# F1 Telemetry Frontend Agent

This agent handles React/TypeScript frontend work for the F1 2025 Telemetry Monitor.

## Scope

- **App.tsx** (`src/App.tsx`): App root, live/history view toggle
- **Dashboard.tsx** (`src/components/Dashboard.tsx`): Main layout, 3-col grid composition
- **TelemetryGauges/** (`RPMGauge`, `SpeedGear`, `PedalBars`, `DRSERS`, `Suspension`): Gauge components
- **TyreMonitor/** (`TyreCard`, `TyreSet`, `TyreSetsPanel`): Tyre display components
- **LapTiming/** (`LapTiming`): uPlot lap chart + sector times
- **LapHistory/** (`LapHistoryTable`, `LapHistoryRow`): Lap history table for persisted sessions
- **CarStatus/** (`CarStatus`): Fuel, ERS, engine mode, brake bias, tyre strategy, damage indicators
- **SessionInfo/** (`SessionInfo`): Track, weather, session time, driver selector
- **SettingsPanel/** (`SettingsPanel`): UDP port, units, theme, logging, export, clear session
- **SessionHistory/** (`SessionHistory`, `SessionCard`): Sessions history browser
- **Stores** (`src/stores/telemetryStore.ts`): Zustand single source of truth
- **Hooks** (`src/hooks/useTelemetry.ts`, `src/hooks/useSessions.ts`): Tauri event → store wiring
- **Types** (`src/types/telemetry.ts`): TypeScript interfaces mirroring Rust structs
- **Utils** (`src/utils/formatLap.ts`): Shared lap formatting utilities
- **Styles** (`src/styles/f1-theme.css`): F1 dark theme CSS variables

## Key Patterns

- Functional components + hooks; Zustand for global state (no Context API)
- Components subscribe to store slices via selectors (avoid full state re-renders)
- CSS variables only — no hardcoded colors; use `var(--color-zone-*)`, `var(--tyre-*)`, etc.
- `uPlot` for charts (no React wrapper)
- All per-car data stored as `Record<number, ...>` index-keyed maps; `activeDriverIndex` selects which car to display
- Tyre positions ordered `[RL, RR, FL, FR]`
- Temperature color zones: <70°C blue, 70-85 green, 85-100 yellow, 100-115 orange, >115 red
- Wear color zones: >80% red, >50% orange, else green

## Data Flow

```
Tauri telemetry-event → useTelemetry hook → Zustand store → React components (selectors)
```

- `useTelemetry` listens to `telemetry-event` and dispatches to store actions by `payload.type`
- Multi-driver: all 22 cars stored in `carsTelemetry`/`carsStatus`/`carDamage`/`motionEx` maps
- `activeDriverIndex` (from SessionInfo dropdown → `set_active_driver` command) overrides which car's data is shown
- Sessions history: `useSessions` hook wraps `list_sessions`/`get_session`/`delete_session`/`save_current_session` Tauri commands

## Current State

- TypeScript compiles clean via `tsc --noEmit`
- Feature D (CarDamage wear, TyreSets panel, Suspension, damage indicators) wired to UI
- Feature 2 (Sessions history browser) and Feature 3 (Lap history table) implemented
- App has Live/History view toggle in topbar

## When to invoke

Use this agent for: adding new UI components, modifying store state/actions, adding Tauri event cases in `useTelemetry`, updating types to match Rust structs, styling with F1 theme, building the sessions/lap history UI.
