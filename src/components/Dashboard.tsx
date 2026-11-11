import RPMGauge from './TelemetryGauges/RPMGauge';
import SpeedGear from './TelemetryGauges/SpeedGear';
import PedalBars from './TelemetryGauges/PedalBars';
import DRSERS from './TelemetryGauges/DRSERS';
import Suspension from './TelemetryGauges/Suspension';
import TyreSet from './TyreMonitor/TyreSet';
import TyreSetsPanel from './TyreMonitor/TyreSetsPanel';
import LapTiming from './LapTiming/LapTiming';
import CarStatus from './CarStatus/CarStatus';
import SessionInfo from './SessionInfo/SessionInfo';
import SettingsPanel from './SettingsPanel/SettingsPanel';
import SessionHistoryPanel from './SessionHistory/SessionHistoryPanel';
import '../styles/f1-theme.css';

const Dashboard: React.FC = () => {
    return (
        <div className="f1-dashboard">
            {/* Row 1: Main Gauges (4 columns spanning full width) */}
            <div className="f1-dashboard__gauges">
                <RPMGauge />
                <SpeedGear />
                <PedalBars />
                <DRSERS />
                <Suspension />
            </div>

            {/* Row 2: Tyre Set (spans full width) */}
            <div className="f1-dashboard__tyres">
                <TyreSet />
                <TyreSetsPanel />
            </div>

            {/* Row 3-4: Three-column layout */}
            {/* Left column: Session Info */}
            <div className="f1-dashboard__session-info">
                <SessionInfo />
            </div>

            {/* Center column: Lap Timing + Session History */}
            <div className="f1-dashboard__lap-timing">
                <LapTiming />
                <SessionHistoryPanel />
            </div>

            {/* Right column: Car Status (spans 2 rows) */}
            <div className="f1-dashboard__car-status">
                <CarStatus />
            </div>

            {/* Row 5: Settings (spans full width) */}
            <div className="f1-dashboard__settings">
                <SettingsPanel />
            </div>
        </div>
    );
};

export default Dashboard;
