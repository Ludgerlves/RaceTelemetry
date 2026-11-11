// src/components/LapHistory/LapHistoryTable.tsx
// Lap history table UI (Feature 3) — shows laps of a selected session with sectors, delta to best.
import React from 'react';
import { useTelemetryStore } from '../../stores/telemetryStore';
import { formatLapTime } from '../../utils/formatLap';
import LapHistoryRow from './LapHistoryRow';
import '../../styles/f1-theme.css';

const LapHistoryTable: React.FC = () => {
    const session = useTelemetryStore(state => state.selectedSession);

    if (!session) {
        return (
            <div className="f1-card f1-lap-history">
                <div className="f1-card__header">
                    <h2 className="f1-card__title">Lap History</h2>
                </div>
                <div className="f1-card__body">
                    <div style={{ padding: 'var(--gap-md)', color: 'var(--f1-white-muted)', textAlign: 'center' }}>
                        Select a session from Session History to view its laps.
                    </div>
                </div>
            </div>
        );
    }

    const laps = [...session.lapHistory].sort((a, b) => a.lapNumber - b.lapNumber);
    const bestLapTime = session.bestLapTime === Infinity ? 0 : session.bestLapTime;

    return (
        <div className="f1-card f1-lap-history">
            <div className="f1-card__header">
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                    <h2 className="f1-card__title">Lap History — {session.trackName}</h2>
                    <div style={{ fontSize: '11px', color: 'var(--f1-white-muted)' }}>
                        {session.sessionType} • {laps.length} lap{laps.length !== 1 ? 's' : ''}
                    </div>
                </div>
            </div>
            <div className="f1-card__body">
                {/* Session summary chips */}
                <div style={{
                    display: 'flex',
                    gap: 'var(--gap-md)',
                    marginBottom: 'var(--gap-md)',
                    flexWrap: 'wrap',
                }}>
                    <div style={{ padding: '6px 12px', background: '#1a1a1a', border: '1px solid var(--f1-border)', borderRadius: 'var(--radius-sm)' }}>
                        <div style={{ fontSize: '10px', color: '#888', textTransform: 'uppercase', marginBottom: '2px' }}>Best Lap</div>
                        <div style={{ fontSize: '14px', color: 'var(--f1-green-sector)', fontFamily: 'var(--font-mono)', fontWeight: 'bold' }}>
                            {formatLapTime(bestLapTime)}
                        </div>
                    </div>
                    <div style={{ padding: '6px 12px', background: '#1a1a1a', border: '1px solid var(--f1-border)', borderRadius: 'var(--radius-sm)' }}>
                        <div style={{ fontSize: '10px', color: '#888', textTransform: 'uppercase', marginBottom: '2px' }}>Total Laps</div>
                        <div style={{ fontSize: '14px', color: 'var(--f1-white)', fontFamily: 'var(--font-mono)', fontWeight: 'bold' }}>
                            {laps.length}
                        </div>
                    </div>
                    <div style={{ padding: '6px 12px', background: '#1a1a1a', border: '1px solid var(--f1-border)', borderRadius: 'var(--radius-sm)' }}>
                        <div style={{ fontSize: '10px', color: '#888', textTransform: 'uppercase', marginBottom: '2px' }}>Valid</div>
                        <div style={{ fontSize: '14px', color: 'var(--f1-white)', fontFamily: 'var(--font-mono)', fontWeight: 'bold' }}>
                            {laps.filter(l => l.isValid).length}
                        </div>
                    </div>
                    <div style={{ padding: '6px 12px', background: '#1a1a1a', border: '1px solid var(--f1-border)', borderRadius: 'var(--radius-sm)' }}>
                        <div style={{ fontSize: '10px', color: '#888', textTransform: 'uppercase', marginBottom: '2px' }}>Invalid</div>
                        <div style={{ fontSize: '14px', color: 'var(--f1-red)', fontFamily: 'var(--font-mono)', fontWeight: 'bold' }}>
                            {laps.filter(l => !l.isValid).length}
                        </div>
                    </div>
                </div>

                {/* Table header */}
                <div style={{
                    display: 'grid',
                    gridTemplateColumns: '60px 1fr 1fr 1fr 1fr 100px',
                    gap: '8px',
                    padding: '8px 10px',
                    background: 'var(--f1-bg-tertiary)',
                    border: '1px solid var(--f1-border)',
                    borderRadius: 'var(--radius-sm)',
                    fontSize: '11px',
                    color: 'var(--f1-white-muted)',
                    textTransform: 'uppercase',
                    letterSpacing: '0.5px',
                    marginBottom: '6px',
                }}>
                    <div style={{ textAlign: 'center' }}>Lap</div>
                    <div>Time</div>
                    <div>S1</div>
                    <div>S2</div>
                    <div>S3</div>
                    <div style={{ textAlign: 'right' }}>Delta</div>
                </div>

                {/* Table body */}
                <div style={{
                    display: 'flex',
                    flexDirection: 'column',
                    gap: '2px',
                    maxHeight: '400px',
                    overflowY: 'auto',
                }}>
                    {laps.length === 0 ? (
                        <div style={{ padding: 'var(--gap-md)', color: 'var(--f1-white-muted)', textAlign: 'center' }}>
                            No laps recorded for this session.
                        </div>
                    ) : (
                        laps.map((lap) => (
                            <LapHistoryRow key={lap.lapNumber} lap={lap} bestLapTime={bestLapTime} />
                        ))
                    )}
                </div>
            </div>
        </div>
    );
};

export default LapHistoryTable;
