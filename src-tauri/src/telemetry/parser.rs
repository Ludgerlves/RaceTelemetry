//! F1 25 UDP Packet Parser
//! Parses raw UDP bytes into typed packet structures using little-endian format

use crate::telemetry::packets::*;
use nom::{
    bytes::complete::take,
    number::complete::{le_f32, le_i8, le_u16, le_u32, le_u64, le_u8},
    IResult,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Buffer too small: expected {expected}, got {actual}")]
    BufferTooSmall { expected: usize, actual: usize },
    #[error("Invalid packet ID: {0}")]
    InvalidPacketId(u8),
    #[error("Nom parse error: {0}")]
    NomError(String),
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
}

impl<I: std::fmt::Debug> From<nom::Err<nom::error::Error<I>>> for ParseError {
    fn from(e: nom::Err<nom::error::Error<I>>) -> Self {
        ParseError::NomError(format!("{:?}", e))
    }
}

/// Parse the standard 29-byte packet header
pub fn parse_header(input: &[u8]) -> IResult<&[u8], PacketHeader> {
    let (input, m_packet_format) = le_u16(input)?;
    let (input, m_game_year) = le_u8(input)?;
    let (input, m_game_major_version) = le_u8(input)?;
    let (input, m_game_minor_version) = le_u8(input)?;
    let (input, m_packet_version) = le_u8(input)?;
    let (input, m_packet_id) = le_u8(input)?;
    let (input, m_session_uid) = le_u64(input)?;
    let (input, m_session_time) = le_f32(input)?;
    let (input, m_frame_identifier) = le_u32(input)?;
    let (input, m_overall_frame_identifier) = le_u32(input)?;
    let (input, m_player_car_index) = le_u8(input)?;
    let (input, m_secondary_player_car_index) = le_u8(input)?;

    Ok((
        input,
        PacketHeader {
            m_packet_format,
            m_game_year,
            m_game_major_version,
            m_game_minor_version,
            m_packet_version,
            m_packet_id,
            m_session_uid,
            m_session_time,
            m_frame_identifier,
            m_overall_frame_identifier,
            m_player_car_index,
            m_secondary_player_car_index,
        }
    ))
}

/// Parse a single CarTelemetryData (60 bytes)
fn parse_car_telemetry_data(input: &[u8]) -> IResult<&[u8], CarTelemetryData> {
    let (input, m_speed) = le_u16(input)?;
    let (input, m_throttle) = le_f32(input)?;
    let (input, m_brake) = le_f32(input)?;
    let (input, m_steering) = le_f32(input)?;
    let (input, m_gear) = le_i8(input)?;
    let (input, m_engine_rpm) = le_u16(input)?;
    let (input, m_drs) = le_u8(input)?;
    let (input, m_rev_lights_percent) = le_u8(input)?;
    let (input, m_rev_lights_bit_value) = le_u16(input)?;
    let (input, m_brakes_temperature) = le_u16(input)?;
    let (input, m_tyres_surface_temperature) = le_u8(input)?;
    let (input, m_tyres_inner_temperature) = le_u8(input)?;
    let (input, m_engine_temperature) = le_u16(input)?;
    let (input, m_tyres_pressure) = le_f32(input)?;
    let (input, m_surface_type) = le_u8(input)?;

    let m_brakes_temperature = [m_brakes_temperature; 4];
    let m_tyres_surface_temperature = [m_tyres_surface_temperature; 4];
    let m_tyres_inner_temperature = [m_tyres_inner_temperature; 4];
    let m_tyres_pressure = [m_tyres_pressure; 4];
    let m_surface_type = [m_surface_type; 4];

    Ok((
        input,
        CarTelemetryData {
            m_speed,
            m_throttle,
            m_brake,
            m_steering,
            m_gear,
            m_engine_rpm,
            m_drs: match m_drs {
                0 => DrsState::NotAvailable,
                1 => DrsState::Available,
                2 => DrsState::Active,
                _ => DrsState::NotAvailable,
            },
            m_rev_lights_percent,
            m_rev_lights_bit_value,
            m_brakes_temperature,
            m_tyres_surface_temperature,
            m_tyres_inner_temperature,
            m_engine_temperature,
            m_tyres_pressure,
            m_surface_type,
        }
    ))
}

/// Parse PacketCarTelemetryData (ID 6)
fn parse_packet_car_telemetry_data(input: &[u8]) -> IResult<&[u8], PacketCarTelemetryData> {
    let (input, header) = parse_header(input)?;

    let mut m_car_telemetry_data = [CarTelemetryData {
        m_speed: 0,
        m_throttle: 0.0,
        m_brake: 0.0,
        m_steering: 0.0,
        m_gear: 0,
        m_engine_rpm: 0,
        m_drs: DrsState::NotAvailable,
        m_rev_lights_percent: 0,
        m_rev_lights_bit_value: 0,
        m_brakes_temperature: [0; 4],
        m_tyres_surface_temperature: [0; 4],
        m_tyres_inner_temperature: [0; 4],
        m_engine_temperature: 0,
        m_tyres_pressure: [0.0; 4],
        m_surface_type: [0; 4],
    }; MAX_CARS];

    let mut input = input;
    for i in 0..MAX_CARS {
        let (inp, data) = parse_car_telemetry_data(input)?;
        m_car_telemetry_data[i] = data;
        input = inp;
    }

    let (input, m_mfd_panel_index) = le_u8(input)?;
    let (input, m_mfd_panel_index_secondary_player) = le_u8(input)?;
    let (input, m_suggested_gear) = le_i8(input)?;

    Ok((
        input,
        PacketCarTelemetryData {
            m_header: header,
            m_car_telemetry_data,
            m_mfd_panel_index,
            m_mfd_panel_index_secondary_player,
            m_suggested_gear,
        }
    ))
}

