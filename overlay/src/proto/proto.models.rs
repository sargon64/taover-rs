#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Characteristic {
    #[prost(string, tag = "1")]
    pub serialized_name: ::prost::alloc::string::String,
    #[prost(int32, repeated, tag = "2")]
    pub difficulties: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Beatmap {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub level_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub characteristic: ::core::option::Option<Characteristic>,
    #[prost(int32, tag = "4")]
    pub difficulty: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewBeatmapLevel {
    #[prost(string, tag = "1")]
    pub level_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub characteristics: ::prost::alloc::vec::Vec<Characteristic>,
    #[prost(bool, tag = "4")]
    pub loaded: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameplayModifiers {
    #[prost(enumeration = "gameplay_modifiers::GameOptions", tag = "1")]
    pub options: i32,
}
/// Nested message and enum types in `GameplayModifiers`.
pub mod gameplay_modifiers {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum GameOptions {
        None = 0,
        /// Negative modifiers
        NoFail = 1,
        NoBombs = 2,
        NoArrows = 4,
        NoObstacles = 8,
        SlowSong = 16,
        /// Positive Modifiers
        InstaFail = 32,
        FailOnClash = 64,
        BatteryEnergy = 128,
        FastNotes = 256,
        FastSong = 512,
        DisappearingArrows = 1024,
        GhostNotes = 2048,
        /// 1.12.2 Additions
        DemoNoFail = 4096,
        DemoNoObstacles = 8192,
        StrictAngles = 16384,
        /// 1.13.4 Additions
        ProMode = 32768,
        ZenMode = 65536,
        SmallCubes = 131072,
        SuperFastSong = 262144,
    }
    impl GameOptions {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GameOptions::None => "None",
                GameOptions::NoFail => "NoFail",
                GameOptions::NoBombs => "NoBombs",
                GameOptions::NoArrows => "NoArrows",
                GameOptions::NoObstacles => "NoObstacles",
                GameOptions::SlowSong => "SlowSong",
                GameOptions::InstaFail => "InstaFail",
                GameOptions::FailOnClash => "FailOnClash",
                GameOptions::BatteryEnergy => "BatteryEnergy",
                GameOptions::FastNotes => "FastNotes",
                GameOptions::FastSong => "FastSong",
                GameOptions::DisappearingArrows => "DisappearingArrows",
                GameOptions::GhostNotes => "GhostNotes",
                GameOptions::DemoNoFail => "DemoNoFail",
                GameOptions::DemoNoObstacles => "DemoNoObstacles",
                GameOptions::StrictAngles => "StrictAngles",
                GameOptions::ProMode => "ProMode",
                GameOptions::ZenMode => "ZenMode",
                GameOptions::SmallCubes => "SmallCubes",
                GameOptions::SuperFastSong => "SuperFastSong",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "None" => Some(Self::None),
                "NoFail" => Some(Self::NoFail),
                "NoBombs" => Some(Self::NoBombs),
                "NoArrows" => Some(Self::NoArrows),
                "NoObstacles" => Some(Self::NoObstacles),
                "SlowSong" => Some(Self::SlowSong),
                "InstaFail" => Some(Self::InstaFail),
                "FailOnClash" => Some(Self::FailOnClash),
                "BatteryEnergy" => Some(Self::BatteryEnergy),
                "FastNotes" => Some(Self::FastNotes),
                "FastSong" => Some(Self::FastSong),
                "DisappearingArrows" => Some(Self::DisappearingArrows),
                "GhostNotes" => Some(Self::GhostNotes),
                "DemoNoFail" => Some(Self::DemoNoFail),
                "DemoNoObstacles" => Some(Self::DemoNoObstacles),
                "StrictAngles" => Some(Self::StrictAngles),
                "ProMode" => Some(Self::ProMode),
                "ZenMode" => Some(Self::ZenMode),
                "SmallCubes" => Some(Self::SmallCubes),
                "SuperFastSong" => Some(Self::SuperFastSong),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerSpecificSettings {
    #[prost(float, tag = "1")]
    pub player_height: f32,
    #[prost(float, tag = "2")]
    pub sfx_volume: f32,
    #[prost(float, tag = "3")]
    pub saber_trail_intensity: f32,
    #[prost(float, tag = "4")]
    pub note_jump_start_beat_offset: f32,
    #[prost(float, tag = "5")]
    pub note_jump_fixed_duration: f32,
    #[prost(enumeration = "player_specific_settings::PlayerOptions", tag = "6")]
    pub options: i32,
    #[prost(
        enumeration = "player_specific_settings::NoteJumpDurationTypeSettings",
        tag = "7"
    )]
    pub note_jump_duration_type_settings: i32,
}
/// Nested message and enum types in `PlayerSpecificSettings`.
pub mod player_specific_settings {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PlayerOptions {
        None = 0,
        LeftHanded = 1,
        StaticLights = 2,
        NoHud = 4,
        AdvancedHud = 8,
        ReduceDebris = 16,
        AutoPlayerHeight = 32,
        NoFailEffects = 64,
        AutoRestart = 128,
        HideNoteSpawnEffect = 256,
        AdaptiveSfx = 512,
    }
    impl PlayerOptions {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PlayerOptions::None => "None",
                PlayerOptions::LeftHanded => "LeftHanded",
                PlayerOptions::StaticLights => "StaticLights",
                PlayerOptions::NoHud => "NoHud",
                PlayerOptions::AdvancedHud => "AdvancedHud",
                PlayerOptions::ReduceDebris => "ReduceDebris",
                PlayerOptions::AutoPlayerHeight => "AutoPlayerHeight",
                PlayerOptions::NoFailEffects => "NoFailEffects",
                PlayerOptions::AutoRestart => "AutoRestart",
                PlayerOptions::HideNoteSpawnEffect => "HideNoteSpawnEffect",
                PlayerOptions::AdaptiveSfx => "AdaptiveSfx",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "None" => Some(Self::None),
                "LeftHanded" => Some(Self::LeftHanded),
                "StaticLights" => Some(Self::StaticLights),
                "NoHud" => Some(Self::NoHud),
                "AdvancedHud" => Some(Self::AdvancedHud),
                "ReduceDebris" => Some(Self::ReduceDebris),
                "AutoPlayerHeight" => Some(Self::AutoPlayerHeight),
                "NoFailEffects" => Some(Self::NoFailEffects),
                "AutoRestart" => Some(Self::AutoRestart),
                "HideNoteSpawnEffect" => Some(Self::HideNoteSpawnEffect),
                "AdaptiveSfx" => Some(Self::AdaptiveSfx),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum NoteJumpDurationTypeSettings {
        Dynamic = 0,
        Static = 1,
    }
    impl NoteJumpDurationTypeSettings {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NoteJumpDurationTypeSettings::Dynamic => "Dynamic",
                NoteJumpDurationTypeSettings::Static => "Static",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Dynamic" => Some(Self::Dynamic),
                "Static" => Some(Self::Static),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameplayParameters {
    #[prost(message, optional, tag = "1")]
    pub beatmap: ::core::option::Option<Beatmap>,
    #[prost(message, optional, tag = "2")]
    pub player_settings: ::core::option::Option<PlayerSpecificSettings>,
    #[prost(message, optional, tag = "3")]
    pub gameplay_modifiers: ::core::option::Option<GameplayModifiers>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Team {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerSettings {
    #[prost(string, tag = "1")]
    pub server_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub enable_teams: bool,
    #[prost(message, repeated, tag = "4")]
    pub teams: ::prost::alloc::vec::Vec<Team>,
    #[prost(int32, tag = "5")]
    pub score_update_frequency: i32,
    #[prost(string, repeated, tag = "6")]
    pub banned_mods: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SongList {
    #[prost(message, repeated, tag = "1")]
    pub levels: ::prost::alloc::vec::Vec<PreviewBeatmapLevel>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag = "1")]
    pub guid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(enumeration = "user::ClientTypes", tag = "4")]
    pub client_type: i32,
    #[prost(message, optional, tag = "5")]
    pub team: ::core::option::Option<Team>,
    #[prost(enumeration = "user::PlayStates", tag = "6")]
    pub play_state: i32,
    #[prost(enumeration = "user::DownloadStates", tag = "7")]
    pub download_state: i32,
    #[prost(string, repeated, tag = "8")]
    pub mod_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "9")]
    pub stream_screen_coordinates: ::core::option::Option<user::Point>,
    #[prost(int64, tag = "10")]
    pub stream_delay_ms: i64,
    #[prost(int64, tag = "11")]
    pub stream_sync_start_ms: i64,
}
/// Nested message and enum types in `User`.
pub mod user {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Point {
        #[prost(int32, tag = "1")]
        pub x: i32,
        #[prost(int32, tag = "2")]
        pub y: i32,
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PlayStates {
        Waiting = 0,
        InGame = 1,
    }
    impl PlayStates {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PlayStates::Waiting => "Waiting",
                PlayStates::InGame => "InGame",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Waiting" => Some(Self::Waiting),
                "InGame" => Some(Self::InGame),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DownloadStates {
        None = 0,
        Downloading = 1,
        Downloaded = 2,
        DownloadError = 3,
    }
    impl DownloadStates {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DownloadStates::None => "None",
                DownloadStates::Downloading => "Downloading",
                DownloadStates::Downloaded => "Downloaded",
                DownloadStates::DownloadError => "DownloadError",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "None" => Some(Self::None),
                "Downloading" => Some(Self::Downloading),
                "Downloaded" => Some(Self::Downloaded),
                "DownloadError" => Some(Self::DownloadError),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ClientTypes {
        Player = 0,
        Coordinator = 1,
        TemporaryConnection = 2,
        WebsocketConnection = 3,
    }
    impl ClientTypes {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ClientTypes::Player => "Player",
                ClientTypes::Coordinator => "Coordinator",
                ClientTypes::TemporaryConnection => "TemporaryConnection",
                ClientTypes::WebsocketConnection => "WebsocketConnection",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Player" => Some(Self::Player),
                "Coordinator" => Some(Self::Coordinator),
                "TemporaryConnection" => Some(Self::TemporaryConnection),
                "WebsocketConnection" => Some(Self::WebsocketConnection),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Match {
    #[prost(string, tag = "1")]
    pub guid: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub associated_users: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub leader: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub selected_level: ::core::option::Option<PreviewBeatmapLevel>,
    #[prost(message, optional, tag = "6")]
    pub selected_characteristic: ::core::option::Option<Characteristic>,
    #[prost(int32, tag = "7")]
    pub selected_difficulty: i32,
    #[prost(string, tag = "8")]
    pub start_time: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QualifierEvent {
    #[prost(string, tag = "1")]
    pub guid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub guild: ::core::option::Option<super::discord::Guild>,
    #[prost(message, optional, tag = "4")]
    pub info_channel: ::core::option::Option<super::discord::Channel>,
    #[prost(message, repeated, tag = "5")]
    pub qualifier_maps: ::prost::alloc::vec::Vec<GameplayParameters>,
    #[prost(bool, tag = "6")]
    pub send_scores_to_info_channel: bool,
    #[prost(int32, tag = "7")]
    pub flags: i32,
}
/// Nested message and enum types in `QualifierEvent`.
pub mod qualifier_event {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum EventSettings {
        None = 0,
        HideScoresFromPlayers = 1,
        DisableScoresaberSubmission = 2,
        EnableLeaderboardMessage = 4,
    }
    impl EventSettings {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventSettings::None => "None",
                EventSettings::HideScoresFromPlayers => "HideScoresFromPlayers",
                EventSettings::DisableScoresaberSubmission => {
                    "DisableScoresaberSubmission"
                }
                EventSettings::EnableLeaderboardMessage => "EnableLeaderboardMessage",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "None" => Some(Self::None),
                "HideScoresFromPlayers" => Some(Self::HideScoresFromPlayers),
                "DisableScoresaberSubmission" => Some(Self::DisableScoresaberSubmission),
                "EnableLeaderboardMessage" => Some(Self::EnableLeaderboardMessage),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoreServer {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub port: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct State {
    #[prost(message, optional, tag = "1")]
    pub server_settings: ::core::option::Option<ServerSettings>,
    #[prost(message, repeated, tag = "2")]
    pub users: ::prost::alloc::vec::Vec<User>,
    #[prost(message, repeated, tag = "4")]
    pub matches: ::prost::alloc::vec::Vec<Match>,
    #[prost(message, repeated, tag = "5")]
    pub events: ::prost::alloc::vec::Vec<QualifierEvent>,
    #[prost(message, repeated, tag = "6")]
    pub known_hosts: ::prost::alloc::vec::Vec<CoreServer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderboardScore {
    #[prost(string, tag = "1")]
    pub event_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub parameters: ::core::option::Option<GameplayParameters>,
    #[prost(string, tag = "3")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub username: ::prost::alloc::string::String,
    #[prost(int32, tag = "5")]
    pub score: i32,
    #[prost(bool, tag = "6")]
    pub full_combo: bool,
    #[prost(string, tag = "7")]
    pub color: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModalOption {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScoreTrackerHand {
    #[prost(int32, tag = "1")]
    pub hit: i32,
    #[prost(int32, tag = "2")]
    pub miss: i32,
    #[prost(int32, tag = "3")]
    pub bad_cut: i32,
    #[prost(float, repeated, tag = "4")]
    pub avg_cut: ::prost::alloc::vec::Vec<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScoreTracker {
    #[prost(int32, tag = "1")]
    pub notes_missed: i32,
    #[prost(int32, tag = "2")]
    pub bad_cuts: i32,
    #[prost(int32, tag = "3")]
    pub bomb_hits: i32,
    #[prost(int32, tag = "4")]
    pub wall_hits: i32,
    #[prost(int32, tag = "5")]
    pub max_combo: i32,
    #[prost(message, optional, tag = "6")]
    pub left_hand: ::core::option::Option<ScoreTrackerHand>,
    #[prost(message, optional, tag = "7")]
    pub right_hand: ::core::option::Option<ScoreTrackerHand>,
}
