import { useState } from 'react';
import Dashboard from './components/Dashboard';
import SessionHistory from './components/SessionHistory/SessionHistory';
import LapHistoryTable from './components/LapHistory/LapHistoryTable';
import { useTelemetry } from './hooks/useTelemetry';
import './styles/f1-theme.css';

type View = 'live' | 'history';

function App() {
  const { session, connection } = useTelemetry();
  const [view, setView] = useState<View>('live');

  return (
    <div className="f1-app">
      <header className="f1-topbar">
        <div className="f1-topbar__brand">
          <div className="f1-topbar__logo">F1</div>
          <div>
            <div className="f1-topbar__title">Telemetry Monitor</div>
            <div className="f1-topbar__subtitle">F1 2025</div>
          </div>
        </div>
        <div className="f1-topbar__status">
          <div className="f1-status-item">
            <span className={`f1-status-dot ${connection.connected ? 'f1-status-dot--connected' : 'f1-status-dot--disconnected'}`}></span>
            <span className="f1-status-label">UDP</span>
            <span className="f1-status-value">{connection.connected ? 'LIVE' : 'OFFLINE'}</span>
          </div>
          <div className="f1-status-item">
            <span className="f1-status-label">Pkts/s</span>
            <span className="f1-status-value">{connection.packetRate.toFixed(1)}</span>
          </div>
          <div className="f1-status-item">
            <span className="f1-status-label">Track</span>
            <span className="f1-status-value">{session.trackName || '—'}</span>
          </div>
          <div className="f1-status-item f1-status-item--view">
            <button
              className={`f1-view-btn ${view === 'live' ? 'f1-view-btn--active' : ''}`}
              onClick={() => setView('live')}
            >
              Live
            </button>
            <button
              className={`f1-view-btn ${view === 'history' ? 'f1-view-btn--active' : ''}`}
              onClick={() => setView('history')}
            >
              History
            </button>
          </div>
        </div>
      </header>
      <main className="f1-main">
        {view === 'live' ? (
          <Dashboard />
        ) : (
          <div className="f1-history-view" style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '20px', height: '100%', overflow: 'hidden' }}>
            <SessionHistory />
            <LapHistoryTable />
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