/// Parse a single CarStatusData (56 bytes)
fn parse_car_status_data(input: &[u8]) -> IResult<&[u8], CarStatusData> {
    let (input, m_traction_control) = le_u8(input)?;
    let (input, m_anti_lock_brakes) = le_u8(input)?;
    let (input, m_fuel_mix) = le_u8(input)?;
    let (input, m_front_brake_bias) = le_u8(input)?;
    let (input, m_pit_limiter_status) = le_u8(input)?;
    let (input, m_fuel_in_tank) = le_f32(input)?;
    let (input, m_fuel_capacity) = le_f32(input)?;
    let (input, m_fuel_remaining_laps) = le_f32(input)?;
    let (input, m_max_rpm) = le_u16(input)?;
    let (input, m_idle_rpm) = le_u16(input)?;
    let (input, m_max_gears) = le_u8(input)?;
    let (input, m_drs_allowed) = le_u8(input)?;
    let (input, m_drs_activation_distance) = le_u16(input)?;
    let (input, m_actual_tyre_compound) = le_u8(input)?;
    let (input, m_visual_tyre_compound) = le_u8(input)?;
    let (input, m_tyres_age_laps) = le_u8(input)?;
    let (input, m_vehicle_fia_flags) = le_i8(input)?;
    let (input, m_ers_store_energy) = le_f32(input)?;
    let (input, m_ers_deploy_mode) = le_u8(input)?;
    let (input, m_ers_harvested_this_lap_mguk) = le_f32(input)?;
    let (input, m_ers_deployed_this_lap_mguk) = le_f32(input)?;
    let (input, m_ers_harvested_this_lap_mguh) = le_f32(input)?;
    let (input, m_ers_deployed_this_lap_mguh) = le_f32(input)?;
    let (input, m_ers_harvested_this_lap_total) = le_f32(input)?;
    let (input, m_ers_deployed_this_lap_total) = le_f32(input)?;

    Ok((
        input,
        CarStatusData {
            m_traction_control,
            m_anti_lock_brakes,
            m_fuel_mix: match m_fuel_mix {
                0 => EngineMode::Lean,
                1 => EngineMode::Standard,
                2 => EngineMode::Rich,
                _ => EngineMode::Standard,
            },
            m_front_brake_bias,
            m_pit_limiter_status,
            m_fuel_in_tank,
            m_fuel_capacity,
            m_fuel_remaining_laps,
            m_max_rpm,
            m_idle_rpm,
            m_max_gears,
            m_drs_allowed,
            m_drs_activation_distance,
            m_actual_tyre_compound: match m_actual_tyre_compound {
                0 => TyreCompound::C0,
                1 => TyreCompound::C1,
                2 => TyreCompound::C2,
                3 => TyreCompound::C3,
                4 => TyreCompound::C4,
                5 => TyreCompound::C5,
                6 => TyreCompound::C6,
                7 => TyreCompound::Inter,
                8 => TyreCompound::Wet,
                _ => TyreCompound::C0,
            },
            m_visual_tyre_compound: match m_visual_tyre_compound {
                0 => VisualTyreCompound::Soft,
                1 => VisualTyreCompound::Medium,
                2 => VisualTyreCompound::Hard,
                3 => VisualTyreCompound::Intermediate,
                4 => VisualTyreCompound::Wet,
                _ => VisualTyreCompound::Hard,
            },
            m_tyres_age_laps,
            m_vehicle_fia_flags,
            m_ers_store_energy,
            m_ers_deploy_mode: match m_ers_deploy_mode {
                0 => ErsMode::None,
                1 => ErsMode::Medium,
                2 => ErsMode::Hotlap,
                3 => ErsMode::Overtake,
                _ => ErsMode::None,
            },
            m_ers_harvested_this_lap_mguk,
            m_ers_deployed_this_lap_mguk,
            m_ers_harvested_this_lap_mguh,
            m_ers_deployed_this_lap_mguh,
            m_ers_harvested_this_lap_total,
            m_ers_deployed_this_lap_total,
        }
    ))
}

/// Parse PacketCarStatusData (ID 7)
fn parse_packet_car_status_data(input: &[u8]) -> IResult<&[u8], PacketCarStatusData> {
    let (input, header) = parse_header(input)?;

    let mut m_car_status_data = [CarStatusData {
        m_traction_control: 0,
        m_anti_lock_brakes: 0,
        m_fuel_mix: EngineMode::Standard,
        m_front_brake_bias: 0,
        m_pit_limiter_status: 0,
        m_fuel_in_tank: 0.0,
        m_fuel_capacity: 0.0,
        m_fuel_remaining_laps: 0.0,
        m_max_rpm: 0,
        m_idle_rpm: 0,
        m_max_gears: 0,
        m_drs_allowed: 0,
        m_drs_activation_distance: 0,
        m_actual_tyre_compound: TyreCompound::C0,
        m_visual_tyre_compound: VisualTyreCompound::Hard,
        m_tyres_age_laps: 0,
        m_vehicle_fia_flags: 0,
        m_ers_store_energy: 0.0,
        m_ers_deploy_mode: ErsMode::None,
        m_ers_harvested_this_lap_mguk: 0.0,
        m_ers_deployed_this_lap_mguk: 0.0,
        m_ers_harvested_this_lap_mguh: 0.0,
        m_ers_deployed_this_lap_mguh: 0.0,
        m_ers_harvested_this_lap_total: 0.0,
        m_ers_deployed_this_lap_total: 0.0,
    }; MAX_CARS];

    let mut input = input;
    for i in 0..MAX_CARS {
        let (inp, data) = parse_car_status_data(input)?;
        m_car_status_data[i] = data;
        input = inp;
    }

    Ok((
        input,
        PacketCarStatusData {
            m_header: header,
            m_car_status_data,
        }
    ))
}

