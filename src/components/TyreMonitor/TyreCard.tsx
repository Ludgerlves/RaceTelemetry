// src/components/TyreMonitor/TyreCard.tsx
import React from 'react';
import { convertTemperature, formatPressure } from '../../utils/unitConversion';
import '../../styles/f1-theme.css';

interface TyreCardProps {
    position: 'FL' | 'FR' | 'RL' | 'RR';
    surfaceTemp: number;
    coreTemp: number;
    pressure: number;
    wear: number;
    compoundName: string;
    compoundColor: string;
    units?: 'metric' | 'imperial';
    isSelected?: boolean;
}

const TyreCard: React.FC<TyreCardProps> = ({
    position,
    surfaceTemp,
    coreTemp,
    pressure,
    wear,
    compoundName,
    compoundColor,
    units = 'metric'
}) => {
    // Temperature color coding (based on original Celsius values)
    const getTempColor = (tempC: number) => {
        if (tempC < 70) return '#0099ff';      // Cold - Blue
        if (tempC < 85) return '#00cc66';      // Cool - Green
        if (tempC < 100) return '#ffff00';     // Optimal - Yellow
        if (tempC < 115) return '#ff8800';     // Hot - Orange
        return '#ff0000';                      // Overheating - Red
    };

    const surfaceColor = getTempColor(surfaceTemp);
    const coreColor = getTempColor(coreTemp);
    const wearColor = wear > 80 ? '#ff0000' : wear > 50 ? '#ff8800' : '#00cc66';

    // Convert for display
    const { value: dispSurfaceTemp, unit: tempUnit } = convertTemperature(surfaceTemp, units);
    const { value: dispCoreTemp } = convertTemperature(coreTemp, units);

    return (
        <div className="tyre-card" style={{
            border: `2px solid ${compoundColor}`,
            backgroundColor: '#141414',
            minWidth: '160px'
        }}>
            {/* Position indicator */}
            <div style={{
                textAlign: 'center',
                padding: '5px',
                backgroundColor: compoundColor,
                color: '#000',
                fontWeight: 'bold',
                fontSize: '14px'
            }}>
                {position}
            </div>

            {/* Compound */}
            <div style={{
                textAlign: 'center',
                padding: '8px',
                color: compoundColor,
                fontSize: '12px',
                fontWeight: 'bold',
                textTransform: 'uppercase'
            }}>
                {compoundName}
            </div>

            {/* Temperature display */}
            <div style={{
                display: 'flex',
                justifyContent: 'space-around',
                padding: '10px',
                borderTop: '1px solid #333',
                borderBottom: '1px solid #333'
            }}>
                <div style={{ textAlign: 'center' }}>
                    <div className="label">SURFACE</div>
                    <div className="value" style={{ color: surfaceColor, fontSize: '20px' }}>
                        {Math.round(dispSurfaceTemp)}{tempUnit}
                    </div>
                </div>
                <div style={{ textAlign: 'center' }}>
                    <div className="label">CORE</div>
                    <div className="value" style={{ color: coreColor, fontSize: '20px' }}>
                        {Math.round(dispCoreTemp)}{tempUnit}
                    </div>
                </div>
            </div>

            {/* Pressure and Wear */}
            <div style={{
                display: 'flex',
                justifyContent: 'space-around',
                padding: '10px'
            }}>
                <div style={{ textAlign: 'center' }}>
                    <div className="label">PRESSURE</div>
                    <div className="value" style={{ fontSize: '18px' }}>
                        {(() => {
                            const { value: p, unit: pu } = formatPressure(pressure, units);
                            return `${p.toFixed(1)} ${pu}`;
                        })()}
                    </div>
                </div>
                <div style={{ textAlign: 'center' }}>
                    <div className="label">WEAR</div>
                    <div className="value" style={{ color: wearColor, fontSize: '18px' }}>
                        {wear}%
                    </div>
                    <div style={{
                        height: '6px',
                        backgroundColor: '#1a1a1a',
                        borderRadius: '3px',
                        marginTop: '4px',
                        overflow: 'hidden'
                    }}>
                        <div style={{
                            width: `${wear}%`,
                            height: '100%',
                            backgroundColor: wearColor,
                            transition: 'width 0.1s ease'
                        }}></div>
                    </div>
                </div>
            </div>

            {/* Visual tyre representation */}
            <div style={{
                height: '40px',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                padding: '10px',
                backgroundColor: '#0a0a0a',
                marginTop: 'auto'
            }}>
                <div style={{
                    width: '80%',
                    height: '24px',
                    borderRadius: '12px',
                    backgroundColor: compoundColor,
                    border: '2px solid #333',
                    position: 'relative'
                }}>
                    {/* Wear indicator - inner ring showing remaining rubber */}
                    <div style={{
                        position: 'absolute',
                        top: '4px',
                        left: '4px',
                        right: '4px',
                        bottom: '4px',
                        borderRadius: '8px',
                        backgroundColor: '#0a0a0a',
                        clipPath: `inset(0 ${100 - wear}% 0 0)`
                    }}></div>
                </div>
            </div>
        </div>
    );
};

export default TyreCard;
