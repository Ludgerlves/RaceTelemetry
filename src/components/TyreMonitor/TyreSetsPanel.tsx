// src/components/TyreMonitor/TyreSetsPanel.tsx
// Displays available tyre sets (Packet ID 12 - TyreSets) for the active driver.
import React from 'react';
import { useTelemetryStore } from '../../stores/telemetryStore';
import '../../styles/f1-theme.css';

const compoundColors: Record<string, string> = {
    'Hard (C0)': 'var(--tyre-c0)',
    'Medium (C1)': 'var(--tyre-c1)',
    'Soft (C2)': 'var(--tyre-c2)',
    'Super Soft (C3)': 'var(--tyre-c3)',
    'Ultra Soft (C4)': 'var(--tyre-c4)',
    'Hypersoft (C5)': 'var(--tyre-c5)',
    'C6 (New)': 'var(--tyre-c6)',
    Intermediate: 'var(--tyre-inter)',
    Wet: 'var(--tyre-wet)',
};

const TyreSetsPanel: React.FC = () => {
    const tyreSets = useTelemetryStore(state => state.tyreSets);
    const activeDriverIndex = useTelemetryStore(state => state.session.activeDriverIndex);

    if (!tyreSets) {
        return null;
    }

    // Only show for the active driver
    if (tyreSets.carIdx !== activeDriverIndex) {
        return null;
    }

    const sets = tyreSets.sets;

    return (
        <div className="f1-card f1-tyre-sets">
            <div className="f1-card__header">
                <h2 className="f1-card__title">Tyre Sets (Car {tyreSets.carIdx})</h2>
            </div>
            <div className="f1-card__body">
                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 'var(--gap-md)' }}>
                    <div style={{ fontSize: '11px', color: 'var(--f1-white-muted)' }}>
                        Fitted: <span style={{ color: 'var(--f1-white)', fontWeight: 'bold' }}>{tyreSets.fittedIdx}</span>
                    </div>
                    <div style={{ fontSize: '11px', color: 'var(--f1-white-muted)' }}>
                        {sets.length} sets available
                    </div>
                </div>

                <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(140px, 1fr))', gap: 'var(--gap-sm)' }}>
                    {sets.map((s, idx) => (
                        <div key={idx} className="tyre-set-card" style={{
                            padding: 'var(--gap-sm)',
                            background: 'var(--f1-bg-secondary)',
                            border: `1px solid ${compoundColors[s.actualTyreCompound] || '#333'}`,
                            borderRadius: 'var(--radius-sm)',
                        }}>
                            <div style={{ fontSize: '10px', color: 'var(--f1-white-muted)', textTransform: 'uppercase', marginBottom: '4px' }}>
                                Set {idx + 1}
                            </div>
                            <div style={{
                                fontSize: '12px',
                                fontWeight: 'bold',
                                color: compoundColors[s.actualTyreCompound] || 'var(--f1-white)',
                                textTransform: 'uppercase',
                                marginBottom: '6px',
                            }}>
                                {s.visualTyreCompound}
                            </div>
                            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 'var(--gap-xs)', fontSize: '11px', color: 'var(--f1-white-muted)' }}>
                                <div>
                                    <div style={{ fontSize: '10px', color: '#666' }}>Wear</div>
                                    <div>{s.wear.toFixed(0)}%</div>
                                </div>
                                <div>
                                    <div style={{ fontSize: '10px', color: '#666' }}>Life</div>
                                    <div>{s.lapsDone}/{s.lifeSpan}</div>
                                </div>
                                <div>
                                    <div style={{ fontSize: '10px', color: '#666' }}>Laps Left</div>
                                    <div>{s.usableLife}</div>
                                </div>
                                <div>
                                    <div style={{ fontSize: '10px', color: '#666' }}>Available</div>
                                    <div>{s.available ? 'Yes' : 'No'}</div>
                                </div>
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
};

export default TyreSetsPanel;