/// Parse PacketSessionData (ID 1)
fn parse_packet_session_data(input: &[u8]) -> IResult<&[u8], PacketSessionData> {
    let (input, header) = parse_header(input)?;

    let (input, m_weather) = le_u8(input)?;
    let (input, m_track_temperature) = le_i8(input)?;
    let (input, m_air_temperature) = le_i8(input)?;
    let (input, m_total_laps) = le_u8(input)?;
    let (input, m_track_length) = le_u16(input)?;
    let (input, m_session_type) = le_u8(input)?;
    let (input, m_track_id) = le_u8(input)?;
    let (input, m_formula) = le_u8(input)?;
    let (input, m_session_time_left) = le_u16(input)?;
    let (input, m_session_duration) = le_u16(input)?;
    let (input, m_pit_speed_limit) = le_u8(input)?;
    let (input, m_game_paused) = le_u8(input)?;
    let (input, m_is_spectating) = le_u8(input)?;
    let (input, m_spectator_car_index) = le_u8(input)?;
    let (input, m_sli_pro_native_support) = le_u8(input)?;

    let weather = match m_weather {
        0 => Weather::Clear,
        1 => Weather::LightCloud,
        2 => Weather::Overcast,
        3 => Weather::LightRain,
        4 => Weather::HeavyRain,
        5 => Weather::Storm,
        _ => Weather::Clear,
    };

    let session_type = match m_session_type {
        1 => SessionType::Practice1,
        2 => SessionType::Practice2,
        3 => SessionType::Practice3,
        4 => SessionType::ShortPractice,
        5 => SessionType::Qualifying1,
        6 => SessionType::Qualifying2,
        7 => SessionType::Qualifying3,
        8 => SessionType::ShortQualifying,
        9 => SessionType::OneShotQualifying,
        10 => SessionType::Race,
        11 => SessionType::Race2,
        12 => SessionType::Race3,
        13 => SessionType::TimeTrial,
        _ => SessionType::Unknown,
    };

    let track_id = match m_track_id {
        0 => TrackId::Melbourne,
        1 => TrackId::PaulRicard,
        2 => TrackId::Shanghai,
        3 => TrackId::Sakhir,
        4 => TrackId::Catalonia,
        5 => TrackId::Monaco,
        6 => TrackId::Montreal,
        7 => TrackId::Silverstone,
        8 => TrackId::Hockenheim,
        9 => TrackId::Hungaroring,
        10 => TrackId::Spa,
        11 => TrackId::Monza,
        12 => TrackId::Singapore,
        13 => TrackId::Suzuka,
        14 => TrackId::AbuDhabi,
        15 => TrackId::Texas,
        16 => TrackId::Brazil,
        17 => TrackId::Austria,
        18 => TrackId::Sochi,
        19 => TrackId::Mexico,
        20 => TrackId::Baku,
        21 => TrackId::Zandvoort,
        22 => TrackId::Imola,
        23 => TrackId::Portimao,
        24 => TrackId::Jeddah,
        25 => TrackId::Miami,
        26 => TrackId::LasVegas,
        27 => TrackId::Lusail,
        _ => TrackId::Melbourne,
    };

    let formula = match m_formula {
        0 => Formula::F1Modern,
        1 => Formula::F1Classic,
        2 => Formula::F2,
        3 => Formula::F1Generic,
        4 => Formula::Beta,
        5 => Formula::F1Esports,
        6 => Formula::F2_2023,
        _ => Formula::F1Modern,
    };

    let (input, m_num_marshal_zones) = le_u8(input)?;

    let mut m_marshal_zones = [MarshalZone { m_zone_start: 0.0, m_zone_flag: 0 }; 21];
    let mut input = input;
    for i in 0..m_num_marshal_zones as usize {
        let (inp, zone_start) = le_f32(input)?;
        let (inp, zone_flag) = le_i8(inp)?;
        m_marshal_zones[i] = MarshalZone { m_zone_start: zone_start, m_zone_flag: zone_flag };
        input = inp;
    }

    let (input, m_safety_car_status) = le_u8(input)?;
    let (input, m_network_game) = le_u8(input)?;
    let (input, m_num_weather_forecast_samples) = le_u8(input)?;

    let mut m_weather_forecast_samples = [WeatherForecastSample {
        m_session_type: 0,
        m_time_offset: 0,
        m_weather: Weather::Clear,
        m_track_temperature: 0,
        m_track_temperature_change: 0,
        m_air_temperature: 0,
        m_air_temperature_change: 0,
        m_rain_percentage: 0,
    }; 56];

    let mut input = input;
    for i in 0..m_num_weather_forecast_samples as usize {
        let (inp, m_session_type) = le_u8(input)?;
        let (inp, m_time_offset) = le_u8(inp)?;
        let (inp, m_weather) = le_u8(inp)?;
        let (inp, m_track_temperature) = le_i8(inp)?;
        let (inp, m_track_temperature_change) = le_i8(inp)?;
        let (inp, m_air_temperature) = le_i8(inp)?;
        let (inp, m_air_temperature_change) = le_i8(inp)?;
        let (inp, m_rain_percentage) = le_u8(inp)?;

        let weather = match m_weather {
            0 => Weather::Clear,
            1 => Weather::LightCloud,
            2 => Weather::Overcast,
            3 => Weather::LightRain,
            4 => Weather::HeavyRain,
            5 => Weather::Storm,
            _ => Weather::Clear,
        };

        m_weather_forecast_samples[i] = WeatherForecastSample {
            m_session_type,
            m_time_offset,
            m_weather: weather,
            m_track_temperature,
            m_track_temperature_change,
            m_air_temperature,
            m_air_temperature_change,
            m_rain_percentage,
        };
        input = inp;
    }

    let (input, m_forecast_accuracy) = le_u8(input)?;
    let (input, m_ai_difficulty) = le_u8(input)?;
    let (input, m_season_link_identifier) = le_u32(input)?;
    let (input, m_weekend_link_identifier) = le_u32(input)?;
    let (input, m_session_link_identifier) = le_u32(input)?;
    let (input, m_pit_stop_window_ideal_lap) = le_u8(input)?;
    let (input, m_pit_stop_window_latest_lap) = le_u8(input)?;
    let (input, m_pit_stop_rejoin_position) = le_u8(input)?;
    let (input, m_steering_assist) = le_u8(input)?;
    let (input, m_braking_assist) = le_u8(input)?;
    let (input, m_gearbox_assist) = le_u8(input)?;
    let (input, m_pit_assist) = le_u8(input)?;
    let (input, m_pit_release_assist) = le_u8(input)?;
    let (input, m_ers_assist) = le_u8(input)?;
    let (input, m_drs_assist) = le_u8(input)?;
    let (input, m_dynamic_racing_line) = le_u8(input)?;
    let (input, m_dynamic_racing_line_type) = le_u8(input)?;
    let (input, m_game_mode) = le_u8(input)?;
    let (input, m_rule_set) = le_u8(input)?;
    let (input, m_time_of_day) = le_u32(input)?;
    let (input, m_session_length) = le_u8(input)?;

    Ok((
        input,
        PacketSessionData {
            m_header: header,
            m_weather: weather,
            m_track_temperature,
            m_air_temperature,
            m_total_laps,
            m_track_length,
            m_session_type: session_type,
            m_track_id: track_id,
            m_formula: formula,
            m_session_time_left,
            m_session_duration,
            m_pit_speed_limit,
            m_game_paused,
            m_is_spectating,
            m_spectator_car_index,
            m_sli_pro_native_support,
            m_num_marshal_zones,
            m_marshal_zones,
            m_safety_car_status,
            m_network_game,
            m_num_weather_forecast_samples,
            m_weather_forecast_samples,
            m_forecast_accuracy,
            m_ai_difficulty,
            m_season_link_identifier,
            m_weekend_link_identifier,
            m_session_link_identifier,
            m_pit_stop_window_ideal_lap,
            m_pit_stop_window_latest_lap,
            m_pit_stop_rejoin_position,
            m_steering_assist,
            m_braking_assist,
            m_gearbox_assist,
            m_pit_assist,
            m_pit_release_assist,
            m_ers_assist,
            m_drs_assist,
            m_dynamic_racing_line,
            m_dynamic_racing_line_type,
            m_game_mode,
            m_rule_set,
            m_time_of_day,
            m_session_length,
        }
    ))
}

