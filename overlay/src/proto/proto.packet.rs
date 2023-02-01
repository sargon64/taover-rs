/// ---- Commands (DO something!) ---- //
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Command {
    #[prost(oneof = "command::Type", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
    pub r#type: ::core::option::Option<command::Type>,
}
/// Nested message and enum types in `Command`.
pub mod command {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LoadSong {
        #[prost(string, tag = "1")]
        pub level_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub custom_host_url: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlaySong {
        #[prost(message, optional, tag = "1")]
        pub gameplay_parameters: ::core::option::Option<
            super::super::models::GameplayParameters,
        >,
        #[prost(bool, tag = "3")]
        pub floating_scoreboard: bool,
        #[prost(bool, tag = "4")]
        pub stream_sync: bool,
        #[prost(bool, tag = "5")]
        pub disable_fail: bool,
        #[prost(bool, tag = "6")]
        pub disable_pause: bool,
        #[prost(bool, tag = "7")]
        pub disable_scoresaber_submission: bool,
        #[prost(bool, tag = "8")]
        pub show_normal_notes_on_stream: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SendBotMessage {
        #[prost(message, optional, tag = "1")]
        pub channel: ::core::option::Option<super::super::discord::Channel>,
        #[prost(string, tag = "2")]
        pub message: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ShowModal {
        #[prost(string, tag = "1")]
        pub modal_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub message_title: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub message_text: ::prost::alloc::string::String,
        #[prost(bool, tag = "4")]
        pub can_close: bool,
        #[prost(message, optional, tag = "5")]
        pub option_1: ::core::option::Option<super::super::models::ModalOption>,
        #[prost(message, optional, tag = "6")]
        pub option_2: ::core::option::Option<super::super::models::ModalOption>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Is this really a command?
        #[prost(bool, tag = "1")]
        Heartbeat(bool),
        #[prost(bool, tag = "2")]
        ReturnToMenu(bool),
        #[prost(bool, tag = "3")]
        DelayTestFinish(bool),
        #[prost(bool, tag = "4")]
        StreamSyncShowImage(bool),
        #[prost(message, tag = "5")]
        LoadSong(LoadSong),
        #[prost(message, tag = "6")]
        PlaySong(PlaySong),
        #[prost(message, tag = "7")]
        SendBotMessage(SendBotMessage),
        #[prost(message, tag = "8")]
        ShowModal(ShowModal),
    }
}
/// ---- Pushes (SUBMIT something!) ---- //
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Push {
    #[prost(oneof = "push::Data", tags = "1, 2, 3")]
    pub data: ::core::option::Option<push::Data>,
}
/// Nested message and enum types in `Push`.
pub mod push {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LeaderboardScore {
        #[prost(message, optional, tag = "1")]
        pub score: ::core::option::Option<super::super::models::LeaderboardScore>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RealtimeScore {
        #[prost(string, tag = "1")]
        pub user_guid: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub score: i32,
        #[prost(int32, tag = "3")]
        pub score_with_modifiers: i32,
        #[prost(int32, tag = "4")]
        pub max_score: i32,
        #[prost(int32, tag = "5")]
        pub max_score_with_modifiers: i32,
        #[prost(int32, tag = "6")]
        pub combo: i32,
        #[prost(float, tag = "7")]
        pub player_health: f32,
        #[prost(float, tag = "8")]
        pub accuracy: f32,
        #[prost(float, tag = "9")]
        pub song_position: f32,
        #[prost(message, optional, tag = "10")]
        pub score_tracker: ::core::option::Option<super::super::models::ScoreTracker>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SongFinished {
        #[prost(message, optional, tag = "1")]
        pub player: ::core::option::Option<super::super::models::User>,
        #[prost(message, optional, tag = "2")]
        pub beatmap: ::core::option::Option<super::super::models::Beatmap>,
        #[prost(enumeration = "song_finished::CompletionType", tag = "3")]
        pub r#type: i32,
        #[prost(int32, tag = "4")]
        pub score: i32,
    }
    /// Nested message and enum types in `SongFinished`.
    pub mod song_finished {
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
        pub enum CompletionType {
            Passed = 0,
            Failed = 1,
            Quit = 2,
        }
        impl CompletionType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    CompletionType::Passed => "Passed",
                    CompletionType::Failed => "Failed",
                    CompletionType::Quit => "Quit",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "Passed" => Some(Self::Passed),
                    "Failed" => Some(Self::Failed),
                    "Quit" => Some(Self::Quit),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "1")]
        LeaderboardScore(LeaderboardScore),
        #[prost(message, tag = "2")]
        RealtimeScore(RealtimeScore),
        #[prost(message, tag = "3")]
        SongFinished(SongFinished),
    }
}
/// ---- Requests (GET (or do?) something where you need a response!) ---- //
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof = "request::Type", tags = "1, 2, 3")]
    pub r#type: ::core::option::Option<request::Type>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Connect {
        #[prost(message, optional, tag = "1")]
        pub user: ::core::option::Option<super::super::models::User>,
        #[prost(string, tag = "2")]
        pub password: ::prost::alloc::string::String,
        #[prost(int32, tag = "3")]
        pub client_version: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LeaderboardScore {
        #[prost(string, tag = "1")]
        pub event_id: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        pub parameters: ::core::option::Option<super::super::models::GameplayParameters>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PreloadImageForStreamSync {
        #[prost(string, tag = "1")]
        pub file_id: ::prost::alloc::string::String,
        #[prost(bool, tag = "2")]
        pub compressed: bool,
        #[prost(bytes = "vec", tag = "3")]
        pub data: ::prost::alloc::vec::Vec<u8>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag = "1")]
        Connect(Connect),
        #[prost(message, tag = "2")]
        LeaderboardScore(LeaderboardScore),
        #[prost(message, tag = "3")]
        PreloadImageForStreamSync(PreloadImageForStreamSync),
    }
}
/// ---- Responses ---- //
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(enumeration = "response::ResponseType", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub responding_to_packet_id: ::prost::alloc::string::String,
    #[prost(oneof = "response::Details", tags = "3, 4, 5, 6, 7, 8")]
    pub details: ::core::option::Option<response::Details>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Connect {
        #[prost(message, optional, tag = "1")]
        pub state: ::core::option::Option<super::super::models::State>,
        #[prost(string, tag = "2")]
        pub self_guid: ::prost::alloc::string::String,
        #[prost(int32, tag = "3")]
        pub server_version: i32,
        #[prost(string, tag = "4")]
        pub message: ::prost::alloc::string::String,
        #[prost(enumeration = "ConnectFailReason", tag = "5")]
        pub reason: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LeaderboardScores {
        #[prost(message, repeated, tag = "1")]
        pub scores: ::prost::alloc::vec::Vec<super::super::models::LeaderboardScore>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LoadedSong {
        #[prost(message, optional, tag = "1")]
        pub level: ::core::option::Option<super::super::models::PreviewBeatmapLevel>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Modal {
        #[prost(string, tag = "1")]
        pub modal_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ModifyQualifier {
        #[prost(string, tag = "1")]
        pub message: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ImagePreloaded {
        #[prost(string, tag = "1")]
        pub file_id: ::prost::alloc::string::String,
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
    pub enum ResponseType {
        Fail = 0,
        Success = 1,
    }
    impl ResponseType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResponseType::Fail => "Fail",
                ResponseType::Success => "Success",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Fail" => Some(Self::Fail),
                "Success" => Some(Self::Success),
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
    pub enum ConnectFailReason {
        IncorrectVersion = 0,
        IncorrectPassword = 1,
    }
    impl ConnectFailReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConnectFailReason::IncorrectVersion => "IncorrectVersion",
                ConnectFailReason::IncorrectPassword => "IncorrectPassword",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IncorrectVersion" => Some(Self::IncorrectVersion),
                "IncorrectPassword" => Some(Self::IncorrectPassword),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        #[prost(message, tag = "3")]
        Connect(Connect),
        #[prost(message, tag = "4")]
        LeaderboardScores(LeaderboardScores),
        #[prost(message, tag = "5")]
        LoadedSong(LoadedSong),
        #[prost(message, tag = "6")]
        Modal(Modal),
        #[prost(message, tag = "7")]
        ModifyQualifier(ModifyQualifier),
        #[prost(message, tag = "8")]
        ImagePreloaded(ImagePreloaded),
    }
}
/// ---- Backbone ---- //
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Acknowledgement {
    #[prost(string, tag = "1")]
    pub packet_id: ::prost::alloc::string::String,
    #[prost(enumeration = "acknowledgement::AcknowledgementType", tag = "2")]
    pub r#type: i32,
}
/// Nested message and enum types in `Acknowledgement`.
pub mod acknowledgement {
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
    pub enum AcknowledgementType {
        MessageReceived = 0,
    }
    impl AcknowledgementType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AcknowledgementType::MessageReceived => "MessageReceived",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MessageReceived" => Some(Self::MessageReceived),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardingPacket {
    #[prost(string, repeated, tag = "1")]
    pub forward_to: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, boxed, tag = "2")]
    pub packet: ::core::option::Option<::prost::alloc::boxed::Box<Packet>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(
        oneof = "event::ChangedObject",
        tags = "1, 2, 3, 6, 7, 8, 9, 10, 11, 12, 13"
    )]
    pub changed_object: ::core::option::Option<event::ChangedObject>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserAddedEvent {
        #[prost(message, optional, tag = "1")]
        pub user: ::core::option::Option<super::super::models::User>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserUpdatedEvent {
        #[prost(message, optional, tag = "1")]
        pub user: ::core::option::Option<super::super::models::User>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserLeftEvent {
        #[prost(message, optional, tag = "1")]
        pub user: ::core::option::Option<super::super::models::User>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchCreatedEvent {
        #[prost(message, optional, tag = "1")]
        pub r#match: ::core::option::Option<super::super::models::Match>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchUpdatedEvent {
        #[prost(message, optional, tag = "1")]
        pub r#match: ::core::option::Option<super::super::models::Match>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchDeletedEvent {
        #[prost(message, optional, tag = "1")]
        pub r#match: ::core::option::Option<super::super::models::Match>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QualifierCreatedEvent {
        #[prost(message, optional, tag = "1")]
        pub event: ::core::option::Option<super::super::models::QualifierEvent>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QualifierUpdatedEvent {
        #[prost(message, optional, tag = "1")]
        pub event: ::core::option::Option<super::super::models::QualifierEvent>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QualifierDeletedEvent {
        #[prost(message, optional, tag = "1")]
        pub event: ::core::option::Option<super::super::models::QualifierEvent>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HostAddedEvent {
        #[prost(message, optional, tag = "1")]
        pub server: ::core::option::Option<super::super::models::CoreServer>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HostDeletedEvent {
        #[prost(message, optional, tag = "1")]
        pub server: ::core::option::Option<super::super::models::CoreServer>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ChangedObject {
        #[prost(message, tag = "1")]
        UserAddedEvent(UserAddedEvent),
        #[prost(message, tag = "2")]
        UserUpdatedEvent(UserUpdatedEvent),
        #[prost(message, tag = "3")]
        UserLeftEvent(UserLeftEvent),
        #[prost(message, tag = "6")]
        MatchCreatedEvent(MatchCreatedEvent),
        #[prost(message, tag = "7")]
        MatchUpdatedEvent(MatchUpdatedEvent),
        #[prost(message, tag = "8")]
        MatchDeletedEvent(MatchDeletedEvent),
        #[prost(message, tag = "9")]
        QualifierCreatedEvent(QualifierCreatedEvent),
        #[prost(message, tag = "10")]
        QualifierUpdatedEvent(QualifierUpdatedEvent),
        #[prost(message, tag = "11")]
        QualifierDeletedEvent(QualifierDeletedEvent),
        #[prost(message, tag = "12")]
        HostAddedEvent(HostAddedEvent),
        #[prost(message, tag = "13")]
        HostDeletedEvent(HostDeletedEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Packet {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    #[prost(oneof = "packet::Packet", tags = "3, 4, 5, 6, 7, 8, 9")]
    pub packet: ::core::option::Option<packet::Packet>,
}
/// Nested message and enum types in `Packet`.
pub mod packet {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Packet {
        #[prost(message, tag = "3")]
        Acknowledgement(super::Acknowledgement),
        #[prost(message, tag = "4")]
        ForwardingPacket(::prost::alloc::boxed::Box<super::ForwardingPacket>),
        #[prost(message, tag = "5")]
        Command(super::Command),
        #[prost(message, tag = "6")]
        Push(super::Push),
        #[prost(message, tag = "7")]
        Request(super::Request),
        #[prost(message, tag = "8")]
        Response(super::Response),
        #[prost(message, tag = "9")]
        Event(super::Event),
    }
}
