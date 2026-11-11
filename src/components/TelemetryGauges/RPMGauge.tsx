// src/components/TelemetryGauges/RPMGauge.tsx
import React from 'react';
import { useTelemetryStore } from '../../stores/telemetryStore';
import '../../styles/f1-theme.css';

const RPMGauge: React.FC = () => {
    const carTelemetry = useTelemetryStore(state => state.carTelemetry);
    const rpm = carTelemetry?.engineRpm || 0;
    const maxRpm = 15000; // Default, could come from carStatus

    // Calculate rotation for needle (0 to 270 degrees for 0 to maxRpm)
    const needleRotation = Math.min((rpm / maxRpm) * 270, 270);

    return (
        <div className="gauge-container telemetry-card">
            <div className="label">RPM</div>
            <div className="value f1-red-accent">{rpm.toLocaleString()}</div>
            <div style={{
                width: '200px',
                height: '200px',
                position: 'relative',
                margin: '0 auto'
            }}>
                {/* Background arc */}
                <svg width="200" height="200" style={{ transform: 'rotate(-135deg)' }}>
                    <circle
                        cx="100" cy="100" r="90"
                        fill="none" stroke="#333" strokeWidth="20"
                        strokeDasharray="565" strokeDashoffset="0"
                    />
                    {/* Green zone */}
                    <circle
                        cx="100" cy="100" r="90"
                        fill="none" stroke="var(--color-zone-green)" strokeWidth="20"
                        strokeDasharray="565" strokeDashoffset="565"
                        strokeLinecap="round"
                        style={{
                            strokeDasharray: `${(0.7 * 565)} ${565}`,
                            strokeDashoffset: 565
                        }}
                    />
                    {/* Yellow zone */}
                    <circle
                        cx="100" cy="100" r="90"
                        fill="none" stroke="var(--color-zone-yellow)" strokeWidth="20"
                        strokeDasharray="565" strokeDashoffset="565"
                        strokeLinecap="round"
                        style={{
                            strokeDasharray: `${(0.15 * 565)} ${565}`,
                            strokeDashoffset: 565 * 0.7
                        }}
                    />
                    {/* Red zone */}
                    <circle
                        cx="100" cy="100" r="90"
                        fill="none" stroke="var(--color-zone-red)" strokeWidth="20"
                        strokeDasharray="565" strokeDashoffset="565"
                        strokeLinecap="round"
                        style={{
                            strokeDasharray: `${(0.15 * 565)} ${565}`,
                            strokeDashoffset: 565 * 0.85
                        }}
                    />
                    {/* Needle */}
                    <line
                        x1="100" y1="100"
                        x2="100" y2="10"
                        stroke="var(--color-accent-red)" strokeWidth="3"
                        strokeLinecap="round"
                        style={{
                            transformOrigin: '100px 100px',
                            transform: `rotate(${needleRotation - 135}deg)`,
                            transition: 'transform 0.05s linear'
                        }}
                    />
                </svg>
                {/* Center text */}
                <div style={{
                    position: 'absolute',
                    top: '50%',
                    left: '50%',
                    transform: 'translate(-50%, -50%)',
                    textAlign: 'center'
                }}>
                    <div style={{ fontSize: '12px', color: '#888' }}>RPM</div>
                    <div style={{ fontSize: '28px', fontWeight: 'bold', fontFamily: 'monospace' }}>
                        {Math.floor(rpm / 1000)}k
                    </div>
                </div>
            </div>
        </div>
    );
};

export default RPMGauge;
