//! F1 25 UDP Telemetry Packet Structures
//! Based on official EA Sports F1 25 UDP Specification
//! Packet Format: 2025, Game Year: 25

use serde::{Deserialize, Serialize};
use std::fmt;

/// Size of the packet header in bytes
pub const PACKET_HEADER_SIZE: usize = 29;

/// Maximum number of cars in a session
pub const MAX_CARS: usize = 22;

/// Wheel indices: 0=RL, 1=RR, 2=FL, 3=FR
pub const WHEEL_RL: usize = 0;
pub const WHEEL_RR: usize = 1;
pub const WHEEL_FL: usize = 2;
pub const WHEEL_FR: usize = 3;

/// All packets start with this header (29 bytes, little endian)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PacketHeader {
    pub m_packet_format: u16,       // 2025
    pub m_game_year: u8,            // 25
    pub m_game_major_version: u8,
    pub m_game_minor_version: u8,
    pub m_packet_version: u8,
    pub m_packet_id: u8,            // 0-15
    pub m_session_uid: u64,
    pub m_session_time: f32,
    pub m_frame_identifier: u32,
    pub m_overall_frame_identifier: u32,
    pub m_player_car_index: u8,
    pub m_secondary_player_car_index: u8,
}

impl PacketHeader {
    pub fn new(packet_id: u8) -> Self {
        Self {
            m_packet_format: 2025,
            m_game_year: 25,
            m_game_major_version: 1,
            m_game_minor_version: 0,
            m_packet_version: 1,
            m_packet_id: packet_id,
            m_session_uid: 0,
            m_session_time: 0.0,
            m_frame_identifier: 0,
            m_overall_frame_identifier: 0,
            m_player_car_index: 255,
            m_secondary_player_car_index: 255,
        }
    }
}

/// Packet IDs (matching F1 25 spec)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PacketId {
    Motion = 0,
    Session = 1,
    LapData = 2,
    Event = 3,
    Participants = 4,
    CarSetups = 5,
    CarTelemetry = 6,
    CarStatus = 7,
    FinalClassification = 8,
    LobbyInfo = 9,
    CarDamage = 10,
    SessionHistory = 11,
    TyreSets = 12,
    MotionEx = 13,
    TimeTrial = 14,
    LapPositions = 15,
}

impl TryFrom<u8> for PacketId {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PacketId::Motion),
            1 => Ok(PacketId::Session),
            2 => Ok(PacketId::LapData),
            3 => Ok(PacketId::Event),
            4 => Ok(PacketId::Participants),
            5 => Ok(PacketId::CarSetups),
            6 => Ok(PacketId::CarTelemetry),
            7 => Ok(PacketId::CarStatus),
            8 => Ok(PacketId::FinalClassification),
            9 => Ok(PacketId::LobbyInfo),
            10 => Ok(PacketId::CarDamage),
            11 => Ok(PacketId::SessionHistory),
            12 => Ok(PacketId::TyreSets),
            13 => Ok(PacketId::MotionEx),
            14 => Ok(PacketId::TimeTrial),
            15 => Ok(PacketId::LapPositions),
            _ => Err(()),
        }
    }
}

