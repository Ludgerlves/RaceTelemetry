// src/components/TelemetryGauges/SpeedGear.tsx
import { useTelemetryStore } from '../../stores/telemetryStore';
import { convertSpeed } from '../../utils/unitConversion';
import '../../styles/f1-theme.css';

const SpeedGear: React.FC = () => {
    const carTelemetry = useTelemetryStore(state => state.carTelemetry);
    const units = useTelemetryStore(state => state.units);
    const speed = carTelemetry?.speed || 0;
    const gear = carTelemetry?.gear ?? 0;
    const { value: displaySpeed, unit } = convertSpeed(speed, units);

    const gearLabels: Record<string, string> = { '-1': 'R', '0': 'N', '1': '1', '2': '2', '3': '3', '4': '4', '5': '5', '6': '6', '7': '7', '8': '8' };
    const gearColor = gear <= 0 ? '#888' : 'var(--color-zone-green)';

    return (
        <div className="gauge-container telemetry-card" style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center' }}>
            <div className="label">SPEED</div>
            <div className="value" style={{ fontSize: '48px', fontWeight: 'bold', fontFamily: 'monospace' }}>
                {Math.round(displaySpeed)}
            </div>
            <div className="label" style={{ marginTop: '5px' }}>{unit}</div>

            <div style={{ marginTop: '20px' }}>
                <div className="label">GEAR</div>
                <div className="value" style={{
                    fontSize: '72px',
                    fontWeight: 'bold',
                    fontFamily: 'monospace',
                    color: gearColor
                }}>
                    {gearLabels[gear.toString()] || gear}
                </div>
            </div>
        </div>
    );
};

export default SpeedGear;
