// src/components/LapTiming/LapTiming.tsx
import React, { useEffect, useRef, useMemo } from 'react';
import { useTelemetryStore } from '../../stores/telemetryStore';
import '../../styles/f1-theme.css';
import uPlot from 'uplot';
import 'uplot/dist/uPlot.min.css';

interface uPlotInstance {
  setData: (data: Float64Array[]) => void;
  destroy: () => void;
}

const LapTiming: React.FC = () => {
    const session = useTelemetryStore(state => state.session);
    const plotRef = useRef<HTMLDivElement>(null);
    const uPlotRef = useRef<uPlotInstance | null>(null);

    // Prepare lap history data for uPlot
    const lapHistory = session.lapHistory;
    const bestLapTime = session.bestLapTime === Infinity ? 0 : session.bestLapTime;

    // Format data for uPlot - use Float64Array for typed arrays
    const data = useMemo(() => {
        const lapNumbers = new Float64Array(lapHistory.map((_, i) => i + 1));
        const lapTimes = new Float64Array(lapHistory.map(l => l.lapTimeMs / 1000));
        const bestLapLine = new Float64Array(lapHistory.map(() => bestLapTime / 1000 || 0));
        return [lapNumbers, lapTimes, bestLapLine];
    }, [lapHistory, bestLapTime]);

    // uPlot options
    const opts = useMemo(() => ({
        title: 'Lap History',
        width: 600,
        height: 200,
        scales: {
            x: { time: false },
            y: {
                range: (_u: uPlot, dataMin: number, dataMax: number): [number, number] => {
                    const padding = (dataMax - dataMin) * 0.1 || 1;
                    return [dataMin - padding, dataMax + padding];
                }
            }
        },
        series: [
            {}, // x-axis
            {
                label: 'Lap Time',
                stroke: 'var(--color-accent-white)',
                width: 2,
                points: { show: true, size: 4, stroke: 'var(--color-accent-white)' },
            },
            {
                label: 'Best Lap',
                stroke: 'var(--color-zone-green)',
                width: 1,
                dash: [5, 5],
                points: { show: false },
            }
        ],
        axes: [
            { scale: 'x', grid: { stroke: '#333' }, ticks: { size: 10 } },
            { scale: 'y', grid: { stroke: '#333' }, ticks: { size: 10 } }
        ],
        hooks: {
            draw: [
                (u: uPlot) => {
                    // Draw sector markers on best lap
                    if (session.bestSectorTimes && session.bestSectorTimes[0] > 0) {
                        const ctx = u.ctx;
                        const { left, top, width, height } = u.bbox;
                        const [s1, s2, s3] = session.bestSectorTimes;
                        const total = s1 + s2 + s3;

                        const x1 = left + (s1 / total) * width;
                        const x2 = left + ((s1 + s2) / total) * width;

                        ctx.strokeStyle = 'var(--color-zone-yellow)';
                        ctx.lineWidth = 1;
                        ctx.setLineDash([3, 3]);
                        [x1, x2].forEach(x => {
                            ctx.beginPath();
                            ctx.moveTo(x, top);
                            ctx.lineTo(x, top + height);
                            ctx.stroke();
                        });
                        ctx.setLineDash([]);
                    }
                }
            ]
        }
    }), [session.bestSectorTimes]);

    // Initialize uPlot
    useEffect(() => {
        if (!plotRef.current) return;

        const u = new uPlot(opts, data, plotRef.current);

        uPlotRef.current = {
            setData: (newData) => u.setData(newData),
            destroy: () => u.destroy(),
        };

        return () => {
            uPlotRef.current?.destroy();
            uPlotRef.current = null;
        };
    }, []); // Only run once on mount

    // Update uPlot data when lap history changes
    useEffect(() => {
        if (uPlotRef.current) {
            uPlotRef.current.setData(data);
        }
    }, [data]);

    const formatTime = (ms: number) => {
        if (!ms || ms === Infinity) return '--:--.---';
        const mins = Math.floor(ms / 60000);
        const secs = Math.floor((ms % 60000) / 1000);
        const millis = ms % 1000;
        return `${mins}:${secs.toString().padStart(2, '0')}.${millis.toString().padStart(3, '0')}`;
    };

    const formatSector = (ms: number) => {
        if (!ms) return '--.---';
        const secs = Math.floor(ms / 1000);
        const millis = ms % 1000;
        return `${secs}.${millis.toString().padStart(3, '0')}`;
    };

    return (
        <div className="timing-card">
            <h3 style={{ borderBottom: '1px solid #333', paddingBottom: '10px' }}>Lap Timing & Sectors</h3>

            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '15px', flexWrap: 'wrap', gap: '10px' }}>
                <div>
                    <div className="label">CURRENT LAP</div>
                    <div className="value">{session.currentLap}</div>
                </div>
                <div>
                    <div className="label">LAST LAP</div>
                    <div className="value">{formatTime(session.lastLapTime)}</div>
                </div>
                <div>
                    <div className="label">BEST LAP</div>
                    <div className="value f1-green-accent">{formatTime(session.bestLapTime)}</div>
                </div>
            </div>

            <div style={{ marginBottom: '20px' }}>
                <div className="label">SECTOR TIMES (S1 / S2 / S3)</div>
                <div style={{ display: 'flex', justifyContent: 'space-between', marginTop: '10px' }}>
                    <div style={{ textAlign: 'center', flex: 1 }}>
                        <div className="value">{formatSector(session.sectorTimes[0])}</div>
                        <div className="label">S1</div>
                    </div>
                    <div style={{ textAlign: 'center', flex: 1 }}>
                        <div className="value">{formatSector(session.sectorTimes[1])}</div>
                        <div className="label">S2</div>
                    </div>
                    <div style={{ textAlign: 'center', flex: 1 }}>
                        <div className="value">{formatSector(session.sectorTimes[2])}</div>
                        <div className="label">S3</div>
                    </div>
                </div>

                {/* Best sectors */}
                <div style={{ display: 'flex', justifyContent: 'space-between', marginTop: '8px', fontSize: '11px', color: '#888' }}>
                    <div style={{ textAlign: 'center', flex: 1 }}>Best: {formatSector(session.bestSectorTimes[0])}</div>
                    <div style={{ textAlign: 'center', flex: 1 }}>Best: {formatSector(session.bestSectorTimes[1])}</div>
                    <div style={{ textAlign: 'center', flex: 1 }}>Best: {formatSector(session.bestSectorTimes[2])}</div>
                </div>
            </div>

            <div style={{ marginBottom: '20px' }}>
                <div className="label">DELTA TO BEST</div>
                <div className="value" style={{
                    color: session.lastLapTime > 0 && session.lastLapTime > session.bestLapTime ? 'var(--color-zone-red)' : 'var(--color-zone-green)'
                }}>
                    {session.bestLapTime !== Infinity && session.lastLapTime > 0
                        ? (session.lastLapTime - session.bestLapTime > 0 ? '+' : '') + ((session.lastLapTime - session.bestLapTime) / 1000).toFixed(3) + 's'
                        : '--.---'}
                </div>
            </div>

            {/* uPlot Chart */}
            <div style={{ height: '220px', border: '1px solid #333', borderRadius: '4px', backgroundColor: '#0a0a0a' }}>
                <div ref={plotRef} style={{ width: '100%', height: '100%' }}></div>
            </div>
        </div>
    );
};

export default LapTiming;
