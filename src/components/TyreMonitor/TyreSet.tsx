// src/components/TyreMonitor/TyreSet.tsx
import React from 'react';
import TyreCard from './TyreCard';
import { useTelemetryStore } from '../../stores/telemetryStore';
import '../../styles/f1-theme.css';

const compoundInfo: Record<string, { name: string; color: string }> = {
    'Hard (C0)': { name: 'Hard', color: '#ffffff' },
    'Medium (C1)': { name: 'Medium', color: '#fff200' },
    'Soft (C2)': { name: 'Soft', color: '#ff3333' },
    'Super Soft (C3)': { name: 'Super Soft', color: '#ff00ff' },
    'Ultra Soft (C4)': { name: 'Ultra Soft', color: '#ff6600' },
    'Hypersoft (C5)': { name: 'Hypersoft', color: '#ff0099' },
    'C6 (New)': { name: 'C6', color: '#cc00ff' },
    'Intermediate': { name: 'Inter', color: '#00cc66' },
    'Wet': { name: 'Wet', color: '#0099ff' },
};

const TyreSet: React.FC = () => {
    const carTelemetry = useTelemetryStore(state => state.carTelemetry);
    const carsTelemetry = useTelemetryStore(state => state.carsTelemetry);
    const carStatus = useTelemetryStore(state => state.carStatus);
    const carsStatus = useTelemetryStore(state => state.carsStatus);
    const carDamage = useTelemetryStore(state => state.carDamage);
    const units = useTelemetryStore(state => state.units);
    const activeDriverIndex = useTelemetryStore(state => state.session.activeDriverIndex);

    // Default tyre data - will be replaced by real data when available
    const tyrePositions = ['RL', 'RR', 'FL', 'FR'] as const;

    const tyreData = tyrePositions.map((pos, idx) => {
        const ct = carsTelemetry[activeDriverIndex] || carTelemetry;
        const cs = carsStatus[activeDriverIndex] || carStatus;
        const cd = carDamage[activeDriverIndex];

        const surfaceTemp = ct?.tyresSurfaceTemperature[idx] || 0;
        const coreTemp = ct?.tyresInnerTemperature[idx] || 0;
        const pressure = ct?.tyresPressure[idx] || 0;

        // Get compound from carStatus
        const compoundKey = cs?.actualTyreCompound || 'Medium (C1)';
        const compound = compoundInfo[compoundKey] || { name: 'Medium', color: '#fff200' };

        // Get wear from carDamage packet (per-car)
        const wear = cd?.tyresWear?.[idx] ?? 0;

        return {
            position: pos,
            surfaceTemp,
            coreTemp,
            pressure,
            wear,
            compoundName: compound.name,
            compoundColor: compound.color,
        };
    });

    return (
        <div className="tyre-set" style={{
            display: 'grid',
            gridTemplateColumns: 'repeat(4, 1fr)',
            gap: '15px',
            width: '100%'
        }}>
            {tyreData.map((data, index) => (
                <TyreCard key={index} {...data} units={units} />
            ))}
        </div>
    );
};

export default TyreSet;
