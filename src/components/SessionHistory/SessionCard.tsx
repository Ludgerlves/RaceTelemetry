// src/components/SessionHistory/SessionCard.tsx
// Card summarizing a persisted session for the sessions history browser.
import React from 'react';
import { useTelemetryStore } from '../../stores/telemetryStore';
import { SessionListItem } from '../../types/telemetry';
import { formatLapTime } from '../../utils/formatLap';
import '../../styles/f1-theme.css';

interface SessionCardProps {
    session: SessionListItem;
    onSelect: (id: string) => void;
}

const formatDate = (ts: number): string => {
    const d = new Date(ts);
    return d.toLocaleDateString(undefined, {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
    });
};

const SessionCard: React.FC<SessionCardProps> = ({ session, onSelect }) => {
    const selectedSession = useTelemetryStore(state => state.selectedSession);
    const isSelected = selectedSession?.id === session.id;

    const bestLap = session.bestLapTime === Infinity ? '--:--.---' : formatLapTime(session.bestLapTime);

    return (
        <div
            className="session-card"
            onClick={() => onSelect(session.id)}
            style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                padding: 'var(--gap-md)',
                background: isSelected ? 'rgba(225, 6, 0, 0.15)' : 'var(--f1-bg-secondary)',
                border: `1px solid ${isSelected ? 'var(--f1-red)' : 'var(--f1-border)'}`,
                borderRadius: 'var(--radius-md)',
                cursor: 'pointer',
                transition: 'all 0.15s ease',
            }}
        >
            <div style={{ flex: 1, minWidth: 0 }}>
                <div style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'baseline',
                    marginBottom: '4px',
                }}>
                    <div style={{ fontWeight: 'bold', color: 'var(--f1-white)', fontSize: '14px' }}>
                        {session.trackName}
                    </div>
                    <div style={{ fontSize: '11px', color: 'var(--f1-white-muted)', fontFamily: 'var(--font-mono)' }}>
                        {session.sessionUid.toString(16).toUpperCase()}
                    </div>
                </div>
                <div style={{ fontSize: '11px', color: 'var(--f1-white-muted)', marginBottom: '6px' }}>
                    {session.sessionType}  •  {formatDate(session.startedAt)}  •  {session.totalLaps} laps
                </div>
                <div style={{ display: 'flex', gap: 'var(--gap-md)', fontSize: '12px', color: 'var(--f1-text-secondary)' }}>
                    <div>
                        <div style={{ fontSize: '10px', color: '#666', textTransform: 'uppercase' }}>Best Lap</div>
                        <div style={{ color: 'var(--f1-green-sector)', fontFamily: 'var(--font-mono)' }}>{bestLap}</div>
                    </div>
                    <div>
                        <div style={{ fontSize: '10px', color: '#666', textTransform: 'uppercase' }}>Weather</div>
                        <div>{session.weather}</div>
                    </div>
                </div>
            </div>
            <div style={{ fontSize: '11px', color: 'var(--f1-white-muted)', whiteSpace: 'nowrap' }}>
                {isSelected ? 'SELECTED' : 'Open'}
            </div>
        </div>
    );
};

export default SessionCard;
