// src/components/TelemetryGauges/PedalBars.tsx
import React from 'react';
import { useTelemetryStore } from '../../stores/telemetryStore';
import '../../styles/f1-theme.css';

const PedalBars: React.FC = () => {
    const carTelemetry = useTelemetryStore(state => state.carTelemetry);
    const throttle = carTelemetry?.throttle || 0;
    const brake = carTelemetry?.brake || 0;

    return (
        <div className="gauge-container telemetry-card" style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center' }}>
            <div className="label">THROTTLE / BRAKE</div>
            <div style={{
                display: 'flex',
                height: '150px',
                width: '80px',
                gap: '20px',
                alignItems: 'flex-end'
            }}>
                {/* Throttle - Green */}
                <div style={{
                    width: '30px',
                    height: '100%',
                    backgroundColor: '#1a1a1a',
                    borderRadius: '15px',
                    position: 'relative',
                    border: '1px solid #333'
                }}>
                    <div style={{
                        position: 'absolute',
                        bottom: 0,
                        left: 0,
                        right: 0,
                        height: `${throttle * 100}%`,
                        backgroundColor: 'var(--color-zone-green)',
                        borderRadius: '15px',
                        transition: 'height 0.05s linear',
                        boxShadow: '0 0 10px var(--color-zone-green)'
                    }}></div>
                    <div style={{
                        position: 'absolute',
                        bottom: '-25px',
                        left: '50%',
                        transform: 'translateX(-50%)',
                        fontSize: '10px',
                        color: '#888'
                    }}>THR</div>
                </div>

                {/* Brake - Red */}
                <div style={{
                    width: '30px',
                    height: '100%',
                    backgroundColor: '#1a1a1a',
                    borderRadius: '15px',
                    position: 'relative',
                    border: '1px solid #333'
                }}>
                    <div style={{
                        position: 'absolute',
                        bottom: 0,
                        left: 0,
                        right: 0,
                        height: `${brake * 100}%`,
                        backgroundColor: 'var(--color-zone-red)',
                        borderRadius: '15px',
                        transition: 'height 0.05s linear',
                        boxShadow: '0 0 10px var(--color-zone-red)'
                    }}></div>
                    <div style={{
                        position: 'absolute',
                        bottom: '-25px',
                        left: '50%',
                        transform: 'translateX(-50%)',
                        fontSize: '10px',
                        color: '#888'
                    }}>BRK</div>
                </div>
            </div>

            <div style={{ display: 'flex', gap: '40px', marginTop: '10px' }}>
                <div style={{ textAlign: 'center' }}>
                    <div className="value" style={{ color: 'var(--color-zone-green)', fontSize: '18px' }}>
                        {Math.round(throttle * 100)}%
                    </div>
                    <div className="label">THROTTLE</div>
                </div>
                <div style={{ textAlign: 'center' }}>
                    <div className="value" style={{ color: 'var(--color-zone-red)', fontSize: '18px' }}>
                        {Math.round(brake * 100)}%
                    </div>
                    <div className="label">BRAKE</div>
                </div>
            </div>
        </div>
    );
};

export default PedalBars;
