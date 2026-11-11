// src/components/LapHistory/LapHistoryRow.tsx
// Single row in the lap history table.
import React from 'react';
import { LapHistoryEntry } from '../../types/telemetry';
import { formatLapTime, formatSectorTime, formatDeltaToBest, isBestLap } from '../../utils/formatLap';

interface LapHistoryRowProps {
    lap: LapHistoryEntry;
    bestLapTime: number;
}

const LapHistoryRow: React.FC<LapHistoryRowProps> = ({ lap, bestLapTime }) => {
    const isBest = isBestLap(lap.lapTimeMs, bestLapTime);
    const delta = formatDeltaToBest(lap.lapTimeMs, bestLapTime);

    return (
        <div
            className="lap-row"
            style={{
                display: 'grid',
                gridTemplateColumns: '60px 1fr 1fr 1fr 1fr 100px',
                gap: '8px',
                padding: '6px 10px',
                background: isBest
                    ? 'rgba(0, 214, 58, 0.12)'
                    : !lap.isValid
                        ? 'rgba(225, 6, 0, 0.08)'
                        : 'var(--f1-bg-secondary)',
                border: `1px solid ${isBest ? 'var(--f1-green-sector)' : !lap.isValid ? 'var(--f1-red)' : 'var(--f1-border)'}`,
                borderRadius: 'var(--radius-sm)',
                fontSize: '12px',
                fontFamily: 'var(--font-mono)',
                alignItems: 'center',
            }}
        >
            <div
                style={{
                    fontWeight: 'bold',
                    color: isBest ? 'var(--f1-green-sector)' : 'var(--f1-white)',
                    textAlign: 'center',
                }}
            >
                #{lap.lapNumber}
            </div>
            <div
                style={{
                    color: isBest ? 'var(--f1-green-sector)' : 'var(--f1-white)',
                    fontWeight: isBest ? 'bold' : 'normal',
                }}
            >
                {formatLapTime(lap.lapTimeMs)}
            </div>
            <div style={{ color: 'var(--f1-white-muted)' }}>{formatSectorTime(lap.sector1Ms)}</div>
            <div style={{ color: 'var(--f1-white-muted)' }}>{formatSectorTime(lap.sector2Ms)}</div>
            <div style={{ color: 'var(--f1-white-muted)' }}>{formatSectorTime(lap.sector3Ms)}</div>
            <div
                style={{
                    color: lap.lapTimeMs > bestLapTime ? 'var(--f1-red)' : 'var(--f1-green-sector)',
                    textAlign: 'right',
                    fontWeight: 'bold',
                }}
            >
                {delta}
            </div>
        </div>
    );
};

export default LapHistoryRow;
