// src/hooks/useSessions.ts
// Hook for browsing persisted telemetry sessions (Feature A).
// Wraps the Tauri commands list_sessions / get_session / delete_session / save_current_session.

import { invoke } from '@tauri-apps/api/core';
import { SessionListItem, SessionRecord } from '../types/telemetry';
import { useTelemetryStore } from '../stores/telemetryStore';

export function useSessions() {
    const setSessionsAction = useTelemetryStore(state => state.setSessions);
    const setSelectedSessionAction = useTelemetryStore(state => state.setSelectedSession);
    const deleteSessionStoreAction = useTelemetryStore(state => state.deleteSession);
    const sessions = useTelemetryStore(state => state.sessions);
    const selectedSession = useTelemetryStore(state => state.selectedSession);

    const loadSessions = async () => {
        try {
            const list: SessionListItem[] = await invoke('list_sessions');
            setSessionsAction(list);
        } catch (e) {
            console.error("Failed to load sessions:", e);
        }
    };

    const loadSession = async (id: string) => {
        try {
            const record: SessionRecord = await invoke('get_session', { id });
            setSelectedSessionAction(record);
            return record;
        } catch (e) {
            console.error("Failed to load session:", e);
            return null;
        }
    };

    const removeSession = async (id: string) => {
        try {
            await invoke('delete_session', { id });
            deleteSessionStoreAction(id);
        } catch (e) {
            console.error("Failed to delete session:", e);
        }
    };

    const saveCurrent = async () => {
        try {
            const id: string = await invoke('save_current_session');
            await loadSessions();
            return id;
        } catch (e) {
            console.error("Failed to save current session:", e);
            return null;
        }
    };

    return {
        sessions,
        selectedSession,
        loadSessions,
        loadSession,
        removeSession,
        saveCurrent,
    };
}