/// Parse PacketLapData (ID 2)
fn parse_packet_lap_data(input: &[u8]) -> IResult<&[u8], PacketLapData> {
    let (input, header) = parse_header(input)?;

    let mut m_lap_data = [LapData {
        m_last_lap_time_in_ms: 0,
        m_current_lap_time_in_ms: 0,
        m_sector1_time_in_ms: 0,
        m_sector2_time_in_ms: 0,
        m_sector3_time_in_ms: 0,
        m_lap_distance: 0.0,
        m_total_distance: 0.0,
        m_safety_car_delta: 0.0,
        m_car_position: 0,
        m_current_lap_num: 0,
        m_pit_status: 0,
        m_num_pit_stops: 0,
        m_sector: 0,
        m_current_lap_invalid: 0,
        m_penalties: 0,
        m_warnings: 0,
        m_num_unserved_drive_through_penalties: 0,
        m_num_unserved_stop_go_penalties: 0,
        m_grid_position: 0,
        m_driver_status: 0,
        m_result_status: 0,
        m_pit_lane_timer_active: 0,
        m_pit_lane_time_lane_in_ms: 0,
        m_pit_stop_timer_in_ms: 0,
        m_pit_stop_should_serve_pen: 0,
    }; MAX_CARS];

    let mut input = input;
    for i in 0..MAX_CARS {
        let (inp, m_last_lap_time_in_ms) = le_u32(input)?;
        let (inp, m_current_lap_time_in_ms) = le_u32(inp)?;
        let (inp, m_sector1_time_in_ms) = le_u16(inp)?;
        let (inp, m_sector2_time_in_ms) = le_u16(inp)?;
        let (inp, m_sector3_time_in_ms) = le_u16(inp)?;
        let (inp, m_lap_distance) = le_f32(inp)?;
        let (inp, m_total_distance) = le_f32(inp)?;
        let (inp, m_safety_car_delta) = le_f32(inp)?;
        let (inp, m_car_position) = le_u8(inp)?;
        let (inp, m_current_lap_num) = le_u8(inp)?;
        let (inp, m_pit_status) = le_u8(inp)?;
        let (inp, m_num_pit_stops) = le_u8(inp)?;
        let (inp, m_sector) = le_u8(inp)?;
        let (inp, m_current_lap_invalid) = le_u8(inp)?;
        let (inp, m_penalties) = le_u8(inp)?;
        let (inp, m_warnings) = le_u8(inp)?;
        let (inp, m_num_unserved_drive_through_penalties) = le_u8(inp)?;
        let (inp, m_num_unserved_stop_go_penalties) = le_u8(inp)?;
        let (inp, m_grid_position) = le_u8(inp)?;
        let (inp, m_driver_status) = le_u8(inp)?;
        let (inp, m_result_status) = le_u8(inp)?;
        let (inp, m_pit_lane_timer_active) = le_u8(inp)?;
        let (inp, m_pit_lane_time_lane_in_ms) = le_u16(inp)?;
        let (inp, m_pit_stop_timer_in_ms) = le_u16(inp)?;
        let (inp, m_pit_stop_should_serve_pen) = le_u8(inp)?;

        m_lap_data[i] = LapData {
            m_last_lap_time_in_ms,
            m_current_lap_time_in_ms,
            m_sector1_time_in_ms,
            m_sector2_time_in_ms,
            m_sector3_time_in_ms,
            m_lap_distance,
            m_total_distance,
            m_safety_car_delta,
            m_car_position,
            m_current_lap_num,
            m_pit_status,
            m_num_pit_stops,
            m_sector,
            m_current_lap_invalid,
            m_penalties,
            m_warnings,
            m_num_unserved_drive_through_penalties,
            m_num_unserved_stop_go_penalties,
            m_grid_position,
            m_driver_status,
            m_result_status,
            m_pit_lane_timer_active,
            m_pit_lane_time_lane_in_ms,
            m_pit_stop_timer_in_ms,
            m_pit_stop_should_serve_pen,
        };
        input = inp;
    }

    let (input, m_time_trial_personal_best_car_idx) = le_u8(input)?;
    let (input, m_time_trial_rival_car_idx) = le_u8(input)?;

    Ok((
        input,
        PacketLapData {
            m_header: header,
            m_lap_data,
            m_time_trial_personal_best_car_idx,
            m_time_trial_rival_car_idx,
        }
    ))
}

