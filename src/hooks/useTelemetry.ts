import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useTelemetryStore } from '../stores/telemetryStore';
import {
    TelemetryEventData,
    SessionStateFrontend,
    CarTelemetryDataFrontend,
    CarStatusDataFrontend,
    CarDamageDataFrontend,
    TyreSetDataFrontend,
    MotionExDataFrontend,
    LapHistoryDataFrontend,
    TyreStintHistoryDataFrontend,
} from '../types/telemetry';

export function useTelemetry() {
    const {
        session,
        carTelemetry,
        carStatus,
        connection,
        sourceIp,
        updateSession,
        setCarTelemetry,
        setCarStatus,
        setCarsTelemetry,
        setCarsStatus,
        updateConnection,
        addLapHistory,
        setBestLap,
        setParticipants,
        setActiveDriverIndex,
        setSourceIp,
        setUnits,
        setCarDamage,
        setTyreSets,
        setMotionEx,
        setSessionHistory,
    } = useTelemetryStore();

    useEffect(() => {
        // 1. Initial Session Load
        invoke<string>('get_session_info').then((info) => {
            try {
                const stateData: SessionStateFrontend = JSON.parse(info);
                updateSession(stateData);
            } catch (e) {
                console.error("Failed to parse initial session info:", e);
            }
        }).catch((e: Error) => console.error("Error getting session info:", e));

        // 1b. Load units setting
        invoke<{ units: string }>('get_settings').then((settings) => {
            if (settings.units === 'metric' || settings.units === 'imperial') {
                setUnits(settings.units);
            }
        }).catch((e: Error) => console.error("Error loading settings units:", e));

        // 2. Listen for real-time events
        let unlisten: (() => void) | null = null;

        listen<TelemetryEventData>('telemetry-event', (event) => {
            // console.log("Received telemetry event:", event.payload.type); // Reduce log noise

            const payload = event.payload;

            switch (payload.type) {
                case 'Session': {
                    const data = payload.data as any;
                    updateSession({
                        sessionUid: data.header.m_session_uid,
                        trackName: data.m_track_id.toString(),
                        sessionType: data.m_session_type.toString(),
                        sessionTimeLeft: data.m_session_time_left,
                        sessionDuration: data.m_session_duration,
                        weather: data.m_weather.toString(),
                        trackTemp: data.m_track_temperature,
                        airTemp: data.m_air_temperature,
                        totalLaps: data.m_total_laps,
                        playerCarIndex: data.m_header.m_player_car_index,
                        numActiveCars: data.m_num_active_cars,
                        isNewSession: data.header.m_session_uid !== session.sessionUid,
                    });
                    break;
                }

                case 'Participants': {
                    const data = payload.data as any;
                    const participants = data.m_participants
                        .slice(0, data.m_num_active_cars)
                        .map((p: any, idx: number) => ({
                            carIndex: idx,
                            name: String.fromCharCode(...p.m_name.filter((c: number) => c !== 0)),
                            teamId: p.m_team_id,
                            raceNumber: p.m_race_number,
                            aiControlled: p.m_ai_controlled !== 0,
                            yourTelemetry: p.m_your_telemetry !== 0,
                            carColorPrimary: p.m_car_colour_primary,
                            carColorSecondary: p.m_car_colour_secondary,
                        }));
                    setParticipants(participants);
                    updateSession({ numActiveCars: data.m_num_active_cars });
                    break;
                }

                case 'CarTelemetry': {
                    const data = payload.data as any;
                    const playerIdx = data.player_index;
                    const cars: Record<number, CarTelemetryDataFrontend> = {};
                    if (data.data) {
                        data.data.forEach((ct: any, idx: number) => {
                            cars[idx] = {
                                speed: ct.m_speed,
                                throttle: ct.m_throttle,
                                brake: ct.m_brake,
                                steering: ct.m_steering,
                                gear: ct.m_gear,
                                engineRpm: ct.m_engine_rpm,
                                drs: ct.m_drs,
                                revLightsPercent: ct.m_rev_lights_percent,
                                brakesTemperature: ct.m_brakes_temperature,
                                tyresSurfaceTemperature: ct.m_tyres_surface_temperature,
                                tyresInnerTemperature: ct.m_tyres_inner_temperature,
                                engineTemperature: ct.m_engine_temperature,
                                tyresPressure: ct.m_tyres_pressure,
                                surfaceType: ct.m_surface_type,
                            };
                        });
                        setCarsTelemetry(cars);
                    }
                    if (data.data && data.data[playerIdx]) {
                        const ct = data.data[playerIdx];
                        const telemetryData: CarTelemetryDataFrontend = {
                            speed: ct.m_speed,
                            throttle: ct.m_throttle,
                            brake: ct.m_brake,
                            steering: ct.m_steering,
                            gear: ct.m_gear,
                            engineRpm: ct.m_engine_rpm,
                            drs: ct.m_drs,
                            revLightsPercent: ct.m_rev_lights_percent,
                            brakesTemperature: ct.m_brakes_temperature,
                            tyresSurfaceTemperature: ct.m_tyres_surface_temperature,
                            tyresInnerTemperature: ct.m_tyres_inner_temperature,
                            engineTemperature: ct.m_engine_temperature,
                            tyresPressure: ct.m_tyres_pressure,
                            surfaceType: ct.m_surface_type,
                        };
                        setCarTelemetry(telemetryData);
                    }
                    break;
                }

                case 'CarStatus': {
                    const data = payload.data as any;
                    const playerIdx = data.player_index;
                    const cars: Record<number, CarStatusDataFrontend> = {};
                    if (data.data) {
                        data.data.forEach((cs: any, idx: number) => {
                            cars[idx] = {
                                tractionControl: cs.m_traction_control,
                                antiLockBrakes: cs.m_anti_lock_brakes,
                                fuelMix: cs.m_fuel_mix,
                                frontBrakeBias: cs.m_front_brake_bias,
                                pitLimiterStatus: cs.m_pit_limiter_status,
                                fuelInTank: cs.m_fuel_in_tank,
                                fuelCapacity: cs.m_fuel_capacity,
                                fuelRemainingLaps: cs.m_fuel_remaining_laps,
                                maxRpm: cs.m_max_rpm,
                                idleRpm: cs.m_idle_rpm,
                                maxGears: cs.m_max_gears,
                                drsAllowed: cs.m_drs_allowed,
                                drsActivationDistance: cs.m_drs_activation_distance,
                                actualTyreCompound: cs.m_actual_tyre_compound.toString(),
                                visualTyreCompound: cs.m_visual_tyre_compound.toString(),
                                tyresAgeLaps: cs.m_tyres_age_laps,
                                vehicleFiaFlags: cs.m_vehicle_fia_flags,
                                ersStoreEnergy: cs.m_ers_store_energy,
                                ersDeployMode: cs.m_ers_deploy_mode,
                                ersHarvestedThisLapMguk: cs.m_ers_harvested_this_lap_mguk,
                                ersDeployedThisLapMguk: cs.m_ers_deployed_this_lap_mguk,
                                ersHarvestedThisLapMguh: cs.m_ers_harvested_this_lap_mguh,
                                ersDeployedThisLapMguh: cs.m_ers_deployed_this_lap_mguh,
                                ersHarvestedThisLapTotal: cs.m_ers_harvested_this_lap_total,
                                ersDeployedThisLapTotal: cs.m_ers_deployed_this_lap_total,
                            };
                        });
                        setCarsStatus(cars);
                    }
                    if (data.data && data.data[playerIdx]) {
                        const cs = data.data[playerIdx];
                        const statusData: CarStatusDataFrontend = {
                            tractionControl: cs.m_traction_control,
                            antiLockBrakes: cs.m_anti_lock_brakes,
                            fuelMix: cs.m_fuel_mix,
                            frontBrakeBias: cs.m_front_brake_bias,
                            pitLimiterStatus: cs.m_pit_limiter_status,
                            fuelInTank: cs.m_fuel_in_tank,
                            fuelCapacity: cs.m_fuel_capacity,
                            fuelRemainingLaps: cs.m_fuel_remaining_laps,
                            maxRpm: cs.m_max_rpm,
                            idleRpm: cs.m_idle_rpm,
                            maxGears: cs.m_max_gears,
                            drsAllowed: cs.m_drs_allowed,
                            drsActivationDistance: cs.m_drs_activation_distance,
                            actualTyreCompound: cs.m_actual_tyre_compound.toString(),
                            visualTyreCompound: cs.m_visual_tyre_compound.toString(),
                            tyresAgeLaps: cs.m_tyres_age_laps,
                            vehicleFiaFlags: cs.m_vehicle_fia_flags,
                            ersStoreEnergy: cs.m_ers_store_energy,
                            ersDeployMode: cs.m_ers_deploy_mode,
                            ersHarvestedThisLapMguk: cs.m_ers_harvested_this_lap_mguk,
                            ersDeployedThisLapMguk: cs.m_ers_deployed_this_lap_mguk,
                            ersHarvestedThisLapMguh: cs.m_ers_harvested_this_lap_mguh,
                            ersDeployedThisLapMguh: cs.m_ers_deployed_this_lap_mguh,
                            ersHarvestedThisLapTotal: cs.m_ers_harvested_this_lap_total,
                            ersDeployedThisLapTotal: cs.m_ers_deployed_this_lap_total,
                        };
                        setCarStatus(statusData);
                    }
                    break;
                }

                case 'LapData': {
                    const data = payload.data as any;
                    const playerIdx = data.player_index;
                    if (data.data && data.data[playerIdx]) {
                        const ld = data.data[playerIdx];
                        const currentLap = ld.m_current_lap_num;
                        const lastLapTime = ld.m_last_lap_time_in_ms;
                        const sectorTimes: [number, number, number] = [
                            ld.m_sector1_time_in_ms,
                            ld.m_sector2_time_in_ms,
                            ld.m_sector3_time_in_ms,
                        ];

                        updateSession({
                            currentLap,
                            lastLapTime,
                            sectorTimes
                        });

                        // New lap completed (lastLapTime > 0 and lap number increased)
                        if (lastLapTime > 0 && currentLap > 1 && ld.m_current_lap_invalid === 0) {
                            const entry = {
                                lapNumber: currentLap - 1,
                                lapTimeMs: lastLapTime,
                                sector1Ms: sectorTimes[0],
                                sector2Ms: sectorTimes[1],
                                sector3Ms: sectorTimes[2],
                                isValid: true,
                                timestamp: Date.now(),
                            };
                            addLapHistory(entry);

                            const state = useTelemetryStore.getState();
                            if (lastLapTime < state.session.bestLapTime) {
                                setBestLap(lastLapTime, sectorTimes);
                            }
                        }
                    }
                    break;
                }

                case 'CarDamage': {
                    const data = payload.data as any;
                    const damage: Record<number, CarDamageDataFrontend> = {};
                    if (data.data) {
                        data.data.forEach((cd: any, idx: number) => {
                            damage[idx] = {
                                tyresWear: cd.m_tyres_wear,
                                tyresDamage: cd.m_tyres_damage,
                                brakesDamage: cd.m_brakes_damage,
                                frontLeftWingDamage: cd.m_front_left_wing_damage,
                                frontRightWingDamage: cd.m_front_right_wing_damage,
                                rearWingDamage: cd.m_rear_wing_damage,
                                floorDamage: cd.m_floor_damage,
                                diffuserDamage: cd.m_diffuser_damage,
                                sidepodDamage: cd.m_sidepod_damage,
                                drsFault: cd.m_drs_fault,
                                gearBoxDamage: cd.m_gear_box_damage,
                                engineDamage: cd.m_engine_damage,
                                engineMguHWear: cd.m_engine_mgu_h_wear,
                                engineMguKWear: cd.m_engine_mgu_k_wear,
                                engineTcWear: cd.m_engine_tc_wear,
                                engineCeWear: cd.m_engine_ce_wear,
                                engineIceWear: cd.m_engine_ice_wear,
                                tyreBlisterRL: cd.m_tyre_blister_rl,
                                tyreBlisterRR: cd.m_tyre_blister_rr,
                                tyreBlisterFL: cd.m_tyre_blister_fl,
                                tyreBlisterFR: cd.m_tyre_blister_fr,
                            };
                        });
                        setCarDamage(damage);
                    }
                    break;
                }

                case 'TyreSets': {
                    const data = payload.data as any;
                    const sets: TyreSetDataFrontend[] = (data.data?.m_tyre_set_data || []).map((s: any) => ({
                        actualTyreCompound: s.m_actual_tyre_compound.toString(),
                        visualTyreCompound: s.m_visual_tyre_compound.toString(),
                        wear: s.m_wear,
                        available: s.m_available,
                        recommendedSession: s.m_recommended_session,
                        lifeSpan: s.m_life_span,
                        usableLife: s.m_usable_life,
                        lapsDone: s.m_laps_done,
                        isNew: s.m_is_new,
                    }));
                    setTyreSets({
                        carIdx: data.data?.m_car_idx ?? 0,
                        sets,
                        fittedIdx: data.data?.m_fitted_idx ?? 0,
                    });
                    break;
                }

                case 'MotionEx': {
                    const data = payload.data as any;
                    const motion: Record<number, MotionExDataFrontend> = {};
                    if (data.data) {
                        data.data.forEach((m: any, idx: number) => {
                            motion[idx] = {
                                suspensionPosition: m.m_suspension_position,
                                suspensionVelocity: m.m_suspension_velocity,
                                suspensionAcceleration: m.m_suspension_acceleration,
                                wheelSpeed: m.m_wheel_speed,
                                wheelSlip: m.m_wheel_slip,
                                localVelocityX: m.m_local_velocity_x,
                                localVelocityY: m.m_local_velocity_y,
                                localVelocityZ: m.m_local_velocity_z,
                                angularVelocityX: m.m_angular_velocity_x,
                                angularVelocityY: m.m_angular_velocity_y,
                                angularVelocityZ: m.m_angular_velocity_z,
                                angularAccelerationX: m.m_angular_acceleration_x,
                                angularAccelerationY: m.m_angular_acceleration_y,
                                angularAccelerationZ: m.m_angular_acceleration_z,
                                frontWheelsAngle: m.m_front_wheels_angle,
                                chassisPitch: m.m_chassis_pitch,
                                wheelCamber: m.m_wheel_camber,
                                wheelCamberGain: m.m_wheel_camber_gain,
                            };
                        });
                        setMotionEx(motion);
                    }
                    break;
                }

                case 'SessionHistory': {
                    const data = payload.data as any;
                    const lapHistory: LapHistoryDataFrontend[] = (data.data?.m_lap_history_data || []).map((l: any) => ({
                        lapTimeInMs: l.m_lap_time_in_ms,
                        sector1TimeInMs: l.m_sector1_time_in_ms,
                        sector2TimeInMs: l.m_sector2_time_in_ms,
                        sector3TimeInMs: l.m_sector3_time_in_ms,
                        lapValidBitFlags: l.m_lap_valid_bit_flags,
                    }));
                    const tyreStintHistory: TyreStintHistoryDataFrontend[] = (data.data?.m_tyre_stint_history_data || []).map((t: any) => ({
                        endLap: t.m_end_lap,
                        tyreActualCompound: t.m_tyre_actual_compound.toString(),
                        tyreVisualCompound: t.m_tyre_visual_compound.toString(),
                    }));
                    setSessionHistory({
                        carIdx: data.data?.m_car_idx ?? 0,
                        numLaps: data.data?.m_num_laps ?? 0,
                        numTyreStints: data.data?.m_num_tyre_stints ?? 0,
                        bestLapTimeLapNum: data.data?.m_best_lap_time_lap_num ?? 0,
                        bestSector1LapNum: data.data?.m_best_sector1_lap_num ?? 0,
                        bestSector2LapNum: data.data?.m_best_sector2_lap_num ?? 0,
                        bestSector3LapNum: data.data?.m_best_sector3_lap_num ?? 0,
                        lapHistory,
                        tyreStintHistory,
                    });
                    break;
                }

                case 'ConnectionStatus': {
                    const data = payload.data as any;
                    updateConnection({
                        packetRate: data.packet_rate,
                        lastPacketTime: data.last_packet_time,
                        connected: data.connected,
                    });
                    break;
                }

                case 'SourceIpDetected': {
                    const data = payload.data as any;
                    setSourceIp(data.ip, data.port);
                    break;
                }

                default:
                    // Unknown event type
                    break;
            }
        }).then((fn) => {
            unlisten = fn;
        });

        return () => {
            if (unlisten) {
                unlisten();
            }
        };
    }, [session.sessionUid, updateSession, setCarTelemetry, setCarStatus, updateConnection, addLapHistory, setBestLap, setParticipants, setActiveDriverIndex, setUnits]);

    return {
        session,
        carTelemetry,
        carStatus,
        connection,
        sourceIp
    };
}