// src/components/TelemetryGauges/DRSERS.tsx
import React from 'react';
import { useTelemetryStore } from '../../stores/telemetryStore';
import '../../styles/f1-theme.css';

const DRSERS: React.FC = () => {
    const carTelemetry = useTelemetryStore(state => state.carTelemetry);
    const carStatus = useTelemetryStore(state => state.carStatus);

    const drs = carTelemetry?.drs || 'NotAvailable';
    const ersMode = carStatus?.ersDeployMode || 'None';
    const ersEnergy = carStatus?.ersStoreEnergy || 0;
    const ersDeployed = carStatus?.ersDeployedThisLapTotal || 0;
    const ersHarvested = carStatus?.ersHarvestedThisLapTotal || 0;

    const drsColors = {
        'NotAvailable': '#444',
        'Available': 'var(--color-zone-yellow)',
        'Active': 'var(--color-accent-red)'
    };

    const ersModeColors = {
        'None': '#444',
        'Medium': '#888',
        'Hotlap': 'var(--color-zone-yellow)',
        'Overtake': 'var(--color-accent-red)'
    };

    const maxERS = 4000000; // 4MJ max
    const ersPercent = Math.min((ersEnergy / maxERS) * 100, 100);

    return (
        <div className="gauge-container telemetry-card">
            <div className="label">DRS / ERS</div>

            {/* DRS Status */}
            <div style={{ marginBottom: '20px' }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px' }}>
                    <div className="label">DRS</div>
                    <div className="value" style={{ color: drsColors[drs as keyof typeof drsColors] }}>
                        {drs}
                    </div>
                </div>
                <div style={{
                    height: '8px',
                    backgroundColor: '#1a1a1a',
                    borderRadius: '4px',
                    overflow: 'hidden'
                }}>
                    <div style={{
                        width: drs === 'Active' ? '100%' : drs === 'Available' ? '50%' : '0%',
                        height: '100%',
                        backgroundColor: drsColors[drs as keyof typeof drsColors],
                        transition: 'width 0.1s ease'
                    }}></div>
                </div>
            </div>

            {/* ERS Mode */}
            <div style={{ marginBottom: '20px' }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px' }}>
                    <div className="label">ERS MODE</div>
                    <div className="value" style={{ color: ersModeColors[ersMode as keyof typeof ersModeColors] }}>
                        {ersMode}
                    </div>
                </div>
            </div>

            {/* ERS Energy Bar */}
            <div>
                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '5px' }}>
                    <div className="label">ENERGY</div>
                    <div className="value">{Math.round(ersPercent)}%</div>
                </div>
                <div style={{
                    height: '10px',
                    backgroundColor: '#1a1a1a',
                    borderRadius: '5px',
                    overflow: 'hidden'
                }}>
                    <div style={{
                        width: `${ersPercent}%`,
                        height: '100%',
                        backgroundColor: ersPercent > 50 ? 'var(--color-zone-green)' : ersPercent > 20 ? 'var(--color-zone-yellow)' : 'var(--color-zone-red)',
                        transition: 'width 0.1s ease, background-color 0.1s ease'
                    }}></div>
                </div>
                <div style={{ display: 'flex', justifyContent: 'space-between', marginTop: '5px', fontSize: '10px', color: '#888' }}>
                    <span>H: {Math.round(ersHarvested / 1000)}kJ</span>
                    <span>D: {Math.round(ersDeployed / 1000)}kJ</span>
                </div>
            </div>
        </div>
    );
};

export default DRSERS;
