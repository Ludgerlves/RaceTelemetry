// src/components/SessionHistory/SessionHistoryPanel.tsx
// Exibe dados do pacote SessionHistory (ID 11) para o driver ativo:
// melhor volta oficial, histórico de voltas e stints de pneus.
import React from 'react';
import { useTelemetryStore } from '../../stores/telemetryStore';
import { formatLapTime, formatSectorTime } from '../../utils/formatLap';
import '../../styles/f1-theme.css';

interface SessionHistoryPanelProps {}

const SessionHistoryPanel: React.FC<SessionHistoryPanelProps> = () => {
    const history = useTelemetryStore(state => state.sessionHistory);
    const activeDriverIndex = useTelemetryStore(state => state.session.activeDriverIndex);

    if (!history) {
        return (
            <div className="telemetry-card session-history-panel">
                <h3 style={{ borderBottom: '1px solid #333', paddingBottom: '10px' }}>Session History</h3>
                <div className="label">Waiting for SessionHistory data...</div>
            </div>
        );
    }

    // Only show for the active driver
    const isActiveDriver = history.carIdx === activeDriverIndex;

    return (
        <div className="telemetry-card session-history-panel">
            <h3 style={{ borderBottom: '1px solid #333', paddingBottom: '10px' }}>
                Session History (Car {history.carIdx})
                {isActiveDriver ? ' — ACTIVE' : ''}
            </h3>

            <div style={{ marginBottom: '15px' }}>
                <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '10px' }}>
                    <div style={{ padding: '8px', background: '#1a1a1a', borderRadius: '4px', border: '1px solid #333' }}>
                        <div style={{ fontSize: '10px', color: '#888', textTransform: 'uppercase', marginBottom: '4px' }}>Best Lap</div>
                        <div style={{ fontSize: '14px', color: 'var(--f1-green-sector)', fontFamily: 'monospace', fontWeight: 'bold' }}>
                            {formatLapTime(history.bestLapTimeLapNum > 0 ? history.bestLapTimeLapNum : 0)}
                        </div>
                        <div style={{ fontSize: '11px', color: '#666', fontFamily: 'monospace' }}>
                            Lap #{history.bestLapTimeLapNum}
                        </div>
                    </div>
                    <div style={{ padding: '8px', background: '#1a1a1a', borderRadius: '4px', border: '1px solid #333' }}>
                        <div style={{ fontSize: '10px', color: '#888', textTransform: 'uppercase', marginBottom: '4px' }}>Laps</div>
                        <div style={{ fontSize: '14px', color: 'var(--f1-white)', fontFamily: 'monospace', fontWeight: 'bold' }}>
                            {history.numLaps}
                        </div>
                    </div>
                    <div style={{ padding: '8px', background: '#1a1a1a', borderRadius: '4px', border: '1px solid #333' }}>
                        <div style={{ fontSize: '10px', color: '#888', textTransform: 'uppercase', marginBottom: '4px' }}>Tyre Stints</div>
                        <div style={{ fontSize: '14px', color: 'var(--f1-white)', fontFamily: 'monospace', fontWeight: 'bold' }}>
                            {history.numTyreStints}
                        </div>
                    </div>
                </div>
            </div>

            {/* Best Sectors */}
            <div style={{ marginBottom: '15px' }}>
                <div style={{ fontSize: '11px', color: '#888', textTransform: 'uppercase', marginBottom: '8px' }}>Best Sectors</div>
                <div style={{ display: 'flex', gap: '10px' }}>
                    <div style={{ flex: 1, padding: '8px', background: '#1a1a1a', borderRadius: '4px', border: '1px solid #333', textAlign: 'center' }}>
                        <div style={{ fontSize: '10px', color: '#666' }}>S1</div>
                        <div style={{ fontSize: '13px', color: 'var(--f1-white)', fontFamily: 'monospace' }}>
                            {history.bestSector1LapNum > 0 ? formatSectorTime(history.bestSector1LapNum) : '--.---'}
                        </div>
                        <div style={{ fontSize: '10px', color: '#666', fontFamily: 'monospace' }}>Lap #{history.bestSector1LapNum}</div>
                    </div>
                    <div style={{ flex: 1, padding: '8px', background: '#1a1a1a', borderRadius: '4px', border: '1px solid #333', textAlign: 'center' }}>
                        <div style={{ fontSize: '10px', color: '#666' }}>S2</div>
                        <div style={{ fontSize: '13px', color: 'var(--f1-white)', fontFamily: 'monospace' }}>
                            {history.bestSector2LapNum > 0 ? formatSectorTime(history.bestSector2LapNum) : '--.---'}
                        </div>
                        <div style={{ fontSize: '10px', color: '#666', fontFamily: 'monospace' }}>Lap #{history.bestSector2LapNum}</div>
                    </div>
                    <div style={{ flex: 1, padding: '8px', background: '#1a1a1a', borderRadius: '4px', border: '1px solid #333', textAlign: 'center' }}>
                        <div style={{ fontSize: '10px', color: '#666' }}>S3</div>
                        <div style={{ fontSize: '13px', color: 'var(--f1-white)', fontFamily: 'monospace' }}>
                            {history.bestSector3LapNum > 0 ? formatSectorTime(history.bestSector3LapNum) : '--.---'}
                        </div>
                        <div style={{ fontSize: '10px', color: '#666', fontFamily: 'monospace' }}>Lap #{history.bestSector3LapNum}</div>
                    </div>
                </div>
            </div>

            {/* Lap History from packet */}
            <div style={{ marginBottom: '15px' }}>
                <div style={{ fontSize: '11px', color: '#888', textTransform: 'uppercase', marginBottom: '8px' }}>Lap History (Packet)</div>
                <div style={{ maxHeight: '160px', overflowY: 'auto', border: '1px solid #333', borderRadius: '4px' }}>
                    {history.lapHistory.length === 0 ? (
                        <div style={{ padding: '10px', color: 'var(--f1-white-muted)', textAlign: 'center' }}>No lap data.</div>
                    ) : (
                        history.lapHistory.map((lap, idx) => (
                            <div key={idx} style={{
                                display: 'grid',
                                gridTemplateColumns: '60px 1fr 1fr 1fr 80px',
                                gap: '6px',
                                padding: '4px 8px',
                                borderBottom: '1px solid #222',
                                fontSize: '11px',
                                fontFamily: 'monospace',
                                alignItems: 'center',
                                background: idx % 2 === 0 ? '#141414' : '#1a1a1a',
                            }}>
                                <div style={{ color: 'var(--f1-white-muted)' }}>{idx + 1}</div>
                                <div style={{ color: 'var(--f1-white)' }}>{formatLapTime(lap.lapTimeInMs)}</div>
                                <div style={{ color: '#888' }}>{formatSectorTime(lap.sector1TimeInMs)}</div>
                                <div style={{ color: '#888' }}>{formatSectorTime(lap.sector2TimeInMs)}</div>
                                <div style={{ color: '#888' }}>{formatSectorTime(lap.sector3TimeInMs)}</div>
                            </div>
                        ))
                    )}
                </div>
            </div>

            {/* Tyre Stint History */}
            <div>
                <div style={{ fontSize: '11px', color: '#888', textTransform: 'uppercase', marginBottom: '8px' }}>Tyre Stints</div>
                <div style={{ display: 'flex', gap: '10px', flexWrap: 'wrap' }}>
                    {history.tyreStintHistory.length === 0 ? (
                        <div style={{ color: 'var(--f1-white-muted)' }}>No stint data.</div>
                    ) : (
                        history.tyreStintHistory.map((stint, idx) => (
                            <div key={idx} style={{
                                padding: '6px 10px',
                                background: '#1a1a1a',
                                border: '1px solid #333',
                                borderRadius: '4px',
                                fontSize: '11px',
                                color: 'var(--f1-white)',
                            }}>
                                <div style={{ fontSize: '10px', color: '#888', textTransform: 'uppercase' }}>{stint.tyreVisualCompound}</div>
                                <div>End Lap #{stint.endLap}</div>
                            </div>
                        ))
                    )}
                </div>
            </div>
        </div>
    );
};

export default SessionHistoryPanel;
