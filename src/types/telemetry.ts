// src/types/telemetry.ts

// Re-export the Rust structs and enums for frontend use
// Using local types instead of importing from Rust for now
export type TelemetryEventData = any;
export type TelemetryHeader = any;

// Lap history entry type
export interface LapHistoryEntry {
    lapNumber: number;
    lapTimeMs: number;
    sector1Ms: number;
    sector2Ms: number;
    sector3Ms: number;
    isValid: boolean;
    timestamp: number;
}

// Simplified frontend state based on the Rust structs
export interface CarTelemetryDataFrontend {
    speed: number; // km/h
    throttle: number; // 0.0-1.0
    brake: number; // 0.0-1.0
    steering: number; // -1.0-1.0
    gear: number;
    engineRpm: number;
    drs: 'NotAvailable' | 'Available' | 'Active';
    revLightsPercent: number;
    brakesTemperature: number[];
    tyresSurfaceTemperature: number[];
    tyresInnerTemperature: number[];
    engineTemperature: number;
    tyresPressure: number[];
    surfaceType: number[];
}

export interface CarStatusDataFrontend {
    tractionControl: number;
    antiLockBrakes: number;
    fuelMix: 'Lean' | 'Standard' | 'Rich';
    frontBrakeBias: number;
    pitLimiterStatus: number;
    fuelInTank: number; // kg
    fuelCapacity: number; // kg
    fuelRemainingLaps: number;
    maxRpm: number;
    idleRpm: number;
    maxGears: number;
    drsAllowed: number;
    drsActivationDistance: number;
    actualTyreCompound: string; // e.g., "Hard (C0)"
    visualTyreCompound: string; // e.g., "Soft"
    tyresAgeLaps: number;
    vehicleFiaFlags: number;
    ersStoreEnergy: number; // Joules
    ersDeployMode: 'None' | 'Medium' | 'Hotlap' | 'Overtake';
    ersHarvestedThisLapMguk: number;
    ersDeployedThisLapMguk: number;
    ersHarvestedThisLapMguh: number;
    ersDeployedThisLapMguh: number;
    ersHarvestedThisLapTotal: number;
    ersDeployedThisLapTotal: number;
}

// Simplified session state for the UI
export interface SessionStateFrontend {
    sessionUid: number;
    trackName: string;
    sessionType: string;
    sessionTimeLeft: number;
    sessionDuration: number;
    weather: string;
    trackTemp: number;
    airTemp: number;
    totalLaps: number;
    playerCarIndex: number;
    numActiveCars: number;
    participants: Array<{
        carIndex: number;
        name: string;
        teamId: number;
        raceNumber: number;
        aiControlled: boolean;
        yourTelemetry: boolean;
        carColorPrimary: number;
        carColorSecondary: number;
    }>;
    currentLap: number;
    bestLapTime: number;
    lastLapTime: number;
    sectorTimes: [number, number, number];
    bestSectorTimes: [number, number, number];
    lapHistory: LapHistoryEntry[];
    isNewSession: boolean;
    activeDriverIndex: number;
    sessionStartedAt: number | null;
}

// Persisted session record (mirrors Rust persistence::SessionRecord / SessionListItem)
export interface SessionListItem {
    id: string;
    sessionUid: number;
    trackName: string;
    sessionType: string;
    startedAt: number;
    durationMs: number;
    bestLapTime: number;
    totalLaps: number;
    weather: string;
}

export interface SessionRecord {
    id: string;
    sessionUid: number;
    trackName: string;
    sessionType: string;
    sessionTimeLeft: number;
    sessionDuration: number;
    weather: string;
    trackTemp: number;
    airTemp: number;
    totalLaps: number;
    playerCarIndex: number;
    activeDriverIndex: number;
    numActiveCars: number;
    startedAt: number;
    participants: SessionStateFrontend['participants'];
    lapHistory: LapHistoryEntry[];
    bestLapTime: number;
    lastLapTime: number;
    sectorTimes: [number, number, number];
    bestSectorTimes: [number, number, number];
}

// CarDamage (Packet ID 10)
export interface CarDamageDataFrontend {
    tyresWear: number[];       // [RL, RR, FL, FR] percentage
    tyresDamage: number[];     // [RL, RR, FL, FR]
    brakesDamage: number[];    // [RL, RR, FL, FR]
    frontLeftWingDamage: number;
    frontRightWingDamage: number;
    rearWingDamage: number;
    floorDamage: number;
    diffuserDamage: number;
    sidepodDamage: number;
    drsFault: number;
    gearBoxDamage: number;
    engineDamage: number;
    engineMguHWear: number;
    engineMguKWear: number;
    engineTcWear: number;
    engineCeWear: number;
    engineIceWear: number;
    tyreBlisterRL: number;
    tyreBlisterRR: number;
    tyreBlisterFL: number;
    tyreBlisterFR: number;
}

// TyreSets (Packet ID 12)
export interface TyreSetDataFrontend {
    actualTyreCompound: string;
    visualTyreCompound: string;
    wear: number;
    available: number;
    recommendedSession: number;
    lifeSpan: number;
    usableLife: number;
    lapsDone: number;
    isNew: number;
}

export interface TyreSetsFrontend {
    carIdx: number;
    sets: TyreSetDataFrontend[];
    fittedIdx: number;
}

// MotionEx (Packet ID 13)
export interface MotionExDataFrontend {
    suspensionPosition: number[];   // [RL, RR, FL, FR]
    suspensionVelocity: number[];   // [RL, RR, FL, FR]
    suspensionAcceleration: number[]; // [RL, RR, FL, FR]
    wheelSpeed: number[];           // [RL, RR, FL, FR]
    wheelSlip: number[];            // [RL, RR, FL, FR]
    localVelocityX: number;
    localVelocityY: number;
    localVelocityZ: number;
    angularVelocityX: number;
    angularVelocityY: number;
    angularVelocityZ: number;
    angularAccelerationX: number;
    angularAccelerationY: number;
    angularAccelerationZ: number;
    frontWheelsAngle: number;
    chassisPitch: number;
    wheelCamber: number[];          // [RL, RR, FL, FR]
    wheelCamberGain: number[];      // [RL, RR, FL, FR]
}

// SessionHistory (Packet ID 11)
export interface LapHistoryDataFrontend {
    lapTimeInMs: number;
    sector1TimeInMs: number;
    sector2TimeInMs: number;
    sector3TimeInMs: number;
    lapValidBitFlags: number;
}

export interface TyreStintHistoryDataFrontend {
    endLap: number;
    tyreActualCompound: string;
    tyreVisualCompound: string;
}

export interface SessionHistoryDataFrontend {
    carIdx: number;
    numLaps: number;
    numTyreStints: number;
    bestLapTimeLapNum: number;
    bestSector1LapNum: number;
    bestSector2LapNum: number;
    bestSector3LapNum: number;
    lapHistory: LapHistoryDataFrontend[];
    tyreStintHistory: TyreStintHistoryDataFrontend[];
}
