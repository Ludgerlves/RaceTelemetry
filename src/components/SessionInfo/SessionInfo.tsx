import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTelemetryStore } from '../../stores/telemetryStore';
import { convertTemperature } from '../../utils/unitConversion';

const weatherColors: Record<string, string> = {
  'Clear': '#ffeb3b',
  'LightCloud': '#b0b0b0',
  'Overcast': '#888888',
  'LightRain': '#0099ff',
  'HeavyRain': '#0066cc',
  'Storm': '#4a4a4f',
};

const sessionTypeLabels: Record<string, string> = {
  'Practice1': 'PRACTICE 1',
  'Practice2': 'PRACTICE 2',
  'Practice3': 'PRACTICE 3',
  'ShortPractice': 'PRACTICE',
  'Qualifying1': 'QUALIFYING 1',
  'Qualifying2': 'QUALIFYING 2',
  'Qualifying3': 'QUALIFYING 3',
  'ShortQualifying': 'QUALIFYING',
  'OneShotQualifying': 'QUALIFYING',
  'Race': 'RACE',
  'Race2': 'RACE 2',
  'Race3': 'RACE 3',
  'TimeTrial': 'TIME TRIAL',
  'Unknown': 'UNKNOWN',
};

const SessionInfo: React.FC = () => {
  const { session, setActiveDriverIndex, units } = useTelemetryStore();
  const [availableDrivers, setAvailableDrivers] = useState<string[]>([]);

  const formatTime = (seconds: number) => {
    if (!seconds) return '--:--';
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  useEffect(() => {
    invoke<string[]>('get_available_drivers').then((drivers) => {
      setAvailableDrivers(drivers);
    }).catch((e: Error) => console.error("Error fetching drivers:", e));
  }, []);

  const handleDriverChange = (index: number) => {
    setActiveDriverIndex(index);
    invoke('set_active_driver', { playerIndex: index }).catch((e: Error) => console.error("Error setting driver:", e));
  };

  const weatherColor = weatherColors[session.weather] || 'var(--f1-white)';

  return (
    <div className="f1-card f1-session">
      <div className="f1-card__header">
        <h2 className="f1-card__title">Session Info</h2>
      </div>
      <div className="f1-card__body">

        <div className="f1-session-grid">
          <div className="f1-session-item">
            <div className="f1-session-label">TRACK</div>
            <div className="f1-session-value f1-session-value--track">{session.trackName}</div>
          </div>
          <div className="f1-session-item">
            <div className="f1-session-label">SESSION</div>
            <div className="f1-session-value">{sessionTypeLabels[session.sessionType] || session.sessionType}</div>
          </div>

          <div className="f1-session-item">
            <div className="f1-session-label">WEATHER</div>
            <div className="f1-session-value" style={{ color: weatherColor }}>
              {session.weather}
            </div>
          </div>
          <div className="f1-session-item">
            <div className="f1-session-label">TEMPERATURES</div>
            <div className="f1-session-value">
                {(() => {
                    const { value: trackTemp, unit: tempUnit } = convertTemperature(session.trackTemp, units);
                    const { value: airTemp } = convertTemperature(session.airTemp, units);
                    return `TRACK ${Math.round(trackTemp)}${tempUnit}  AIR ${Math.round(airTemp)}${tempUnit}`;
                })()}
            </div>
          </div>

          <div className="f1-session-item">
            <div className="f1-session-label">TIME REMAINING</div>
            <div className="f1-session-value">{formatTime(session.sessionTimeLeft)}</div>
          </div>
          <div className="f1-session-item">
            <div className="f1-session-label">TOTAL LAPS</div>
            <div className="f1-session-value">{session.totalLaps}</div>
          </div>

          <div className="f1-session-item">
            <div className="f1-session-label">ACTIVE CARS</div>
            <div className="f1-session-value">{session.numActiveCars}</div>
          </div>
          <div className="f1-session-item">
            <div className="f1-session-label">PLAYER INDEX</div>
            <div className="f1-session-value">{session.playerCarIndex}</div>
          </div>
        </div>

        <div className="f1-session-item f1-session-item--span f1-driver-selector">
          <div className="f1-session-label">ACTIVE DRIVER</div>
          <select 
            className="f1-select"
            onChange={(e) => handleDriverChange(parseInt(e.target.value))}
            value={session.activeDriverIndex}
          >
            {availableDrivers.map((driver, index) => (
              <option key={index} value={index} selected={index === session.activeDriverIndex}>
                {driver}
              </option>
            ))}
            {availableDrivers.length === 0 && (
              <option value="" disabled>Waiting for participants...</option>
            )}
          </select>
        </div>

        <div className="f1-session-item f1-session-item--span" style={{ borderTop: '1px solid var(--f1-border)', paddingTop: 'var(--gap-md)', marginTop: 'var(--gap-sm)', fontSize: '10px', color: 'var(--f1-white-muted)', fontFamily: 'var(--font-mono)' }}>
          SESSION UID: {session.sessionUid.toString(16).toUpperCase()}
        </div>

      </div>
    </div>
  );
};

export default SessionInfo;