/// Parse PacketParticipantsData (ID 4)
fn parse_packet_participants_data(input: &[u8]) -> IResult<&[u8], PacketParticipantsData> {
    let (input, header) = parse_header(input)?;
    let (input, m_num_active_cars) = le_u8(input)?;

    let mut m_participants = [ParticipantData {
        m_ai_controlled: 0,
        m_driver_id: 0,
        m_network_id: 0,
        m_team_id: TeamId::Mercedes,
        m_my_team: 0,
        m_race_number: 0,
        m_nationality: 0,
        m_name: [0; 48],
        m_your_telemetry: 0,
        m_show_online_names: 0,
        m_platform: 0,
        m_car_colour_primary: 0,
        m_car_colour_secondary: 0,
    }; MAX_CARS];

    let mut input = input;
    for i in 0..MAX_CARS {
        let (inp, m_ai_controlled) = le_u8(input)?;
        let (inp, m_driver_id) = le_u8(inp)?;
        let (inp, m_network_id) = le_u8(inp)?;
        let (inp, m_team_id) = le_u8(inp)?;
        let (inp, m_my_team) = le_u8(inp)?;
        let (inp, m_race_number) = le_u8(inp)?;
        let (inp, m_nationality) = le_u8(inp)?;
        let (inp, m_name) = take(48usize)(inp)?;
        let (inp, m_your_telemetry) = le_u8(inp)?;
        let (inp, m_show_online_names) = le_u8(inp)?;
        let (inp, m_platform) = le_u8(inp)?;
        let (inp, m_car_colour_primary) = le_u8(inp)?;
        let (inp, m_car_colour_secondary) = le_u8(inp)?;

        m_participants[i] = ParticipantData {
            m_ai_controlled,
            m_driver_id,
            m_network_id,
            m_team_id: match m_team_id {
                0 => TeamId::Mercedes,
                1 => TeamId::Ferrari,
                2 => TeamId::RedBull,
                3 => TeamId::Williams,
                4 => TeamId::RacingBulls,
                5 => TeamId::Alpine,
                6 => TeamId::AstonMartin,
                7 => TeamId::Haas,
                8 => TeamId::Mclaren,
                9 => TeamId::Sauber,
                _ => TeamId::Mercedes,
            },
            m_my_team,
            m_race_number,
            m_nationality,
            m_name: m_name.try_into().unwrap_or([0; 48]),
            m_your_telemetry,
            m_show_online_names,
            m_platform,
            m_car_colour_primary,
            m_car_colour_secondary,
        };
        input = inp;
    }

    Ok((
        input,
        PacketParticipantsData {
            m_header: header,
            m_num_active_cars,
            m_participants,
        }
    ))
}

/// Parse PacketEventData (ID 3)
fn parse_packet_event_data(input: &[u8]) -> IResult<&[u8], PacketEventData> {
    let (input, header) = parse_header(input)?;
    let (input, m_event_string_code) = take(4usize)(input)?;
    let (input, m_event_details) = take(16usize)(input)?;

    let m_event_string_code: [u8; 4] = m_event_string_code.try_into().unwrap_or([0; 4]);
    let m_event_details: [u8; 16] = m_event_details.try_into().unwrap_or([0; 16]);

    Ok((
        input,
        PacketEventData {
            m_header: header,
            m_event_string_code,
            m_event_details: EventDataDetails { m_data: m_event_details },
        }
    ))
}

/// Parse PacketCarDamageData (ID 10)
fn parse_packet_car_damage_data(input: &[u8]) -> IResult<&[u8], PacketCarDamageData> {
    let (input, header) = parse_header(input)?;

    let mut m_car_damage_data = [CarDamageData {
        m_tyres_wear: [0.0; 4],
        m_tyres_damage: [0; 4],
        m_brakes_damage: [0; 4],
        m_front_left_wing_damage: 0,
        m_front_right_wing_damage: 0,
        m_rear_wing_damage: 0,
        m_floor_damage: 0,
        m_diffuser_damage: 0,
        m_sidepod_damage: 0,
        m_drs_fault: 0,
        m_gear_box_damage: 0,
        m_engine_damage: 0,
        m_engine_mgu_h_wear: 0,
        m_engine_mgu_k_wear: 0,
        m_engine_tc_wear: 0,
        m_engine_ce_wear: 0,
        m_engine_ice_wear: 0,
        m_tyre_blister_rl: 0,
        m_tyre_blister_rr: 0,
        m_tyre_blister_fl: 0,
        m_tyre_blister_fr: 0,
    }; MAX_CARS];

    let mut input = input;
    for i in 0..MAX_CARS {
        let (inp, m_tyres_wear_rl) = le_f32(input)?;
        let (inp, m_tyres_wear_rr) = le_f32(inp)?;
        let (inp, m_tyres_wear_fl) = le_f32(inp)?;
        let (inp, m_tyres_wear_fr) = le_f32(inp)?;
        let (inp, m_tyres_damage_rl) = le_u8(inp)?;
        let (inp, m_tyres_damage_rr) = le_u8(inp)?;
        let (inp, m_tyres_damage_fl) = le_u8(inp)?;
        let (inp, m_tyres_damage_fr) = le_u8(inp)?;
        let (inp, m_brakes_damage_rl) = le_u8(inp)?;
        let (inp, m_brakes_damage_rr) = le_u8(inp)?;
        let (inp, m_brakes_damage_fl) = le_u8(inp)?;
        let (inp, m_brakes_damage_fr) = le_u8(inp)?;
        let (inp, m_front_left_wing_damage) = le_u8(inp)?;
        let (inp, m_front_right_wing_damage) = le_u8(inp)?;
        let (inp, m_rear_wing_damage) = le_u8(inp)?;
        let (inp, m_floor_damage) = le_u8(inp)?;
        let (inp, m_diffuser_damage) = le_u8(inp)?;
        let (inp, m_sidepod_damage) = le_u8(inp)?;
        let (inp, m_drs_fault) = le_u8(inp)?;
        let (inp, m_gear_box_damage) = le_u8(inp)?;
        let (inp, m_engine_damage) = le_u8(inp)?;
        let (inp, m_engine_mgu_h_wear) = le_u8(inp)?;
        let (inp, m_engine_mgu_k_wear) = le_u8(inp)?;
        let (inp, m_engine_tc_wear) = le_u8(inp)?;
        let (inp, m_engine_ce_wear) = le_u8(inp)?;
        let (inp, m_engine_ice_wear) = le_u8(inp)?;
        let (inp, m_tyre_blister_rl) = le_u8(inp)?;
        let (inp, m_tyre_blister_rr) = le_u8(inp)?;
        let (inp, m_tyre_blister_fl) = le_u8(inp)?;
        let (inp, m_tyre_blister_fr) = le_u8(inp)?;

        m_car_damage_data[i] = CarDamageData {
            m_tyres_wear: [m_tyres_wear_rl, m_tyres_wear_rr, m_tyres_wear_fl, m_tyres_wear_fr],
            m_tyres_damage: [m_tyres_damage_rl, m_tyres_damage_rr, m_tyres_damage_fl, m_tyres_damage_fr],
            m_brakes_damage: [m_brakes_damage_rl, m_brakes_damage_rr, m_brakes_damage_fl, m_brakes_damage_fr],
            m_front_left_wing_damage,
            m_front_right_wing_damage,
            m_rear_wing_damage,
            m_floor_damage,
            m_diffuser_damage,
            m_sidepod_damage,
            m_drs_fault,
            m_gear_box_damage,
            m_engine_damage,
            m_engine_mgu_h_wear,
            m_engine_mgu_k_wear,
            m_engine_tc_wear,
            m_engine_ce_wear,
            m_engine_ice_wear,
            m_tyre_blister_rl,
            m_tyre_blister_rr,
            m_tyre_blister_fl,
            m_tyre_blister_fr,
        };
        input = inp;
    }

    Ok((
        input,
        PacketCarDamageData {
            m_header: header,
            m_car_damage_data,
        }
    ))
}

