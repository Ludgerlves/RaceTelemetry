// src/stores/telemetryStore.ts
import { create } from 'zustand';
import {
    SessionStateFrontend,
    CarTelemetryDataFrontend,
    CarStatusDataFrontend,
    LapHistoryEntry,
    SessionListItem,
    SessionRecord,
    CarDamageDataFrontend,
    TyreSetsFrontend,
    MotionExDataFrontend,
    SessionHistoryDataFrontend,
} from '../types/telemetry';
import { UnitSystem } from '../utils/unitConversion';

interface SourceIpInfo {
    ip: string | null;
    port: number | null;
}

interface TelemetryState {
    // Session state
    session: SessionStateFrontend;

    // Real-time car data (player car - kept for backwards compat)
    carTelemetry: CarTelemetryDataFrontend | null;
    carStatus: CarStatusDataFrontend | null;

    // Real-time car data (all cars, multi-driver)
    carsTelemetry: Record<number, CarTelemetryDataFrontend>;
    carsStatus: Record<number, CarStatusDataFrontend>;

    // Parsed-but-previously-unused packets
    carDamage: Record<number, CarDamageDataFrontend>;
    tyreSets: TyreSetsFrontend | null;
    motionEx: Record<number, MotionExDataFrontend>;
    sessionHistory: SessionHistoryDataFrontend | null;

    // Persisted sessions history
    sessions: SessionListItem[];
    selectedSession: SessionRecord | null;

    // Connection info
    connection: {
        packetRate: number;
        lastPacketTime: number;
        connected: boolean;
    };

    // Detected source IP (from F1 game)
    sourceIp: SourceIpInfo;

    // Unit system setting (metric | imperial)
    units: UnitSystem;

    // Actions
    updateSession: (newState: Partial<SessionStateFrontend>) => void;
    setCarTelemetry: (data: CarTelemetryDataFrontend) => void;
    setCarStatus: (data: CarStatusDataFrontend) => void;
    updateConnection: (data: { packetRate: number; lastPacketTime: number; connected: boolean }) => void;
    addLapHistory: (entry: LapHistoryEntry) => void;
    setBestLap: (lapTime: number, sectors: [number, number, number]) => void;
    setParticipants: (participants: SessionStateFrontend['participants']) => void;
    setActiveDriverIndex: (index: number) => void;
    setSourceIp: (ip: string | null, port: number | null) => void;
    setUnits: (units: UnitSystem) => void;

    // Session lifecycle
    clearSession: () => void;

    // Multi-driver car data
    setCarsTelemetry: (cars: Record<number, CarTelemetryDataFrontend>) => void;
    setCarsStatus: (cars: Record<number, CarStatusDataFrontend>) => void;

    // Parsed-but-previously-unused packets
    setCarDamage: (damage: Record<number, CarDamageDataFrontend>) => void;
    setTyreSets: (sets: TyreSetsFrontend | null) => void;
    setMotionEx: (motion: Record<number, MotionExDataFrontend>) => void;
    setSessionHistory: (history: SessionHistoryDataFrontend | null) => void;

    // Persisted sessions history
    setSessions: (sessions: SessionListItem[]) => void;
    setSelectedSession: (session: SessionRecord | null) => void;
    deleteSession: (id: string) => void;
}

export const useTelemetryStore = create<TelemetryState>((set) => ({
    session: {
        sessionUid: 0,
        trackName: 'Waiting for session...',
        sessionType: 'N/A',
        sessionTimeLeft: 0,
        sessionDuration: 0,
        weather: 'N/A',
        trackTemp: 0,
        airTemp: 0,
        totalLaps: 0,
        playerCarIndex: 0,
        numActiveCars: 0,
        participants: [],
        currentLap: 0,
        bestLapTime: Infinity,
        lastLapTime: 0,
        sectorTimes: [0, 0, 0],
        bestSectorTimes: [0, 0, 0],
        lapHistory: [],
        isNewSession: true,
        activeDriverIndex: 0,
        sessionStartedAt: null,
    },

    carTelemetry: null,
    carStatus: null,

    carsTelemetry: {},
    carsStatus: {},

    carDamage: {},
    tyreSets: null,
    motionEx: {},
    sessionHistory: null,

    connection: { packetRate: 0, lastPacketTime: 0, connected: false },

    sourceIp: { ip: null, port: null },

    units: 'metric',

    sessions: [],
    selectedSession: null,

    updateSession: (newState) => set((state) => ({
        session: { ...state.session, ...newState }
    })),

    setCarTelemetry: (data) => set({ carTelemetry: data }),

    setCarStatus: (data) => set({ carStatus: data }),

    updateConnection: (data) => set((state) => ({
        connection: { ...state.connection, ...data }
    })),

    addLapHistory: (entry) => set((state) => ({
        session: {
            ...state.session,
            lapHistory: [...state.session.lapHistory, entry].slice(-100)
        }
    })),

    setBestLap: (lapTime, sectors) => set((state) => ({
        session: {
            ...state.session,
            bestLapTime: lapTime,
            bestSectorTimes: sectors
        }
    })),

    setParticipants: (participants) => set((state) => ({
        session: { ...state.session, participants }
    })),

    setActiveDriverIndex: (index) => set((state) => ({
        session: { ...state.session, activeDriverIndex: index }
    })),

    setSourceIp: (ip, port) => set({ sourceIp: { ip, port } }),

    setUnits: (units) => set({ units }),

    clearSession: () => set((state) => ({
        session: {
            ...state.session,
            lapHistory: [],
            bestLapTime: Infinity,
            currentLap: 0,
            lastLapTime: 0,
            sectorTimes: [0, 0, 0],
            bestSectorTimes: [0, 0, 0],
        }
    })),

    // Multi-driver car data
    setCarsTelemetry: (cars: Record<number, CarTelemetryDataFrontend>) => set({ carsTelemetry: cars }),
    setCarsStatus: (cars: Record<number, CarStatusDataFrontend>) => set({ carsStatus: cars }),

    // Parsed-but-previously-unused packets
    setCarDamage: (damage: Record<number, CarDamageDataFrontend>) => set({ carDamage: damage }),
    setTyreSets: (sets: TyreSetsFrontend | null) => set({ tyreSets: sets }),
    setMotionEx: (motion: Record<number, MotionExDataFrontend>) => set({ motionEx: motion }),
    setSessionHistory: (history: SessionHistoryDataFrontend | null) => set({ sessionHistory: history }),

    // Persisted sessions history
    setSessions: (sessions: SessionListItem[]) => set({ sessions }),
    setSelectedSession: (session: SessionRecord | null) => set({ selectedSession: session }),
    deleteSession: (id: string) => set((state) => ({
        sessions: state.sessions.filter((s) => s.id !== id),
        selectedSession: state.selectedSession?.id === id ? null : state.selectedSession,
    })),
}));