/// =========================================================================
/// PACKET 0: Motion Data (1349 bytes)
// =========================================================================
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MotionData {
    pub m_world_position_x: f32,
    pub m_world_position_y: f32,
    pub m_world_position_z: f32,
    pub m_world_velocity_x: f32,
    pub m_world_velocity_y: f32,
    pub m_world_velocity_z: f32,
    pub m_world_forward_dir_x: i16,
    pub m_world_forward_dir_y: i16,
    pub m_world_forward_dir_z: i16,
    pub m_world_right_dir_x: i16,
    pub m_world_right_dir_y: i16,
    pub m_world_right_dir_z: i16,
    pub m_g_force_lateral: f32,
    pub m_g_force_longitudinal: f32,
    pub m_g_force_vertical: f32,
    pub m_yaw: f32,
    pub m_pitch: f32,
    pub m_roll: f32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PacketMotionData {
    pub m_header: PacketHeader,
    pub m_car_motion_data: [MotionData; MAX_CARS],
    // Additional suspension/tyre data (simplified for now)
    #[serde(with = "serde_arrays::F32Array")]
    pub m_suspension_position: [f32; 4 * MAX_CARS],
    #[serde(with = "serde_arrays::F32Array")]
    pub m_suspension_velocity: [f32; 4 * MAX_CARS],
    #[serde(with = "serde_arrays::F32Array")]
    pub m_suspension_acceleration: [f32; 4 * MAX_CARS],
    #[serde(with = "serde_arrays::F32Array")]
    pub m_wheel_speed: [f32; 4 * MAX_CARS],
    #[serde(with = "serde_arrays::F32Array")]
    pub m_wheel_slip: [f32; 4 * MAX_CARS],
    pub m_local_velocity_x: f32,
    pub m_local_velocity_y: f32,
    pub m_local_velocity_z: f32,
    pub m_angular_velocity_x: f32,
    pub m_angular_velocity_y: f32,
    pub m_angular_velocity_z: f32,
    pub m_angular_acceleration_x: f32,
    pub m_angular_acceleration_y: f32,
    pub m_angular_acceleration_z: f32,
    pub m_front_wheels_angle: f32,
}

/// =========================================================================
/// PACKET 1: Session Data (753 bytes)
// =========================================================================
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weather {
    Clear = 0,
    LightCloud = 1,
    Overcast = 2,
    LightRain = 3,
    HeavyRain = 4,
    Storm = 5,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionType {
    Unknown = 0,
    Practice1 = 1,
    Practice2 = 2,
    Practice3 = 3,
    ShortPractice = 4,
    Qualifying1 = 5,
    Qualifying2 = 6,
    Qualifying3 = 7,
    ShortQualifying = 8,
    OneShotQualifying = 9,
    Race = 10,
    Race2 = 11,
    Race3 = 12,
    TimeTrial = 13,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackId {
    Melbourne = 0,
    PaulRicard = 1,
    Shanghai = 2,
    Sakhir = 3,
    Catalonia = 4,
    Monaco = 5,
    Montreal = 6,
    Silverstone = 7,
    Hockenheim = 8,
    Hungaroring = 9,
    Spa = 10,
    Monza = 11,
    Singapore = 12,
    Suzuka = 13,
    AbuDhabi = 14,
    Texas = 15,
    Brazil = 16,
    Austria = 17,
    Sochi = 18,
    Mexico = 19,
    Baku = 20,
    Zandvoort = 21,
    Imola = 22,
    Portimao = 23,
    Jeddah = 24,
    Miami = 25,
    LasVegas = 26,
    Lusail = 27,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Formula {
    F1Modern = 0,
    F1Classic = 1,
    F2 = 2,
    F1Generic = 3,
    Beta = 4,
    F1Esports = 5,
    F2_2023 = 6,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MarshalZone {
    pub m_zone_start: f32,     // Fraction (0..1) of lap
    pub m_zone_flag: i8,       // -1 = invalid/none, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WeatherForecastSample {
    pub m_session_type: u8,
    pub m_time_offset: u8,
    pub m_weather: Weather,
    pub m_track_temperature: i8,
    pub m_track_temperature_change: i8,
    pub m_air_temperature: i8,
    pub m_air_temperature_change: i8,
    pub m_rain_percentage: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketSessionData {
    pub m_header: PacketHeader,
    pub m_weather: Weather,
    pub m_track_temperature: i8,
    pub m_air_temperature: i8,
    pub m_total_laps: u8,
    pub m_track_length: u16,
    pub m_session_type: SessionType,
    pub m_track_id: TrackId,
    pub m_formula: Formula,
    pub m_session_time_left: u16,
    pub m_session_duration: u16,
    pub m_pit_speed_limit: u8,
    pub m_game_paused: u8,
    pub m_is_spectating: u8,
    pub m_spectator_car_index: u8,
    pub m_sli_pro_native_support: u8,
    pub m_num_marshal_zones: u8,
    #[serde(with = "serde_arrays::MarshalZoneArray")]
    pub m_marshal_zones: [MarshalZone; 21],
    pub m_safety_car_status: u8, // 0 = no safety car, 1 = full, 2 = virtual
    pub m_network_game: u8,
    pub m_num_weather_forecast_samples: u8,
    #[serde(with = "serde_arrays::WeatherForecastSampleArray")]
    pub m_weather_forecast_samples: [WeatherForecastSample; 56],
    pub m_forecast_accuracy: u8,
    pub m_ai_difficulty: u8,
    pub m_season_link_identifier: u32,
    pub m_weekend_link_identifier: u32,
    pub m_session_link_identifier: u32,
    pub m_pit_stop_window_ideal_lap: u8,
    pub m_pit_stop_window_latest_lap: u8,
    pub m_pit_stop_rejoin_position: u8,
    pub m_steering_assist: u8,
    pub m_braking_assist: u8,
    pub m_gearbox_assist: u8,
    pub m_pit_assist: u8,
    pub m_pit_release_assist: u8,
    pub m_ers_assist: u8,
    pub m_drs_assist: u8,
    pub m_dynamic_racing_line: u8,
    pub m_dynamic_racing_line_type: u8,
    pub m_game_mode: u8,
    pub m_rule_set: u8,
    pub m_time_of_day: u32,
    pub m_session_length: u8,
}

/// Serde helpers for large arrays
mod serde_arrays {
    use super::{MarshalZone, WeatherForecastSample, Weather, MAX_CARS, LapHistoryData, TyreStintHistoryData, TyreSetData, MotionExData, LapPositionData, CarDamageData, ParticipantData, TyreCompound, VisualTyreCompound, TeamId};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub mod MarshalZoneArray {
        use super::*;
        pub fn serialize<S>(array: &[MarshalZone; 21], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(array.iter())
        }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<[MarshalZone; 21], D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<MarshalZone> = Vec::deserialize(deserializer)?;
            let mut array = [MarshalZone { m_zone_start: 0.0, m_zone_flag: 0 }; 21];
            for (i, item) in vec.into_iter().take(21).enumerate() {
                array[i] = item;
            }
            Ok(array)
        }
    }

    pub mod WeatherForecastSampleArray {
        use super::*;
        pub fn serialize<S>(array: &[WeatherForecastSample; 56], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(array.iter())
        }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<[WeatherForecastSample; 56], D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<WeatherForecastSample> = Vec::deserialize(deserializer)?;
            let mut array = [WeatherForecastSample {
                m_session_type: 0,
                m_time_offset: 0,
                m_weather: Weather::Clear,
                m_track_temperature: 0,
                m_track_temperature_change: 0,
                m_air_temperature: 0,
                m_air_temperature_change: 0,
                m_rain_percentage: 0,
            }; 56];
            for (i, item) in vec.into_iter().take(56).enumerate() {
                array[i] = item;
            }
            Ok(array)
        }
    }

    // Generic helper for large f32 arrays
    pub mod F32Array {
        use serde::{Deserialize, Deserializer, Serializer};

        pub fn serialize<S, const N: usize>(array: &[f32; N], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(array.iter())
        }

        pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[f32; N], D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<f32> = Vec::deserialize(deserializer)?;
            let mut array = [0.0f32; N];
            for (i, item) in vec.into_iter().take(N).enumerate() {
                array[i] = item;
            }
            Ok(array)
        }
    }

    // Generic helper for large i16 arrays
    pub mod I16Array {
        use serde::{Deserialize, Deserializer, Serializer};

        pub fn serialize<S, const N: usize>(array: &[i16; N], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(array.iter())
        }

        pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<[i16; N], D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<i16> = Vec::deserialize(deserializer)?;
            let mut array = [0i16; N];
            for (i, item) in vec.into_iter().take(N).enumerate() {
                array[i] = item;
            }
            Ok(array)
        }
    }

    // Generic helper for arrays of Copy types
    pub mod CopyArray {
        use serde::{Deserialize, Deserializer, Serializer};

        pub fn serialize<S, T: Copy + serde::Serialize, const N: usize>(array: &[T; N], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(array.iter())
        }

        pub fn deserialize<'de, D, T: Copy + Deserialize<'de> + Default, const N: usize>(deserializer: D) -> Result<[T; N], D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<T> = Vec::deserialize(deserializer)?;
            let mut array = [T::default(); N];
            for (i, item) in vec.into_iter().take(N).enumerate() {
                array[i] = item;
            }
            Ok(array)
        }
    }

    // For non-Copy types that need custom default
    pub mod NonCopyArray {
        use serde::{Deserialize, Deserializer, Serializer};

        pub fn serialize<S, T: serde::Serialize, const N: usize>(array: &[T; N], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(array.iter())
        }

        pub fn deserialize<'de, D, T: Deserialize<'de> + Default, const N: usize>(deserializer: D) -> Result<[T; N], D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<T> = Vec::deserialize(deserializer)?;
            let mut array: [T; N] = std::array::from_fn(|_| T::default());
            for (i, item) in vec.into_iter().take(N).enumerate() {
                array[i] = item;
            }
            Ok(array)
        }
    }

    // LapHistoryData array (100 elements)
    pub mod LapHistoryDataArray {
        use super::*;
        pub fn serialize<S>(array: &[LapHistoryData; 100], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(array.iter())
        }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<[LapHistoryData; 100], D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<LapHistoryData> = Vec::deserialize(deserializer)?;
            let mut array = [LapHistoryData {
                m_lap_time_in_ms: 0,
                m_sector1_time_in_ms: 0,
                m_sector2_time_in_ms: 0,
                m_sector3_time_in_ms: 0,
                m_lap_valid_bit_flags: 0,
            }; 100];
            for (i, item) in vec.into_iter().take(100).enumerate() {
                array[i] = item;
            }
            Ok(array)
        }
    }

    // TyreStintHistoryData array (8 elements)
    pub mod TyreStintHistoryDataArray {
        use super::*;
        pub fn serialize<S>(array: &[TyreStintHistoryData; 8], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(array.iter())
        }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<[TyreStintHistoryData; 8], D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<TyreStintHistoryData> = Vec::deserialize(deserializer)?;
            let mut array = [TyreStintHistoryData {
                m_end_lap: 0,
                m_tyre_actual_compound: TyreCompound::C0,
                m_tyre_visual_compound: VisualTyreCompound::Hard,
            }; 8];
            for (i, item) in vec.into_iter().take(8).enumerate() {
                array[i] = item;
            }
            Ok(array)
        }
    }

    // TyreSetData array (20 elements)
    pub mod TyreSetDataArray {
        use super::*;
        pub fn serialize<S>(array: &[TyreSetData; 20], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(array.iter())
        }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<[TyreSetData; 20], D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<TyreSetData> = Vec::deserialize(deserializer)?;
            let mut array = [TyreSetData {
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
            for (i, item) in vec.into_iter().take(20).enumerate() {
                array[i] = item;
            }
            Ok(array)
        }
    }

    // MotionExData array (MAX_CARS = 22 elements)
    pub mod MotionExDataArray {
        use super::*;
        pub fn serialize<S>(array: &[MotionExData; MAX_CARS], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(array.iter())
        }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<[MotionExData; MAX_CARS], D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<MotionExData> = Vec::deserialize(deserializer)?;
            let mut array = [MotionExData {
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
            for (i, item) in vec.into_iter().take(MAX_CARS).enumerate() {
                array[i] = item;
            }
            Ok(array)
        }
    }

    // LapPositionData array (MAX_CARS = 22 elements)
    pub mod LapPositionDataArray {
        use super::*;
        pub fn serialize<S>(array: &[LapPositionData; MAX_CARS], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(array.iter())
        }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<[LapPositionData; MAX_CARS], D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<LapPositionData> = Vec::deserialize(deserializer)?;
            let mut array = [LapPositionData {
                m_lap_distance: 0.0,
                m_sector: 0,
                m_lap_number: 0,
                m_pit_status: 0,
                m_grid_position: 0,
                m_driver_status: 0,
                m_result_status: 0,
            }; MAX_CARS];
            for (i, item) in vec.into_iter().take(MAX_CARS).enumerate() {
                array[i] = item;
            }
            Ok(array)
        }
    }

    // CarDamageData array (MAX_CARS = 22 elements)
    pub mod CarDamageDataArray {
        use super::*;
        pub fn serialize<S>(array: &[CarDamageData; MAX_CARS], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(array.iter())
        }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<[CarDamageData; MAX_CARS], D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<CarDamageData> = Vec::deserialize(deserializer)?;
            let mut array = [CarDamageData {
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
            for (i, item) in vec.into_iter().take(MAX_CARS).enumerate() {
                array[i] = item;
            }
            Ok(array)
        }
    }

    // ParticipantData array (MAX_CARS = 22 elements) - uses NonCopyArray since ParticipantData is not Copy
    pub mod ParticipantDataArray {
        use super::*;
        pub fn serialize<S>(array: &[ParticipantData; MAX_CARS], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(array.iter())
        }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<[ParticipantData; MAX_CARS], D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<ParticipantData> = Vec::deserialize(deserializer)?;
            let mut array: [ParticipantData; MAX_CARS] = std::array::from_fn(|_| ParticipantData {
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
            });
            for (i, item) in vec.into_iter().take(MAX_CARS).enumerate() {
                array[i] = item;
            }
            Ok(array)
        }
    }
}