/// Parse PacketTyreSetsData (ID 12)
fn parse_packet_tyre_sets_data(input: &[u8]) -> IResult<&[u8], PacketTyreSetsData> {
    let (input, header) = parse_header(input)?;
    let (input, m_car_idx) = le_u8(input)?;

    let mut m_tyre_set_data = [TyreSetData {
        m_actual_tyre_compound: TyreCompound::C0,
        m_visual_tyre_compound: VisualTyreCompound::Hard,
        m_wear: 0,
        m_available: 0,
        m_recommended_session: 0,
        m_life_span: 0,
        m_usable_life: 0,
        m_laps_done: 0,
        m_is_new: 0,
    }; 20];

    let mut input = input;
    for i in 0..20 {
        let (inp, m_actual_tyre_compound) = le_u8(input)?;
        let (inp, m_visual_tyre_compound) = le_u8(inp)?;
        let (inp, m_wear) = le_u8(inp)?;
        let (inp, m_available) = le_u8(inp)?;
        let (inp, m_recommended_session) = le_u8(inp)?;
        let (inp, m_life_span) = le_u8(inp)?;
        let (inp, m_usable_life) = le_u8(inp)?;
        let (inp, m_laps_done) = le_u8(inp)?;
        let (inp, m_is_new) = le_u8(inp)?;

        m_tyre_set_data[i] = TyreSetData {
            m_actual_tyre_compound: match m_actual_tyre_compound {
                0 => TyreCompound::C0, 1 => TyreCompound::C1, 2 => TyreCompound::C2,
                3 => TyreCompound::C3, 4 => TyreCompound::C4, 5 => TyreCompound::C5,
                6 => TyreCompound::C6, 7 => TyreCompound::Inter, 8 => TyreCompound::Wet,
                _ => TyreCompound::C0,
            },
            m_visual_tyre_compound: match m_visual_tyre_compound {
                0 => VisualTyreCompound::Soft, 1 => VisualTyreCompound::Medium,
                2 => VisualTyreCompound::Hard, 3 => VisualTyreCompound::Intermediate,
                4 => VisualTyreCompound::Wet, _ => VisualTyreCompound::Hard,
            },
            m_wear,
            m_available,
            m_recommended_session,
            m_life_span,
            m_usable_life,
            m_laps_done,
            m_is_new,
        };
        input = inp;
    }

    let (input, m_fitted_idx) = le_u8(input)?;

    Ok((
        input,
        PacketTyreSetsData {
            m_header: header,
            m_car_idx,
            m_tyre_set_data,
            m_fitted_idx,
        }
    ))
}

