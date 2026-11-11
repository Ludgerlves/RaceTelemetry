// src/components/SessionHistory/SessionHistory.tsx
// Sessions history browser UI (Feature 2) — lists persisted sessions.
import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTelemetryStore } from '../../stores/telemetryStore';
import { useSessions } from '../../hooks/useSessions';
import SessionCard from './SessionCard';
import '../../styles/f1-theme.css';

const SessionHistory: React.FC = () => {
    const { sessions, loadSessions, removeSession } = useSessions();
    const setSelectedSession = useTelemetryStore(state => state.setSelectedSession);
    const [selectedId, setSelectedId] = useState<string | null>(null);
    const [search, setSearch] = useState('');

    useEffect(() => {
        loadSessions();
    }, [loadSessions]);

    const filtered = sessions.filter((s: any) =>
        s.trackName.toLowerCase().includes(search.toLowerCase()) ||
        s.sessionType.toLowerCase().includes(search.toLowerCase())
    );

    const handleSelect = async (id: string) => {
        setSelectedId(id);
        const record = await invoke('get_session', { id });
        setSelectedSession(record as any);
    };

    const handleDelete = async (id: string) => {
        if (!window.confirm('Delete this session?')) return;
        await removeSession(id);
        if (selectedId === id) {
            setSelectedId(null);
            setSelectedSession(null);
        }
    };

    return (
        <div className="f1-card f1-session-history">
            <div className="f1-card__header">
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                    <h2 className="f1-card__title">Session History</h2>
                    <div style={{ fontSize: '11px', color: 'var(--f1-white-muted)' }}>
                        {sessions.length} saved session{sessions.length !== 1 ? 's' : ''}
                    </div>
                </div>
            </div>
            <div className="f1-card__body">
                <div style={{ marginBottom: 'var(--gap-md)' }}>
                    <input
                        type="text"
                        placeholder="Search sessions by track or session type..."
                        value={search}
                        onChange={(e) => setSearch(e.target.value)}
                        className="f1-input"
                        style={{ width: '100%' }}
                    />
                </div>

                <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--gap-xs)' }}>
                    {filtered.length === 0 ? (
                        <div style={{ padding: 'var(--gap-md)', color: 'var(--f1-white-muted)', textAlign: 'center' }}>
                            No sessions found. Complete a session in F1 25 to see it here.
                        </div>
                    ) : (
                        filtered.map((session: any) => (
                            <SessionCard
                                key={session.id}
                                session={session}
                                onSelect={handleSelect}
                            />
                        ))
                    )}
                </div>

                {selectedId && (
                    <div style={{
                        marginTop: 'var(--gap-lg)',
                        padding: 'var(--gap-md)',
                        background: 'var(--f1-bg-secondary)',
                        border: '1px solid var(--f1-border)',
                        borderRadius: 'var(--radius-md)',
                    }}>
                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 'var(--gap-md)' }}>
                            <div style={{ fontWeight: 'bold', color: 'var(--f1-white)' }}>Selected Session</div>
                            <button
                                className="f1-btn f1-btn--danger f1-btn--sm"
                                onClick={() => handleDelete(selectedId!)}
                            >
                                Delete
                            </button>
                        </div>
                        <div style={{ fontSize: '11px', color: 'var(--f1-white-muted)' }}>
                            Select a session above to view its laps in the Lap History panel.
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};

export default SessionHistory;
