import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTelemetryStore } from '../../stores/telemetryStore';

interface Settings {
  udpPort: number;
  units: 'metric' | 'imperial';
  theme: 'f1-dark' | 'f1-light';
  autoConnect: boolean;
  logLevel: 'debug' | 'info' | 'warn' | 'error';
  // Machine IP Configuration
  sourceIpOverride: string;
  targetIp: string;
  targetPort: number;
  enableForwarding: boolean;
}

interface SaveSettingsResult {
  success: boolean;
  message: string;
}

interface ConnectionTestResult {
  success: boolean;
  message: string;
  detectedIp: string | null;
  detectedPort: number | null;
  packetCount: number;
}

const defaultSettings: Settings = {
  udpPort: 20777,
  units: 'metric',
  theme: 'f1-dark',
  autoConnect: true,
  logLevel: 'info',
  sourceIpOverride: '',
  targetIp: '',
  targetPort: 20777,
  enableForwarding: false,
};

const SettingsPanel: React.FC = () => {
  const [settings, setSettings] = useState<Settings>(defaultSettings);
  const [isLoading, setIsLoading] = useState(true);
  const [saveStatus, setSaveStatus] = useState<'idle' | 'saving' | 'saved' | 'error'>('idle');
  const [saveMessage, setSaveMessage] = useState<string>('');
  // Connection test state
  const [isTestingConnection, setIsTestingConnection] = useState(false);
  const [connectionTestResult, setConnectionTestResult] = useState<ConnectionTestResult | null>(null);

  const connection = useTelemetryStore(state => state.connection);
  const sourceIp = useTelemetryStore(state => state.sourceIp);
  const clearSession = useTelemetryStore(state => state.clearSession);
  const [localIps, setLocalIps] = useState<string[]>([]);
  const [isLoadingNetwork, setIsLoadingNetwork] = useState(true);

  useEffect(() => {
    invoke<Settings>('get_settings').then((s) => {
      setSettings(s);
      setIsLoading(false);
    }).catch(() => {
      setIsLoading(false);
    });
  }, []);

  // Load local IPs on mount
  useEffect(() => {
    const loadLocalIps = async () => {
      try {
        const ips = await invoke<string[]>('get_local_ips');
        setLocalIps(ips);
      } catch (e) {
        console.error("Failed to load local IPs:", e);
      } finally {
        setIsLoadingNetwork(false);
      }
    };
    loadLocalIps();
  }, []);

  const handleChange = (key: keyof Settings, value: any) => {
    setSettings(prev => ({ ...prev, [key]: value }));
    setSaveStatus('idle');
    setSaveMessage('');
    setConnectionTestResult(null);
  };

  const handleSave = async () => {
    setSaveStatus('saving');
    setSaveMessage('');
    try {
      const result = await invoke<SaveSettingsResult>('save_settings', { settings });
      if (result.success) {
        setSaveStatus('saved');
        setSaveMessage(result.message);
      } else {
        setSaveStatus('error');
        setSaveMessage(result.message);
      }
      setTimeout(() => setSaveStatus('idle'), 3000);
    } catch (e) {
      console.error("Failed to save settings:", e);
      setSaveStatus('error');
      setSaveMessage(e instanceof Error ? e.message : 'Unknown error');
      setTimeout(() => setSaveStatus('idle'), 3000);
    }
  };

  const handleTestConnection = async () => {
    setIsTestingConnection(true);
    setConnectionTestResult(null);
    try {
      const result = await invoke<ConnectionTestResult>('test_connection', { port: settings.udpPort });
      setConnectionTestResult(result);
    } catch (e) {
      console.error("Connection test failed:", e);
      setConnectionTestResult({
        success: false,
        message: e instanceof Error ? e.message : 'Connection test failed',
        detectedIp: null,
        detectedPort: null,
        packetCount: 0,
      });
    } finally {
      setIsTestingConnection(false);
    }
  };

  const handleReset = async () => {
    setSettings(defaultSettings);
    try {
      await invoke('save_settings', { settings: defaultSettings });
    } catch (e) {
      console.error("Failed to reset settings:", e);
    }
  };

  if (isLoading) {
    return (
      <div className="f1-card">
        <div className="f1-card__body" style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', minHeight: '100px', color: 'var(--color-text-secondary)' }}>
          Loading settings...
        </div>
      </div>
    );
  }

  return (
    <div className="f1-card f1-settings">
      <div className="f1-card__header">
        <h2 className="f1-card__title">Settings</h2>
      </div>
      <div className="f1-card__body">

        {/* Network Settings */}
        <div className="f1-settings-group">
          <div className="f1-settings-group-title">Network</div>
          <div className="f1-settings-grid">
            <div className="f1-setting">
              <div className="f1-setting-label">UDP Port</div>
              <input
                type="number"
                className="f1-input"
                value={settings.udpPort}
                onChange={(e) => handleChange('udpPort', parseInt(e.target.value))}
                min={1024}
                max={65535}
              />
            </div>
            <div className="f1-setting">
              <div className="f1-setting-label">Connection</div>
              <div style={{ display: 'flex', alignItems: 'center', gap: 'var(--gap-sm)' }}>
                <div className="f1-value" style={{ color: connection.connected ? 'var(--f1-green-sector)' : 'var(--f1-red)' }}>
                  {connection.connected ? 'CONNECTED' : 'DISCONNECTED'}
                </div>
                <button
                  className="f1-btn f1-btn--secondary f1-btn--sm"
                  onClick={handleTestConnection}
                  disabled={isTestingConnection || connection.connected}
                  style={{ marginLeft: 'var(--gap-sm)' }}
                >
                  {isTestingConnection ? 'Testing...' : 'Test Connection'}
                </button>
              </div>
            </div>
            {connectionTestResult && (
              <div className="f1-setting" style={{ gridColumn: '1 / -1' }}>
                <div className="f1-setting-hint" style={{
                  padding: 'var(--gap-sm)',
                  borderRadius: 'var(--radius-sm)',
                  backgroundColor: connectionTestResult.success ? 'rgba(0, 214, 58, 0.1)' : 'rgba(225, 6, 0, 0.1)',
                  border: `1px solid ${connectionTestResult.success ? 'var(--f1-green-sector)' : 'var(--f1-red)'}`,
                  marginTop: 'var(--gap-xs)'
                }}>
                  {connectionTestResult.message}
                </div>
              </div>
            )}
            <div className="f1-setting">
              <div className="f1-setting-label">Packet Rate</div>
              <div className="f1-value">{connection.packetRate.toFixed(1)} Hz</div>
            </div>
            <div className="f1-setting">
              <label className="f1-checkbox">
                <input
                  type="checkbox"
                  checked={settings.autoConnect}
                  onChange={(e) => handleChange('autoConnect', e.target.checked)}
                />
                <span>Auto-connect on startup</span>
              </label>
            </div>
          </div>
        </div>

        {/* Machine IP Configuration */}
        <div className="f1-settings-group">
          <div className="f1-settings-group-title">
            Machine IP Configuration
            <span style={{ fontSize: '11px', fontWeight: 'normal', color: 'var(--f1-white-muted)', marginLeft: 'var(--gap-sm)' }}>
              (Optional: Data Forwarding)
            </span>
          </div>
          <p className="f1-setting-hint" style={{ marginBottom: 'var(--gap-md)' }}>
            Source IP Override is for manual PS5 IP entry. Forwarding fields below are only needed if sending telemetry to another machine.
          </p>
          <div className="f1-settings-grid">
            <div className="f1-setting">
              <div className="f1-setting-label">Source IP Override</div>
              <input
                type="text"
                className="f1-input"
                placeholder="Auto-detected (leave empty)"
                value={settings.sourceIpOverride}
                onChange={(e) => handleChange('sourceIpOverride', e.target.value)}
              />
              <p className="f1-setting-hint">Manually specify F1 game IP. Leave empty for auto-detection.</p>
            </div>
            <div className="f1-setting">
              <div className="f1-setting-label">Target IP (Forwarding)</div>
              <input
                type="text"
                className="f1-input"
                placeholder="192.168.1.xxx"
                value={settings.targetIp}
                onChange={(e) => handleChange('targetIp', e.target.value)}
              />
              <p className="f1-setting-hint">Optional: IP to forward telemetry data to. Leave empty for local-only.</p>
            </div>
            <div className="f1-setting">
              <div className="f1-setting-label">Target Port (Forwarding)</div>
              <input
                type="number"
                className="f1-input"
                value={settings.targetPort}
                onChange={(e) => handleChange('targetPort', parseInt(e.target.value) || 20777)}
                min={1024}
                max={65535}
              />
              <p className="f1-setting-hint">Optional: Port for forwarding. Default 20777.</p>
            </div>
            <div className="f1-setting">
              <label className="f1-checkbox">
                <input
                  type="checkbox"
                  checked={settings.enableForwarding}
                  onChange={(e) => handleChange('enableForwarding', e.target.checked)}
                />
                <span>Enable Data Forwarding (Optional)</span>
              </label>
              <p className="f1-setting-hint">Only enable if forwarding to another machine. Data stays local by default.</p>
            </div>
          </div>
        </div>

        {/* F1 Game Telemetry Configuration */}
        <div className="f1-settings-group">
          <div className="f1-settings-group-title">F1 Game Telemetry Setup</div>
          <p className="f1-setting-hint">
            Configure these values in the F1 25 game: <strong>Settings → Telemetry Settings → UDP Telemetry</strong>
          </p>

          {/* Detected Source IP */}
          <div className="f1-section-card">
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 'var(--gap-sm)' }}>
              <div className="f1-setting-label" style={{ margin: 0 }}>Detected Source IP (Game → App)</div>
              {sourceIp.ip && (
                <span className="f1-status-badge">
                  ● LIVE
                </span>
              )}
            </div>
            <div className="f1-settings-grid" style={{ gridTemplateColumns: 'repeat(2, 1fr)' }}>
              <div className="f1-setting">
                <div className="f1-setting-label" style={{ fontSize: '11px', color: 'var(--f1-white-muted)' }}>Source IP Address</div>
                <div className="f1-value" style={{
                  fontFamily: 'var(--font-mono)',
                  fontSize: '14px',
                  color: sourceIp.ip ? 'var(--f1-green-sector)' : 'var(--f1-white-muted)'
                }}>
                  {sourceIp.ip || 'Waiting for packets...'}
                </div>
              </div>
              <div className="f1-setting">
                <div className="f1-setting-label" style={{ fontSize: '11px', color: 'var(--f1-white-muted)' }}>Source Port</div>
                <div className="f1-value" style={{ fontFamily: 'var(--font-mono)', fontSize: '14px' }}>
                  {sourceIp.port ? sourceIp.port.toString() : '—'}
                </div>
              </div>
            </div>
            {sourceIp.ip && (
              <div className="f1-setting-hint" style={{ marginTop: 'var(--gap-sm)', paddingTop: 'var(--gap-sm)', borderTop: '1px solid var(--f1-border)' }}>
                Use this IP in F1 game telemetry settings if different from your local IPs below.
              </div>
            )}
          </div>

          {/* Local IPs to configure in F1 game */}
          <div className="f1-section-card">
            <div className="f1-setting-label" style={{ marginBottom: 'var(--gap-sm)' }}>Your Local IPs (Configure in F1 Game)</div>
            {isLoadingNetwork ? (
              <div style={{ color: 'var(--f1-white-muted)', fontSize: '13px' }}>Loading local IPs...</div>
            ) : localIps.length > 0 ? (
              <div style={{ display: 'flex', flexWrap: 'wrap', gap: 'var(--gap-sm)' }}>
                {localIps.map((ip, idx) => (
                  <div key={idx} className="f1-ip-card">
                    <code style={{
                      fontFamily: 'var(--font-mono)',
                      fontSize: '13px',
                      color: 'var(--f1-white)',
                      flex: 1
                    }}>
                      {ip}:{settings.udpPort}
                    </code>
                    <button
                      onClick={() => navigator.clipboard.writeText(`${ip}:${settings.udpPort}`)}
                      className="f1-btn f1-btn--secondary f1-btn--sm"
                      title="Copy to clipboard"
                    >
                      Copy
                    </button>
                  </div>
                ))}
              </div>
            ) : (
              <div style={{ color: 'var(--f1-white-muted)', fontSize: '13px' }}>No non-loopback IPs detected</div>
            )}
            <div className="f1-setting-hint" style={{ marginTop: 'var(--gap-sm)' }}>
              <strong>In F1 25 Game:</strong> Go to Settings → Telemetry Settings → UDP Telemetry →
              Enter one of the above IPs and port <strong>{settings.udpPort}</strong>.
              Enable "Send Telemetry" and set rate to 60Hz (or 20Hz for lower bandwidth).
            </div>
          </div>
        </div>

        {/* Display Settings */}
        <div className="f1-settings-group">
          <div className="f1-settings-group-title">Display</div>
          <div className="f1-settings-grid">
            <div className="f1-setting">
              <div className="f1-setting-label">Units</div>
              <select className="f1-select" value={settings.units} onChange={(e) => handleChange('units', e.target.value as 'metric' | 'imperial')}>
                <option value="metric">Metric (km/h, °C, kg, PSI)</option>
                <option value="imperial">Imperial (mph, °F, lbs, PSI)</option>
              </select>
            </div>
            <div className="f1-setting">
              <div className="f1-setting-label">Theme</div>
              <select className="f1-select" value={settings.theme} onChange={(e) => handleChange('theme', e.target.value as 'f1-dark' | 'f1-light')}>
                <option value="f1-dark">F1 Dark</option>
                <option value="f1-light">F1 Light</option>
              </select>
            </div>
          </div>
        </div>

        {/* Logging Settings */}
        <div className="f1-settings-group">
          <div className="f1-settings-group-title">Logging</div>
          <div className="f1-settings-grid">
            <div className="f1-setting">
              <div className="f1-setting-label">Log Level</div>
              <select className="f1-select" value={settings.logLevel} onChange={(e) => handleChange('logLevel', e.target.value as Settings['logLevel'])}>
                <option value="debug">Debug</option>
                <option value="info">Info</option>
                <option value="warn">Warning</option>
                <option value="error">Error Only</option>
              </select>
            </div>
          </div>
        </div>

        {/* Data Export */}
        <div className="f1-settings-group">
          <div className="f1-settings-group-title">Data Export</div>
          <div className="f1-settings-actions">
            <button className="f1-btn f1-btn--secondary" onClick={() => invoke('export_session', { format: 'json' })}>
              Export JSON
            </button>
            <button className="f1-btn f1-btn--secondary" onClick={() => invoke('export_session', { format: 'csv' })}>
              Export CSV
            </button>
            <button className="f1-btn f1-btn--danger" onClick={() => invoke('clear_session_data').then(() => clearSession())}>
              Clear Session
            </button>
          </div>
        </div>

        {/* Save/Reset */}
        <div className="f1-settings-actions">
          <button className="f1-btn f1-btn--secondary" onClick={handleReset} disabled={saveStatus === 'saving'}>
            Reset Defaults
          </button>
          <button className="f1-btn f1-btn--primary" onClick={handleSave} disabled={saveStatus === 'saving'}>
            {saveStatus === 'saving' ? 'Saving...' : saveStatus === 'saved' ? 'Saved!' : saveStatus === 'error' ? 'Error!' : 'Save Changes'}
          </button>
          {saveStatus === 'saved' && (
            <span className="f1-saved-toast" style={{ backgroundColor: 'var(--f1-green-sector)' }}>
              {saveMessage || 'Settings saved'}
            </span>
          )}
          {saveStatus === 'error' && (
            <span className="f1-saved-toast" style={{ backgroundColor: 'var(--f1-red)', color: 'var(--f1-white)' }}>
              {saveMessage || 'Save failed'}
            </span>
          )}
        </div>

      </div>
    </div>
  );
};

export default SettingsPanel;