/// Parse PacketMotionExData (ID 13)
fn parse_packet_motion_ex_data(input: &[u8]) -> IResult<&[u8], PacketMotionExData> {
    let (input, header) = parse_header(input)?;

    let mut m_motion_ex_data = [MotionExData {
        m_suspension_position: [0.0; 4],
        m_suspension_velocity: [0.0; 4],
        m_suspension_acceleration: [0.0; 4],
        m_wheel_speed: [0.0; 4],
        m_wheel_slip: [0.0; 4],
        m_local_velocity_x: 0.0,
        m_local_velocity_y: 0.0,
        m_local_velocity_z: 0.0,
        m_angular_velocity_x: 0.0,
        m_angular_velocity_y: 0.0,
        m_angular_velocity_z: 0.0,
        m_angular_acceleration_x: 0.0,
        m_angular_acceleration_y: 0.0,
        m_angular_acceleration_z: 0.0,
        m_front_wheels_angle: 0.0,
        m_chassis_pitch: 0.0,
        m_wheel_camber: [0.0; 4],
        m_wheel_camber_gain: [0.0; 4],
    }; MAX_CARS];

    let mut input = input;
    for i in 0..MAX_CARS {
        let (inp, m_suspension_position_rl) = le_f32(input)?;
        let (inp, m_suspension_position_rr) = le_f32(inp)?;
        let (inp, m_suspension_position_fl) = le_f32(inp)?;
        let (inp, m_suspension_position_fr) = le_f32(inp)?;
        let (inp, m_suspension_velocity_rl) = le_f32(inp)?;
        let (inp, m_suspension_velocity_rr) = le_f32(inp)?;
        let (inp, m_suspension_velocity_fl) = le_f32(inp)?;
        let (inp, m_suspension_velocity_fr) = le_f32(inp)?;
        let (inp, m_suspension_acceleration_rl) = le_f32(inp)?;
        let (inp, m_suspension_acceleration_rr) = le_f32(inp)?;
        let (inp, m_suspension_acceleration_fl) = le_f32(inp)?;
        let (inp, m_suspension_acceleration_fr) = le_f32(inp)?;
        let (inp, m_wheel_speed_rl) = le_f32(inp)?;
        let (inp, m_wheel_speed_rr) = le_f32(inp)?;
        let (inp, m_wheel_speed_fl) = le_f32(inp)?;
        let (inp, m_wheel_speed_fr) = le_f32(inp)?;
        let (inp, m_wheel_slip_rl) = le_f32(inp)?;
        let (inp, m_wheel_slip_rr) = le_f32(inp)?;
        let (inp, m_wheel_slip_fl) = le_f32(inp)?;
        let (inp, m_wheel_slip_fr) = le_f32(inp)?;
        let (inp, m_local_velocity_x) = le_f32(inp)?;
        let (inp, m_local_velocity_y) = le_f32(inp)?;
        let (inp, m_local_velocity_z) = le_f32(inp)?;
        let (inp, m_angular_velocity_x) = le_f32(inp)?;
        let (inp, m_angular_velocity_y) = le_f32(inp)?;
        let (inp, m_angular_velocity_z) = le_f32(inp)?;
        let (inp, m_angular_acceleration_x) = le_f32(inp)?;
        let (inp, m_angular_acceleration_y) = le_f32(inp)?;
        let (inp, m_angular_acceleration_z) = le_f32(inp)?;
        let (inp, m_front_wheels_angle) = le_f32(inp)?;
        let (inp, m_chassis_pitch) = le_f32(inp)?;
        let (inp, m_wheel_camber_rl) = le_f32(inp)?;
        let (inp, m_wheel_camber_rr) = le_f32(inp)?;
        let (inp, m_wheel_camber_fl) = le_f32(inp)?;
        let (inp, m_wheel_camber_fr) = le_f32(inp)?;
        let (inp, m_wheel_camber_gain_rl) = le_f32(inp)?;
        let (inp, m_wheel_camber_gain_rr) = le_f32(inp)?;
        let (inp, m_wheel_camber_gain_fl) = le_f32(inp)?;
        let (inp, m_wheel_camber_gain_fr) = le_f32(inp)?;

        m_motion_ex_data[i] = MotionExData {
            m_suspension_position: [m_suspension_position_rl, m_suspension_position_rr, m_suspension_position_fl, m_suspension_position_fr],
            m_suspension_velocity: [m_suspension_velocity_rl, m_suspension_velocity_rr, m_suspension_velocity_fl, m_suspension_velocity_fr],
            m_suspension_acceleration: [m_suspension_acceleration_rl, m_suspension_acceleration_rr, m_suspension_acceleration_fl, m_suspension_acceleration_fr],
            m_wheel_speed: [m_wheel_speed_rl, m_wheel_speed_rr, m_wheel_speed_fl, m_wheel_speed_fr],
            m_wheel_slip: [m_wheel_slip_rl, m_wheel_slip_rr, m_wheel_slip_fl, m_wheel_slip_fr],
            m_local_velocity_x,
            m_local_velocity_y,
            m_local_velocity_z,
            m_angular_velocity_x,
            m_angular_velocity_y,
            m_angular_velocity_z,
            m_angular_acceleration_x,
            m_angular_acceleration_y,
            m_angular_acceleration_z,
            m_front_wheels_angle,
            m_chassis_pitch,
            m_wheel_camber: [m_wheel_camber_rl, m_wheel_camber_rr, m_wheel_camber_fl, m_wheel_camber_fr],
            m_wheel_camber_gain: [m_wheel_camber_gain_rl, m_wheel_camber_gain_rr, m_wheel_camber_gain_fl, m_wheel_camber_gain_fr],
        };
        input = inp;
    }

    Ok((
        input,
        PacketMotionExData {
            m_header: header,
            m_motion_ex_data,
        }
    ))
}