/// =========================================================================
/// PACKET 2: Lap Data (1285 bytes)
// =========================================================================
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LapData {
    pub m_last_lap_time_in_ms: u32,
    pub m_current_lap_time_in_ms: u32,
    pub m_sector1_time_in_ms: u16,
    pub m_sector2_time_in_ms: u16,
    pub m_sector3_time_in_ms: u16,
    pub m_lap_distance: f32,
    pub m_total_distance: f32,
    pub m_safety_car_delta: f32,
    pub m_car_position: u8,
    pub m_current_lap_num: u8,
    pub m_pit_status: u8,       // 0 = none, 1 = pitting, 2 = in pit area
    pub m_num_pit_stops: u8,
    pub m_sector: u8,           // 0 = sector1, 1 = sector2, 2 = sector3
    pub m_current_lap_invalid: u8,
    pub m_penalties: u8,
    pub m_warnings: u8,
    pub m_num_unserved_drive_through_penalties: u8,
    pub m_num_unserved_stop_go_penalties: u8,
    pub m_grid_position: u8,
    pub m_driver_status: u8,    // 0 = in garage, 1 = flying lap, 2 = in lap, 3 = out lap, 4 = on track
    pub m_result_status: u8,    // 0 = invalid, 1 = inactive, 2 = active, 3 = finished, 4 = dnf, 5 = dsq, 6 = not classified, 7 = retired
    pub m_pit_lane_timer_active: u8,
    pub m_pit_lane_time_lane_in_ms: u16,
    pub m_pit_stop_timer_in_ms: u16,
    pub m_pit_stop_should_serve_pen: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketLapData {
    pub m_header: PacketHeader,
    pub m_lap_data: [LapData; MAX_CARS],
    pub m_time_trial_personal_best_car_idx: u8,
    pub m_time_trial_rival_car_idx: u8,
}

/// =========================================================================
/// PACKET 3: Event Data (variable size)
// =========================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventCode {
    SessionStarted,
    SessionEnded,
    FastestLap,
    Retirement,
    DRSEnabled,
    DRSDisabled,
    TeamMateInPits,
    ChequeredFlag,
    RaceWinner,
    PenaltyIssued,
    SpeedTrapTriggered,
    StartLights,
    LightsOut,
    FlashbackActivated,
    FlashbackDeactivated,
    ButtonStatus,
    RedFlag,
    Overtake,
    SafetyCar,
    Collision,
}

