// src/components/TelemetryGauges/Suspension.tsx
// Displays suspension telemetry (Packet ID 13 - MotionEx) for the active driver.
import React from 'react';
import { useTelemetryStore } from '../../stores/telemetryStore';
import '../../styles/f1-theme.css';

const wheelLabels: Record<number, string> = { 0: 'RL', 1: 'RR', 2: 'FL', 3: 'FR' };

const Suspension: React.FC = () => {
    const motionEx = useTelemetryStore(state => state.motionEx);
    const activeDriverIndex = useTelemetryStore(state => state.session.activeDriverIndex);

    const motion = motionEx[activeDriverIndex];

    if (!motion) {
        return (
            <div className="telemetry-card suspension-card">
                <h3 style={{ borderBottom: '1px solid #333', paddingBottom: '10px' }}>Suspension</h3>
                <div className="label">Waiting for MotionEx data...</div>
            </div>
        );
    }

    const formatPos = (v: number) => v.toFixed(3);
    const formatVel = (v: number) => v.toFixed(2);
    const formatCamber = (v: number) => v.toFixed(2);

    return (
        <div className="telemetry-card suspension-card">
            <h3 style={{ borderBottom: '1px solid #333', paddingBottom: '10px' }}>Suspension (Car {activeDriverIndex})</h3>

            {/* Per-wheel suspension */}
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: '10px', marginBottom: '15px' }}>
                {[0, 1, 2, 3].map((wheelIdx) => (
                    <div key={wheelIdx} style={{
                        padding: '8px',
                        background: '#1a1a1a',
                        borderRadius: '4px',
                        border: '1px solid #333',
                    }}>
                        <div style={{ fontSize: '10px', color: '#888', textTransform: 'uppercase', marginBottom: '6px' }}>
                            {wheelLabels[wheelIdx]}
                        </div>
                        <div style={{ fontSize: '11px', color: 'var(--f1-white-muted)', marginBottom: '2px' }}>Pos</div>
                        <div style={{ fontSize: '13px', color: 'var(--f1-white)', fontFamily: 'monospace' }}>{formatPos(motion.suspensionPosition[wheelIdx])}</div>
                        <div style={{ fontSize: '11px', color: 'var(--f1-white-muted)', marginBottom: '2px' }}>Vel</div>
                        <div style={{ fontSize: '13px', color: 'var(--f1-white)', fontFamily: 'monospace' }}>{formatVel(motion.suspensionVelocity[wheelIdx])}</div>
                        <div style={{ fontSize: '11px', color: 'var(--f1-white-muted)', marginBottom: '2px' }}>Camber</div>
                        <div style={{ fontSize: '13px', color: 'var(--f1-white)', fontFamily: 'monospace' }}>{formatCamber(motion.wheelCamber[wheelIdx])}</div>
                    </div>
                ))}
            </div>

            {/* Chassis data */}
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '10px', marginBottom: '15px' }}>
                <div style={{ padding: '8px', background: '#1a1a1a', borderRadius: '4px', border: '1px solid #333' }}>
                    <div style={{ fontSize: '10px', color: '#888', marginBottom: '4px' }}>Front Wheels Angle</div>
                    <div style={{ fontSize: '14px', color: 'var(--f1-white)', fontFamily: 'monospace' }}>{motion.frontWheelsAngle.toFixed(2)}°</div>
                </div>
                <div style={{ padding: '8px', background: '#1a1a1a', borderRadius: '4px', border: '1px solid #333' }}>
                    <div style={{ fontSize: '10px', color: '#888', marginBottom: '4px' }}>Chassis Pitch</div>
                    <div style={{ fontSize: '14px', color: 'var(--f1-white)', fontFamily: 'monospace' }}>{motion.chassisPitch.toFixed(2)}°</div>
                </div>
                <div style={{ padding: '8px', background: '#1a1a1a', borderRadius: '4px', border: '1px solid #333' }}>
                    <div style={{ fontSize: '10px', color: '#888', marginBottom: '4px' }}>Wheel Slip (max)</div>
                    <div style={{ fontSize: '14px', color: 'var(--f1-white)', fontFamily: 'monospace' }}>
                        {Math.max(...motion.wheelSlip).toFixed(2)}
                    </div>
                </div>
            </div>

            {/* Local velocity */}
            <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '11px', color: 'var(--f1-white-muted)' }}>
                <div>
                    <div style={{ fontSize: '10px', color: '#666', marginBottom: '2px' }}>Local Vel X</div>
                    <div style={{ fontFamily: 'monospace' }}>{motion.localVelocityX.toFixed(1)}</div>
                </div>
                <div>
                    <div style={{ fontSize: '10px', color: '#666', marginBottom: '2px' }}>Local Vel Y</div>
                    <div style={{ fontFamily: 'monospace' }}>{motion.localVelocityY.toFixed(1)}</div>
                </div>
                <div>
                    <div style={{ fontSize: '10px', color: '#666', marginBottom: '2px' }}>Local Vel Z</div>
                    <div style={{ fontFamily: 'monospace' }}>{motion.localVelocityZ.toFixed(1)}</div>
                </div>
            </div>
        </div>
    );
};

export default Suspension;