/// Parse PacketSessionHistoryData (ID 11)
fn parse_packet_session_history_data(input: &[u8]) -> IResult<&[u8], PacketSessionHistoryData> {
    let (input, header) = parse_header(input)?;
    let (input, m_car_idx) = le_u8(input)?;
    let (input, m_num_laps) = le_u8(input)?;
    let (input, m_num_tyre_stints) = le_u8(input)?;
    let (input, m_best_lap_time_lap_num) = le_u8(input)?;
    let (input, m_best_sector1_lap_num) = le_u8(input)?;
    let (input, m_best_sector2_lap_num) = le_u8(input)?;
    let (input, m_best_sector3_lap_num) = le_u8(input)?;

    let mut m_lap_history_data = [LapHistoryData {
        m_lap_time_in_ms: 0,
        m_sector1_time_in_ms: 0,
        m_sector2_time_in_ms: 0,
        m_sector3_time_in_ms: 0,
        m_lap_valid_bit_flags: 0,
    }; 100];

    let mut input = input;
    for i in 0..100 {
        let (inp, m_lap_time_in_ms) = le_u32(input)?;
        let (inp, m_sector1_time_in_ms) = le_u16(inp)?;
        let (inp, m_sector2_time_in_ms) = le_u16(inp)?;
        let (inp, m_sector3_time_in_ms) = le_u16(inp)?;
        let (inp, m_lap_valid_bit_flags) = le_u8(inp)?;

        m_lap_history_data[i] = LapHistoryData {
            m_lap_time_in_ms,
            m_sector1_time_in_ms,
            m_sector2_time_in_ms,
            m_sector3_time_in_ms,
            m_lap_valid_bit_flags,
        };
        input = inp;
    }

    let mut m_tyre_stint_history_data = [TyreStintHistoryData {
        m_end_lap: 0,
        m_tyre_actual_compound: TyreCompound::C0,
        m_tyre_visual_compound: VisualTyreCompound::Hard,
    }; 8];

    for i in 0..8 {
        let (inp, m_end_lap) = le_u8(input)?;
        let (inp, m_tyre_actual_compound) = le_u8(inp)?;
        let (inp, m_tyre_visual_compound) = le_u8(inp)?;

        m_tyre_stint_history_data[i] = TyreStintHistoryData {
            m_end_lap,
            m_tyre_actual_compound: match m_tyre_actual_compound {
                0 => TyreCompound::C0, 1 => TyreCompound::C1, 2 => TyreCompound::C2,
                3 => TyreCompound::C3, 4 => TyreCompound::C4, 5 => TyreCompound::C5,
                6 => TyreCompound::C6, 7 => TyreCompound::Inter, 8 => TyreCompound::Wet,
                _ => TyreCompound::C0,
            },
            m_tyre_visual_compound: match m_tyre_visual_compound {
                0 => VisualTyreCompound::Soft, 1 => VisualTyreCompound::Medium,
                2 => VisualTyreCompound::Hard, 3 => VisualTyreCompound::Intermediate,
                4 => VisualTyreCompound::Wet, _ => VisualTyreCompound::Hard,
            },
        };
        input = inp;
    }

    Ok((
        input,
        PacketSessionHistoryData {
            m_header: header,
            m_car_idx,
            m_num_laps,
            m_num_tyre_stints,
            m_best_lap_time_lap_num,
            m_best_sector1_lap_num,
            m_best_sector2_lap_num,
            m_best_sector3_lap_num,
            m_lap_history_data,
            m_tyre_stint_history_data,
        }
    ))
}

/// Main packet parser - dispatches to appropriate parser based on packet ID
pub fn parse_packet(input: &[u8]) -> Result<TelemetryEvent, ParseError> {
    if input.len() < 29 {
        return Err(ParseError::BufferTooSmall {
            expected: 29,
            actual: input.len(),
        });
    }

    // Parse header first to get packet ID
    let (_, header) = parse_header(input).map_err(|e| ParseError::NomError(format!("{:?}", e)))?;

    match PacketId::try_from(header.m_packet_id) {
        Ok(PacketId::CarTelemetry) => {
            let (_, packet) = parse_packet_car_telemetry_data(input).map_err(|e| ParseError::NomError(format!("{:?}", e)))?;
            Ok(TelemetryEvent::CarTelemetry {
                header: packet.m_header,
                data: packet.m_car_telemetry_data,
                player_index: packet.m_header.m_player_car_index,
            })
        }
        Ok(PacketId::CarStatus) => {
            let (_, packet) = parse_packet_car_status_data(input).map_err(|e| ParseError::NomError(format!("{:?}", e)))?;
            Ok(TelemetryEvent::CarStatus {
                header: packet.m_header,
                data: packet.m_car_status_data,
                player_index: packet.m_header.m_player_car_index,
            })
        }
        Ok(PacketId::Session) => {
            let (_, packet) = parse_packet_session_data(input).map_err(|e| ParseError::NomError(format!("{:?}", e)))?;
            Ok(TelemetryEvent::Session { data: packet })
        }
        Ok(PacketId::LapData) => {
            let (_, packet) = parse_packet_lap_data(input).map_err(|e| ParseError::NomError(format!("{:?}", e)))?;
            Ok(TelemetryEvent::LapData {
                header: packet.m_header,
                data: packet.m_lap_data,
                player_index: packet.m_header.m_player_car_index,
            })
        }
        Ok(PacketId::Participants) => {
            let (_, packet) = parse_packet_participants_data(input).map_err(|e| ParseError::NomError(format!("{:?}", e)))?;
            Ok(TelemetryEvent::Participants { data: packet })
        }
        Ok(PacketId::Event) => {
            let (_, packet) = parse_packet_event_data(input).map_err(|e| ParseError::NomError(format!("{:?}", e)))?;
            Ok(TelemetryEvent::Event { data: packet })
        }
        Ok(PacketId::CarDamage) => {
            let (_, packet) = parse_packet_car_damage_data(input).map_err(|e| ParseError::NomError(format!("{:?}", e)))?;
            Ok(TelemetryEvent::CarDamage {
                header: packet.m_header,
                data: packet.m_car_damage_data,
                player_index: packet.m_header.m_player_car_index,
            })
        }
        Ok(PacketId::TyreSets) => {
            let (_, packet) = parse_packet_tyre_sets_data(input).map_err(|e| ParseError::NomError(format!("{:?}", e)))?;
            Ok(TelemetryEvent::TyreSets { data: packet })
        }
        Ok(PacketId::MotionEx) => {
            let (_, packet) = parse_packet_motion_ex_data(input).map_err(|e| ParseError::NomError(format!("{:?}", e)))?;
            Ok(TelemetryEvent::MotionEx {
                header: packet.m_header,
                data: packet.m_motion_ex_data,
                player_index: packet.m_header.m_player_car_index,
            })
        }
        Ok(PacketId::SessionHistory) => {
            let (_, packet) = parse_packet_session_history_data(input).map_err(|e| ParseError::NomError(format!("{:?}", e)))?;
            let player_index = packet.m_header.m_player_car_index;
            Ok(TelemetryEvent::SessionHistory {
                header: packet.m_header,
                data: packet,
                player_index,
            })
        }
        Ok(_packet_id) => {
            // Other packet types - return connection status for now
            Ok(TelemetryEvent::ConnectionStatus {
                connected: true,
                packet_rate: 0.0,
                last_packet_time: 0,
            })
        }
        Err(_) => Err(ParseError::InvalidPacketId(header.m_packet_id)),
    }
}