impl EventCode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "SSTA" => Some(EventCode::SessionStarted),
            "SEND" => Some(EventCode::SessionEnded),
            "FTLP" => Some(EventCode::FastestLap),
            "RTMT" => Some(EventCode::Retirement),
            "DRSE" => Some(EventCode::DRSEnabled),
            "DRSD" => Some(EventCode::DRSDisabled),
            "TMPT" => Some(EventCode::TeamMateInPits),
            "CHQF" => Some(EventCode::ChequeredFlag),
            "RCWN" => Some(EventCode::RaceWinner),
            "PENA" => Some(EventCode::PenaltyIssued),
            "SPTP" => Some(EventCode::SpeedTrapTriggered),
            "STLG" => Some(EventCode::StartLights),
            "LGOT" => Some(EventCode::LightsOut),
            "FLBK" => Some(EventCode::FlashbackActivated), // Same code as FlashbackDeactivated, check event data
            "BUTN" => Some(EventCode::ButtonStatus),
            "RDFL" => Some(EventCode::RedFlag),
            "OVTK" => Some(EventCode::Overtake),
            "SCAR" => Some(EventCode::SafetyCar),
            "COLL" => Some(EventCode::Collision),
            _ => None,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EventDataDetails {
    // Union-like - actual content depends on event type
    pub m_data: [u8; 16],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketEventData {
    pub m_header: PacketHeader,
    pub m_event_string_code: [u8; 4],
    pub m_event_details: EventDataDetails,
}

/// =========================================================================
/// PACKET 4: Participants Data (1284 bytes)
// =========================================================================
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamId {
    Mercedes = 0,
    Ferrari = 1,
    RedBull = 2,
    Williams = 3,
    RacingBulls = 4,
    Alpine = 5,
    AstonMartin = 6,
    Haas = 7,
    Mclaren = 8,
    Sauber = 9,
    // F2 teams...
}

// Custom serde for [u8; 48] name field
mod participant_name_serde {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(name: &[u8; 48], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert to string, trimming null bytes
        let s = std::str::from_utf8(name)
            .unwrap_or("")
            .trim_end_matches('\0');
        serializer.serialize_str(s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 48], D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut arr = [0u8; 48];
        let bytes = s.as_bytes();
        let len = bytes.len().min(48);
        arr[..len].copy_from_slice(&bytes[..len]);
        Ok(arr)
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ParticipantData {
    pub m_ai_controlled: u8,
    pub m_driver_id: u8,
    pub m_network_id: u8,
    pub m_team_id: TeamId,
    pub m_my_team: u8,
    pub m_race_number: u8,
    pub m_nationality: u8,
    #[serde(with = "participant_name_serde")]
    pub m_name: [u8; 48],        // 32 chars in F1 25, padded
    pub m_your_telemetry: u8,
    pub m_show_online_names: u8,
    pub m_platform: u8,
    pub m_car_colour_primary: u8,
    pub m_car_colour_secondary: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketParticipantsData {
    pub m_header: PacketHeader,
    pub m_num_active_cars: u8,
    #[serde(with = "serde_arrays::ParticipantDataArray")]
    pub m_participants: [ParticipantData; MAX_CARS],
}

/// =========================================================================
/// PACKET 5: Car Setups Data (1133 bytes)
// =========================================================================
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CarSetupData {
    pub m_front_wing: u8,
    pub m_rear_wing: u8,
    pub m_on_throttle: u8,
    pub m_off_throttle: u8,
    pub m_front_camber: f32,
    pub m_rear_camber: f32,
    pub m_front_toe: f32,
    pub m_rear_toe: f32,
    pub m_front_suspension: u8,
    pub m_rear_suspension: u8,
    pub m_front_anti_roll_bar: u8,
    pub m_rear_anti_roll_bar: u8,
    pub m_front_suspension_height: u8,
    pub m_rear_suspension_height: u8,
    pub m_brake_pressure: u8,
    pub m_brake_bias: u8,
    pub m_rear_left_tyre_pressure: f32,
    pub m_rear_right_tyre_pressure: f32,
    pub m_front_left_tyre_pressure: f32,
    pub m_front_right_tyre_pressure: f32,
    pub m_ballast: u8,
    pub m_fuel_load: f32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketCarSetupData {
    pub m_header: PacketHeader,
    pub m_car_setups: [CarSetupData; MAX_CARS],
}

/// =========================================================================
/// PACKET 6: Car Telemetry Data (1352 bytes) - KEY PACKET
// =========================================================================
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrsState {
    NotAvailable = 0,
    Available = 1,
    Active = 2,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CarTelemetryData {
    pub m_speed: u16,                    // km/h
    pub m_throttle: f32,                 // 0.0 to 1.0
    pub m_brake: f32,                    // 0.0 to 1.0
    pub m_steering: f32,                 // -1.0 to 1.0
    pub m_gear: i8,                      // -1 = R, 0 = N, 1-8
    pub m_engine_rpm: u16,
    pub m_drs: DrsState,
    pub m_rev_lights_percent: u8,        // 0-100
    pub m_rev_lights_bit_value: u16,     // Bit flags for rev lights
    pub m_brakes_temperature: [u16; 4],  // RL, RR, FL, FR (celsius)
    pub m_tyres_surface_temperature: [u8; 4],  // RL, RR, FL, FR (celsius)
    pub m_tyres_inner_temperature: [u8; 4],    // RL, RR, FL, FR (celsius)
    pub m_engine_temperature: u16,       // celsius
    pub m_tyres_pressure: [f32; 4],      // RL, RR, FL, FR (PSI)
    pub m_surface_type: [u8; 4],         // RL, RR, FL, FR
}

#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketCarTelemetryData {
    pub m_header: PacketHeader,
    pub m_car_telemetry_data: [CarTelemetryData; MAX_CARS],
    pub m_mfd_panel_index: u8,
    pub m_mfd_panel_index_secondary_player: u8,
    pub m_suggested_gear: i8,
}

/// =========================================================================
/// PACKET 7: Car Status Data (1239 bytes) - KEY PACKET
// =========================================================================
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TyreCompound {
    C0 = 0,   // Hard
    C1 = 1,   // Medium
    C2 = 2,   // Soft
    C3 = 3,   // Super Soft
    C4 = 4,   // Ultra Soft
    C5 = 5,   // Hypersoft
    C6 = 6,   // New in F1 25
    Inter = 7,
    Wet = 8,
    // F2 compounds...
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisualTyreCompound {
    Soft = 0,
    Medium = 1,
    Hard = 2,
    Intermediate = 3,
    Wet = 4,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErsMode {
    None = 0,
    Medium = 1,
    Hotlap = 2,
    Overtake = 3,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EngineMode {
    Lean = 0,
    Standard = 1,
    Rich = 2,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CarStatusData {
    pub m_traction_control: u8,            // 0 = off, 1 = medium, 2 = full
    pub m_anti_lock_brakes: u8,            // 0 = off, 1 = on
    pub m_fuel_mix: EngineMode,
    pub m_front_brake_bias: u8,            // Front brake bias (percentage * 100)
    pub m_pit_limiter_status: u8,          // 0 = off, 1 = on
    pub m_fuel_in_tank: f32,               // Current fuel mass (kg)
    pub m_fuel_capacity: f32,              // Fuel capacity (kg)
    pub m_fuel_remaining_laps: f32,        // Fuel remaining in terms of laps
    pub m_max_rpm: u16,
    pub m_idle_rpm: u16,
    pub m_max_gears: u8,
    pub m_drs_allowed: u8,                 // 0 = not allowed, 1 = allowed
    pub m_drs_activation_distance: u16,    // 0 = DRS not available, non-zero = distance
    pub m_actual_tyre_compound: TyreCompound,
    pub m_visual_tyre_compound: VisualTyreCompound,
    pub m_tyres_age_laps: u8,
    pub m_vehicle_fia_flags: i8,           // -1 = invalid/none, 0 = none, 1 = green, 2 = blue, 3 = yellow, 4 = red
    pub m_ers_store_energy: f32,           // ERS energy in Joules
    pub m_ers_deploy_mode: ErsMode,
    pub m_ers_harvested_this_lap_mguk: f32,
    pub m_ers_deployed_this_lap_mguk: f32,
    pub m_ers_harvested_this_lap_mguh: f32,
    pub m_ers_deployed_this_lap_mguh: f32,
    pub m_ers_harvested_this_lap_total: f32,
    pub m_ers_deployed_this_lap_total: f32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketCarStatusData {
    pub m_header: PacketHeader,
    pub m_car_status_data: [CarStatusData; MAX_CARS],
}

/// =========================================================================
/// PACKET 8: Final Classification Data (1042 bytes)
// =========================================================================
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FinalClassificationData {
    pub m_position: u8,
    pub m_num_laps: u8,
    pub m_grid_position: u8,
    pub m_points: u8,
    pub m_num_pit_stops: u8,
    pub m_result_status: u8,
    pub m_best_lap_time_in_ms: u32,
    pub m_total_race_time: f64,
    pub m_penalties_time: u8,
    pub m_num_penalties: u8,
    pub m_num_tyre_stints: u8,
    pub m_tyre_stints_actual: [u8; 8],
    pub m_tyre_stints_visual: [u8; 8],
    pub m_tyre_stints_end_laps: [u8; 8],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketFinalClassificationData {
    pub m_header: PacketHeader,
    pub m_num_cars: u8,
    pub m_classification_data: [FinalClassificationData; MAX_CARS],
}

/// =========================================================================
/// PACKET 9: Lobby Info Data (954 bytes)
// =========================================================================
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LobbyInfoData {
    pub m_ai_controlled: u8,
    pub m_team_id: TeamId,
    pub m_nationality: u8,
    pub m_platform: u8,
    #[serde(with = "participant_name_serde")]
    pub m_name: [u8; 48],
    pub m_car_number: u8,
    pub m_ready_status: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketLobbyInfoData {
    pub m_header: PacketHeader,
    pub m_num_players: u8,
    pub m_lobby_players: [LobbyInfoData; MAX_CARS],
}

/// =========================================================================
/// PACKET 10: Car Damage Data (1041 bytes) - KEY PACKET
// =========================================================================
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CarDamageData {
    pub m_tyres_wear: [f32; 4],            // RL, RR, FL, FR (percentage)
    pub m_tyres_damage: [u8; 4],           // RL, RR, FL, FR
    pub m_brakes_damage: [u8; 4],          // RL, RR, FL, FR
    pub m_front_left_wing_damage: u8,
    pub m_front_right_wing_damage: u8,
    pub m_rear_wing_damage: u8,
    pub m_floor_damage: u8,
    pub m_diffuser_damage: u8,
    pub m_sidepod_damage: u8,
    pub m_drs_fault: u8,                   // 0 = OK, 1 = fault
    pub m_gear_box_damage: u8,
    pub m_engine_damage: u8,
    pub m_engine_mgu_h_wear: u8,
    pub m_engine_mgu_k_wear: u8,
    pub m_engine_tc_wear: u8,
    pub m_engine_ce_wear: u8,
    pub m_engine_ice_wear: u8,
    pub m_tyre_blister_rl: u8,             // New in F1 25
    pub m_tyre_blister_rr: u8,
    pub m_tyre_blister_fl: u8,
    pub m_tyre_blister_fr: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketCarDamageData {
    pub m_header: PacketHeader,
    #[serde(with = "serde_arrays::CarDamageDataArray")]
    pub m_car_damage_data: [CarDamageData; MAX_CARS],
}

/// =========================================================================
/// PACKET 11: Session History Data (1460 bytes)
// =========================================================================
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LapHistoryData {
    pub m_lap_time_in_ms: u32,
    pub m_sector1_time_in_ms: u16,
    pub m_sector2_time_in_ms: u16,
    pub m_sector3_time_in_ms: u16,
    pub m_lap_valid_bit_flags: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TyreStintHistoryData {
    pub m_end_lap: u8,
    pub m_tyre_actual_compound: TyreCompound,
    pub m_tyre_visual_compound: VisualTyreCompound,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketSessionHistoryData {
    pub m_header: PacketHeader,
    pub m_car_idx: u8,
    pub m_num_laps: u8,
    pub m_num_tyre_stints: u8,
    pub m_best_lap_time_lap_num: u8,
    pub m_best_sector1_lap_num: u8,
    pub m_best_sector2_lap_num: u8,
    pub m_best_sector3_lap_num: u8,
    #[serde(with = "serde_arrays::LapHistoryDataArray")]
    pub m_lap_history_data: [LapHistoryData; 100],
    #[serde(with = "serde_arrays::TyreStintHistoryDataArray")]
    pub m_tyre_stint_history_data: [TyreStintHistoryData; 8],
}

/// =========================================================================
/// PACKET 12: Tyre Sets Data (231 bytes) - KEY PACKET
// =========================================================================
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TyreSetData {
    pub m_actual_tyre_compound: TyreCompound,
    pub m_visual_tyre_compound: VisualTyreCompound,
    pub m_wear: u8,
    pub m_available: u8,
    pub m_recommended_session: u8,
    pub m_life_span: u8,
    pub m_usable_life: u8,
    pub m_laps_done: u8,
    pub m_is_new: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketTyreSetsData {
    pub m_header: PacketHeader,
    pub m_car_idx: u8,
    #[serde(with = "serde_arrays::TyreSetDataArray")]
    pub m_tyre_set_data: [TyreSetData; 20],
    pub m_fitted_idx: u8,
}

/// =========================================================================
/// PACKET 13: Motion Ex Data (273 bytes) - KEY PACKET
// =========================================================================
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MotionExData {
    pub m_suspension_position: [f32; 4],     // RL, RR, FL, FR
    pub m_suspension_velocity: [f32; 4],
    pub m_suspension_acceleration: [f32; 4],
    pub m_wheel_speed: [f32; 4],
    pub m_wheel_slip: [f32; 4],
    pub m_local_velocity_x: f32,
    pub m_local_velocity_y: f32,
    pub m_local_velocity_z: f32,
    pub m_angular_velocity_x: f32,
    pub m_angular_velocity_y: f32,
    pub m_angular_velocity_z: f32,
    pub m_angular_acceleration_x: f32,
    pub m_angular_acceleration_y: f32,
    pub m_angular_acceleration_z: f32,
    pub m_front_wheels_angle: f32,
    // New in F1 25
    pub m_chassis_pitch: f32,
    pub m_wheel_camber: [f32; 4],
    pub m_wheel_camber_gain: [f32; 4],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketMotionExData {
    pub m_header: PacketHeader,
    #[serde(with = "serde_arrays::MotionExDataArray")]
    pub m_motion_ex_data: [MotionExData; MAX_CARS],
}

/// =========================================================================
/// PACKET 14: Time Trial Data (101 bytes)
// =========================================================================
#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketTimeTrialData {
    pub m_header: PacketHeader,
    pub m_car_idx: u8,
    pub m_lap_data: LapData,
    pub m_personal_best_lap_data: LapData,
    pub m_rival_lap_data: LapData,
}

/// =========================================================================
/// PACKET 15: Lap Positions Data (1131 bytes) - New in F1 25
// =========================================================================
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LapPositionData {
    pub m_lap_distance: f32,
    pub m_sector: u8,
    pub m_lap_number: u8,
    pub m_pit_status: u8,
    pub m_grid_position: u8,
    pub m_driver_status: u8,
    pub m_result_status: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketLapPositionsData {
    pub m_header: PacketHeader,
    #[serde(with = "serde_arrays::LapPositionDataArray")]
    pub m_lap_position_data: [LapPositionData; MAX_CARS],
}

/// =========================================================================
/// Unified Telemetry Event for Frontend
// =========================================================================
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TelemetryEvent {
    CarTelemetry {
        header: PacketHeader,
        data: [CarTelemetryData; MAX_CARS],
        player_index: u8,
    },
    CarStatus {
        header: PacketHeader,
        data: [CarStatusData; MAX_CARS],
        player_index: u8,
    },
    Session {
        data: PacketSessionData,
    },
    LapData {
        header: PacketHeader,
        data: [LapData; MAX_CARS],
        player_index: u8,
    },
    Participants {
        data: PacketParticipantsData,
    },
    CarDamage {
        header: PacketHeader,
        data: [CarDamageData; MAX_CARS],
        player_index: u8,
    },
    TyreSets {
        data: PacketTyreSetsData,
    },
    MotionEx {
        header: PacketHeader,
        data: [MotionExData; MAX_CARS],
        player_index: u8,
    },
    Event {
        data: PacketEventData,
    },
    LapPositions {
        header: PacketHeader,
        data: [LapPositionData; MAX_CARS],
        player_index: u8,
    },
    SessionHistory {
        header: PacketHeader,
        data: PacketSessionHistoryData,
        player_index: u8,
    },
    ConnectionStatus {
        connected: bool,
        packet_rate: f32,
        last_packet_time: u64,
    },
    SourceIpDetected {
        ip: String,
        port: u16,
    },
    GameDetected {
        game_id: String,
        game_name: String,
        source_ip: String,
        source_port: u16,
    },
}

/// Helper: Get player car telemetry from array
pub fn get_player_telemetry(data: &[CarTelemetryData; MAX_CARS], index: u8) -> Option<CarTelemetryData> {
    if index < MAX_CARS as u8 {
        Some(data[index as usize])
    } else {
        None
    }
}

/// Helper: Get player car status from array
pub fn get_player_status(data: &[CarStatusData; MAX_CARS], index: u8) -> Option<CarStatusData> {
    if index < MAX_CARS as u8 {
        Some(data[index as usize])
    } else {
        None
    }
}

impl fmt::Display for TrackId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TrackId::Melbourne => "Melbourne",
            TrackId::PaulRicard => "Paul Ricard",
            TrackId::Shanghai => "Shanghai",
            TrackId::Sakhir => "Sakhir (Bahrain)",
            TrackId::Catalonia => "Barcelona-Catalunya",
            TrackId::Monaco => "Monaco",
            TrackId::Montreal => "Montreal",
            TrackId::Silverstone => "Silverstone",
            TrackId::Hockenheim => "Hockenheim",
            TrackId::Hungaroring => "Hungaroring",
            TrackId::Spa => "Spa-Francorchamps",
            TrackId::Monza => "Monza",
            TrackId::Singapore => "Singapore",
            TrackId::Suzuka => "Suzuka",
            TrackId::AbuDhabi => "Abu Dhabi",
            TrackId::Texas => "COTA (Texas)",
            TrackId::Brazil => "Interlagos (Brazil)",
            TrackId::Austria => "Red Bull Ring (Austria)",
            TrackId::Sochi => "Sochi",
            TrackId::Mexico => "Mexico City",
            TrackId::Baku => "Baku",
            TrackId::Zandvoort => "Zandvoort",
            TrackId::Imola => "Imola",
            TrackId::Portimao => "Portimao",
            TrackId::Jeddah => "Jeddah",
            TrackId::Miami => "Miami",
            TrackId::LasVegas => "Las Vegas",
            TrackId::Lusail => "Lusail",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for SessionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            SessionType::Practice1 => "Practice 1",
            SessionType::Practice2 => "Practice 2",
            SessionType::Practice3 => "Practice 3",
            SessionType::ShortPractice => "Short Practice",
            SessionType::Qualifying1 => "Qualifying 1",
            SessionType::Qualifying2 => "Qualifying 2",
            SessionType::Qualifying3 => "Qualifying 3",
            SessionType::ShortQualifying => "Short Qualifying",
            SessionType::OneShotQualifying => "One-Shot Qualifying",
            SessionType::Race => "Race",
            SessionType::Race2 => "Race 2",
            SessionType::Race3 => "Race 3",
            SessionType::TimeTrial => "Time Trial",
            _ => "Unknown",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for TyreCompound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TyreCompound::C0 => "Hard (C0)",
            TyreCompound::C1 => "Medium (C1)",
            TyreCompound::C2 => "Soft (C2)",
            TyreCompound::C3 => "Super Soft (C3)",
            TyreCompound::C4 => "Ultra Soft (C4)",
            TyreCompound::C5 => "Hypersoft (C5)",
            TyreCompound::C6 => "C6 (New)",
            TyreCompound::Inter => "Intermediate",
            TyreCompound::Wet => "Wet",
        };
        write!(f, "{}", name)
    }
}

impl TyreCompound {
    pub fn color(&self) -> &'static str {
        match self {
            TyreCompound::C0 => "#ffffff",     // Hard - White
            TyreCompound::C1 => "#fff200",     // Medium - Yellow
            TyreCompound::C2 => "#ff3333",     // Soft - Red
            TyreCompound::C3 => "#ff00ff",     // Super Soft - Magenta
            TyreCompound::C4 => "#ff6600",     // Ultra Soft - Orange
            TyreCompound::C5 => "#ff0099",     // Hypersoft - Pink
            TyreCompound::C6 => "#cc00ff",     // C6 - Purple
            TyreCompound::Inter => "#00cc66",  // Intermediate - Green
            TyreCompound::Wet => "#0099ff",    // Wet - Blue
        }
    }
}

impl VisualTyreCompound {
    pub fn color(&self) -> &'static str {
        match self {
            VisualTyreCompound::Soft => "#ff3333",
            VisualTyreCompound::Medium => "#fff200",
            VisualTyreCompound::Hard => "#ffffff",
            VisualTyreCompound::Intermediate => "#00cc66",
            VisualTyreCompound::Wet => "#0099ff",
        }
    }
}