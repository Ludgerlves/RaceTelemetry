// src/components/CarStatus/CarStatus.tsx
import { useTelemetryStore } from '../../stores/telemetryStore';
import { convertFuel } from '../../utils/unitConversion';
import '../../styles/f1-theme.css';

const CarStatus: React.FC = () => {
    const carStatus = useTelemetryStore(state => state.carStatus);
    const carsStatus = useTelemetryStore(state => state.carsStatus);
    const carDamage = useTelemetryStore(state => state.carDamage);
    const units = useTelemetryStore(state => state.units);
    const activeDriverIndex = useTelemetryStore(state => state.session.activeDriverIndex);

    const status = carsStatus[activeDriverIndex] || carStatus;
    const damage = carDamage[activeDriverIndex];

    if (!status) {
        return (
            <div className="status-card">
                <h3 style={{ borderBottom: '1px solid #333', paddingBottom: '10px' }}>Car Strategy & Status</h3>
                <div className="label">Waiting for telemetry...</div>
            </div>
        );
    }

    const { value: fuelInTank, unit: fuelUnit } = convertFuel(status.fuelInTank, units);
    const { value: fuelCapacity } = convertFuel(status.fuelCapacity, units);
    const fuelPercent = status.fuelCapacity > 0
        ? (status.fuelInTank / status.fuelCapacity) * 100
        : 0;

    const ersPercent = status.ersStoreEnergy / 4000000 * 100;

    const fuelMixColors = {
        'Lean': '#00cc66',
        'Standard': '#ffff00',
        'Rich': '#ff8800'
    };

    const ersModeColors = {
        'None': '#444',
        'Medium': '#888',
        'Hotlap': 'var(--color-zone-yellow)',
        'Overtake': 'var(--color-accent-red)'
    };

    return (
        <div className="status-card">
            <h3 style={{ borderBottom: '1px solid #333', paddingBottom: '10px' }}>Car Strategy & Status</h3>

            {/* Fuel Section */}
            <div style={{ marginBottom: '20px' }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '8px' }}>
                    <div className="label">FUEL</div>
                    <div className="value">{fuelInTank.toFixed(1)} {fuelUnit} / {fuelCapacity.toFixed(1)} {fuelUnit}</div>
                </div>
                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '4px' }}>
                    <div className="label">{status.fuelRemainingLaps.toFixed(1)} laps remaining</div>
                    <div className="label">{status.fuelMix} Mix</div>
                </div>
                <div style={{
                    height: '8px',
                    backgroundColor: '#1a1a1a',
                    borderRadius: '4px',
                    overflow: 'hidden'
                }}>
                    <div style={{
                        width: `${Math.min(fuelPercent, 100)}%`,
                        height: '100%',
                        backgroundColor: fuelPercent > 30 ? 'var(--color-zone-green)' : fuelPercent > 15 ? 'var(--color-zone-yellow)' : 'var(--color-zone-red)',
                        transition: 'width 0.1s ease'
                    }}></div>
                </div>
            </div>

            {/* ERS Section */}
            <div style={{ marginBottom: '20px' }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '8px' }}>
                    <div className="label">ERS</div>
                    <div className="value" style={{ color: ersModeColors[status.ersDeployMode as keyof typeof ersModeColors] }}>
                        {status.ersDeployMode}
                    </div>
                </div>
                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '4px' }}>
                    <div className="label">{status.ersStoreEnergy.toFixed(0)} J ({(status.ersStoreEnergy/1000000).toFixed(2)} MJ)</div>
                    <div className="label">H: {status.ersHarvestedThisLapTotal.toFixed(0)} J | D: {status.ersDeployedThisLapTotal.toFixed(0)} J</div>
                </div>
                <div style={{
                    height: '8px',
                    backgroundColor: '#1a1a1a',
                    borderRadius: '4px',
                    overflow: 'hidden'
                }}>
                    <div style={{
                        width: `${Math.min(ersPercent, 100)}%`,
                        height: '100%',
                        backgroundColor: ersPercent > 50 ? 'var(--color-zone-green)' : ersPercent > 20 ? 'var(--color-zone-yellow)' : 'var(--color-zone-red)',
                        transition: 'width 0.1s ease'
                    }}></div>
                </div>
            </div>

            {/* Engine & Brake */}
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '15px', marginBottom: '15px' }}>
                <div>
                    <div className="label">ENGINE MODE</div>
                    <div className="value" style={{ color: fuelMixColors[status.fuelMix as keyof typeof fuelMixColors] }}>
                        {status.fuelMix}
                    </div>
                </div>
                <div>
                    <div className="label">BRAKE BIAS</div>
                    <div className="value">{status.frontBrakeBias / 100}% Front</div>
                </div>
                <div>
                    <div className="label">MAX RPM</div>
                    <div className="value">{status.maxRpm.toLocaleString()}</div>
                </div>
                <div>
                    <div className="label">IDLE RPM</div>
                    <div className="value">{status.idleRpm.toLocaleString()}</div>
                </div>
            </div>

            {/* Tyres */}
            <div style={{ borderTop: '1px solid #333', paddingTop: '15px' }}>
                <div className="label">TYRES</div>
                <div style={{ display: 'flex', justifyContent: 'space-between', marginTop: '8px' }}>
                    <div>
                        <div className="label">Compound</div>
                        <div className="value">{status.visualTyreCompound}</div>
                    </div>
                    <div>
                        <div className="label">Age</div>
                        <div className="value">{status.tyresAgeLaps} laps</div>
                    </div>
                    <div>
                        <div className="label">Actual</div>
                        <div className="value">{status.actualTyreCompound}</div>
                    </div>
                </div>
            </div>

            {/* Damage Indicators */}
            {damage && (
                <div style={{ borderTop: '1px solid #333', paddingTop: '15px', marginTop: '15px' }}>
                    <div className="label">DAMAGE</div>
                    <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: '10px', marginTop: '8px' }}>
                        <div style={{ padding: '8px', background: '#1a1a1a', borderRadius: '4px', border: '1px solid #333' }}>
                            <div style={{ fontSize: '10px', color: '#888', marginBottom: '4px' }}>Tyres Wear</div>
                            <div style={{ fontSize: '13px', color: 'var(--f1-white)', fontFamily: 'monospace' }}>
                                {Math.max(...damage.tyresWear).toFixed(0)}%
                            </div>
                        </div>
                        <div style={{ padding: '8px', background: '#1a1a1a', borderRadius: '4px', border: '1px solid #333' }}>
                            <div style={{ fontSize: '10px', color: '#888', marginBottom: '4px' }}>Engine</div>
                            <div style={{ fontSize: '13px', color: 'var(--f1-white)', fontFamily: 'monospace' }}>
                                {damage.engineDamage.toFixed(0)}%
                            </div>
                        </div>
                        <div style={{ padding: '8px', background: '#1a1a1a', borderRadius: '4px', border: '1px solid #333' }}>
                            <div style={{ fontSize: '10px', color: '#888', marginBottom: '4px' }}>Front Wing</div>
                            <div style={{ fontSize: '13px', color: 'var(--f1-white)', fontFamily: 'monospace' }}>
                                {damage.frontLeftWingDamage.toFixed(0)}%
                            </div>
                        </div>
                        <div style={{ padding: '8px', background: '#1a1a1a', borderRadius: '4px', border: '1px solid #333' }}>
                            <div style={{ fontSize: '10px', color: '#888', marginBottom: '4px' }}>Rear Wing</div>
                            <div style={{ fontSize: '13px', color: 'var(--f1-white)', fontFamily: 'monospace' }}>
                                {damage.rearWingDamage.toFixed(0)}%
                            </div>
                        </div>
                        <div style={{ padding: '8px', background: '#1a1a1a', borderRadius: '4px', border: '1px solid #333' }}>
                            <div style={{ fontSize: '10px', color: '#888', marginBottom: '4px' }}>Floor</div>
                            <div style={{ fontSize: '13px', color: 'var(--f1-white)', fontFamily: 'monospace' }}>
                                {damage.floorDamage.toFixed(0)}%
                            </div>
                        </div>
                        <div style={{ padding: '8px', background: '#1a1a1a', borderRadius: '4px', border: '1px solid #333' }}>
                            <div style={{ fontSize: '10px', color: '#888', marginBottom: '4px' }}>Diffuser</div>
                            <div style={{ fontSize: '13px', color: 'var(--f1-white)', fontFamily: 'monospace' }}>
                                {damage.diffuserDamage.toFixed(0)}%
                            </div>
                        </div>
                    </div>
                </div>
            )}

            {/* Status flags */}
            <div style={{ borderTop: '1px solid #333', paddingTop: '15px', marginTop: '15px' }}>
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '15px' }}>
                    <div style={{
                        display: 'flex',
                        alignItems: 'center',
                        gap: '5px',
                        color: status.drsAllowed ? 'var(--color-zone-green)' : '#666'
                    }}>
                        <span style={{
                            width: '10px',
                            height: '10px',
                            borderRadius: '50%',
                            backgroundColor: status.drsAllowed ? 'var(--color-zone-green)' : '#444'
                        }}></span>
                        <span>DRS: {status.drsAllowed ? 'ALLOWED' : 'NOT ALLOWED'}</span>
                    </div>
                    <div style={{
                        display: 'flex',
                        alignItems: 'center',
                        gap: '5px',
                        color: status.pitLimiterStatus ? 'var(--color-zone-yellow)' : '#666'
                    }}>
                        <span style={{
                            width: '10px',
                            height: '10px',
                            borderRadius: '50%',
                            backgroundColor: status.pitLimiterStatus ? 'var(--color-zone-yellow)' : '#444'
                        }}></span>
                        <span>Pit Limiter: {status.pitLimiterStatus ? 'ON' : 'OFF'}</span>
                    </div>
                    <div style={{
                        display: 'flex',
                        alignItems: 'center',
                        gap: '5px',
                        color: status.vehicleFiaFlags > 0 ? 'var(--color-zone-yellow)' : '#666'
                    }}>
                        <span style={{
                            width: '10px',
                            height: '10px',
                            borderRadius: '50%',
                            backgroundColor: status.vehicleFiaFlags > 0 ? 'var(--color-zone-yellow)' : '#444'
                        }}></span>
                        <span>Flags: {status.vehicleFiaFlags}</span>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default CarStatus;
