// Shared lap formatting utilities for lap history tables and timing displays.

export const formatLapTime = (ms: number): string => {
    if (!ms || ms === Infinity) return '--:--.---';
    const mins = Math.floor(ms / 60000);
    const secs = Math.floor((ms % 60000) / 1000);
    const millis = ms % 1000;
    return `${mins}:${secs.toString().padStart(2, '0')}.${millis.toString().padStart(3, '0')}`;
};

export const formatSectorTime = (ms: number): string => {
    if (!ms) return '--.---';
    const secs = Math.floor(ms / 1000);
    const millis = ms % 1000;
    return `${secs}.${millis.toString().padStart(3, '0')}`;
};

export const formatDeltaToBest = (lapTimeMs: number, bestLapTimeMs: number): string => {
    if (!lapTimeMs || !bestLapTimeMs || bestLapTimeMs === Infinity) return '--.---';
    const delta = lapTimeMs - bestLapTimeMs;
    const sign = delta > 0 ? '+' : delta < 0 ? '-' : '';
    const abs = Math.abs(delta);
    return `${sign}${Math.floor(abs / 1000)}.${(abs % 1000).toString().padStart(3, '0')}s`;
};

export const isBestLap = (lapTimeMs: number, bestLapTimeMs: number): boolean => {
    return lapTimeMs > 0 && bestLapTimeMs > 0 && lapTimeMs <= bestLapTimeMs;
};
