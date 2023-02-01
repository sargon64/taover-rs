// Automatically generated rust module for 'models.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct Characteristic<'a> {
    pub serialized_name: Cow<'a, str>,
    pub difficulties: Vec<i32>,
}

impl<'a> MessageRead<'a> for Characteristic<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.serialized_name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.difficulties = r.read_packed(bytes, |r, bytes| Ok(r.read_int32(bytes)?))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Characteristic<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.serialized_name == "" { 0 } else { 1 + sizeof_len((&self.serialized_name).len()) }
        + if self.difficulties.is_empty() { 0 } else { 1 + sizeof_len(self.difficulties.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.serialized_name != "" { w.write_with_tag(10, |w| w.write_string(&**&self.serialized_name))?; }
        w.write_packed_with_tag(18, &self.difficulties, |w, m| w.write_int32(*m), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct Beatmap<'a> {
    pub name: Cow<'a, str>,
    pub level_id: Cow<'a, str>,
    pub characteristic: Option<proto::models::Characteristic<'a>>,
    pub difficulty: i32,
}

impl<'a> MessageRead<'a> for Beatmap<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.level_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.characteristic = Some(r.read_message::<proto::models::Characteristic>(bytes)?),
                Ok(32) => msg.difficulty = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Beatmap<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.level_id == "" { 0 } else { 1 + sizeof_len((&self.level_id).len()) }
        + self.characteristic.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.difficulty == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.difficulty) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != "" { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        if self.level_id != "" { w.write_with_tag(18, |w| w.write_string(&**&self.level_id))?; }
        if let Some(ref s) = self.characteristic { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.difficulty != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.difficulty))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct PreviewBeatmapLevel<'a> {
    pub level_id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub characteristics: Vec<proto::models::Characteristic<'a>>,
    pub loaded: bool,
}

impl<'a> MessageRead<'a> for PreviewBeatmapLevel<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.level_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.characteristics.push(r.read_message::<proto::models::Characteristic>(bytes)?),
                Ok(32) => msg.loaded = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PreviewBeatmapLevel<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.level_id == "" { 0 } else { 1 + sizeof_len((&self.level_id).len()) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + self.characteristics.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.loaded == false { 0 } else { 1 + sizeof_varint(*(&self.loaded) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.level_id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.level_id))?; }
        if self.name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        for s in &self.characteristics { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.loaded != false { w.write_with_tag(32, |w| w.write_bool(*&self.loaded))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameplayModifiers {
    pub options: proto::models::mod_GameplayModifiers::GameOptions,
}

impl<'a> MessageRead<'a> for GameplayModifiers {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.options = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GameplayModifiers {
    fn get_size(&self) -> usize {
        0
        + if self.options == proto::models::mod_GameplayModifiers::GameOptions::None_pb { 0 } else { 1 + sizeof_varint(*(&self.options) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.options != proto::models::mod_GameplayModifiers::GameOptions::None_pb { w.write_with_tag(8, |w| w.write_enum(*&self.options as i32))?; }
        Ok(())
    }
}

pub mod mod_GameplayModifiers {


#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum GameOptions {
    None_pb = 0,
    NoFail = 1,
    NoBombs = 2,
    NoArrows = 4,
    NoObstacles = 8,
    SlowSong = 16,
    InstaFail = 32,
    FailOnClash = 64,
    BatteryEnergy = 128,
    FastNotes = 256,
    FastSong = 512,
    DisappearingArrows = 1024,
    GhostNotes = 2048,
    DemoNoFail = 4096,
    DemoNoObstacles = 8192,
    StrictAngles = 16384,
    ProMode = 32768,
    ZenMode = 65536,
    SmallCubes = 131072,
    SuperFastSong = 262144,
}

impl Default for GameOptions {
    fn default() -> Self {
        GameOptions::None_pb
    }
}

impl From<i32> for GameOptions {
    fn from(i: i32) -> Self {
        match i {
            0 => GameOptions::None_pb,
            1 => GameOptions::NoFail,
            2 => GameOptions::NoBombs,
            4 => GameOptions::NoArrows,
            8 => GameOptions::NoObstacles,
            16 => GameOptions::SlowSong,
            32 => GameOptions::InstaFail,
            64 => GameOptions::FailOnClash,
            128 => GameOptions::BatteryEnergy,
            256 => GameOptions::FastNotes,
            512 => GameOptions::FastSong,
            1024 => GameOptions::DisappearingArrows,
            2048 => GameOptions::GhostNotes,
            4096 => GameOptions::DemoNoFail,
            8192 => GameOptions::DemoNoObstacles,
            16384 => GameOptions::StrictAngles,
            32768 => GameOptions::ProMode,
            65536 => GameOptions::ZenMode,
            131072 => GameOptions::SmallCubes,
            262144 => GameOptions::SuperFastSong,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for GameOptions {
    fn from(s: &'a str) -> Self {
        match s {
            "None_pb" => GameOptions::None_pb,
            "NoFail" => GameOptions::NoFail,
            "NoBombs" => GameOptions::NoBombs,
            "NoArrows" => GameOptions::NoArrows,
            "NoObstacles" => GameOptions::NoObstacles,
            "SlowSong" => GameOptions::SlowSong,
            "InstaFail" => GameOptions::InstaFail,
            "FailOnClash" => GameOptions::FailOnClash,
            "BatteryEnergy" => GameOptions::BatteryEnergy,
            "FastNotes" => GameOptions::FastNotes,
            "FastSong" => GameOptions::FastSong,
            "DisappearingArrows" => GameOptions::DisappearingArrows,
            "GhostNotes" => GameOptions::GhostNotes,
            "DemoNoFail" => GameOptions::DemoNoFail,
            "DemoNoObstacles" => GameOptions::DemoNoObstacles,
            "StrictAngles" => GameOptions::StrictAngles,
            "ProMode" => GameOptions::ProMode,
            "ZenMode" => GameOptions::ZenMode,
            "SmallCubes" => GameOptions::SmallCubes,
            "SuperFastSong" => GameOptions::SuperFastSong,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct PlayerSpecificSettings {
    pub player_height: f32,
    pub sfx_volume: f32,
    pub saber_trail_intensity: f32,
    pub note_jump_start_beat_offset: f32,
    pub note_jump_fixed_duration: f32,
    pub options: proto::models::mod_PlayerSpecificSettings::PlayerOptions,
    pub note_jump_duration_type_settings: proto::models::mod_PlayerSpecificSettings::NoteJumpDurationTypeSettings,
}

impl<'a> MessageRead<'a> for PlayerSpecificSettings {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.player_height = r.read_float(bytes)?,
                Ok(21) => msg.sfx_volume = r.read_float(bytes)?,
                Ok(29) => msg.saber_trail_intensity = r.read_float(bytes)?,
                Ok(37) => msg.note_jump_start_beat_offset = r.read_float(bytes)?,
                Ok(45) => msg.note_jump_fixed_duration = r.read_float(bytes)?,
                Ok(48) => msg.options = r.read_enum(bytes)?,
                Ok(56) => msg.note_jump_duration_type_settings = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PlayerSpecificSettings {
    fn get_size(&self) -> usize {
        0
        + if self.player_height == 0f32 { 0 } else { 1 + 4 }
        + if self.sfx_volume == 0f32 { 0 } else { 1 + 4 }
        + if self.saber_trail_intensity == 0f32 { 0 } else { 1 + 4 }
        + if self.note_jump_start_beat_offset == 0f32 { 0 } else { 1 + 4 }
        + if self.note_jump_fixed_duration == 0f32 { 0 } else { 1 + 4 }
        + if self.options == proto::models::mod_PlayerSpecificSettings::PlayerOptions::None_pb { 0 } else { 1 + sizeof_varint(*(&self.options) as u64) }
        + if self.note_jump_duration_type_settings == proto::models::mod_PlayerSpecificSettings::NoteJumpDurationTypeSettings::Dynamic { 0 } else { 1 + sizeof_varint(*(&self.note_jump_duration_type_settings) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.player_height != 0f32 { w.write_with_tag(13, |w| w.write_float(*&self.player_height))?; }
        if self.sfx_volume != 0f32 { w.write_with_tag(21, |w| w.write_float(*&self.sfx_volume))?; }
        if self.saber_trail_intensity != 0f32 { w.write_with_tag(29, |w| w.write_float(*&self.saber_trail_intensity))?; }
        if self.note_jump_start_beat_offset != 0f32 { w.write_with_tag(37, |w| w.write_float(*&self.note_jump_start_beat_offset))?; }
        if self.note_jump_fixed_duration != 0f32 { w.write_with_tag(45, |w| w.write_float(*&self.note_jump_fixed_duration))?; }
        if self.options != proto::models::mod_PlayerSpecificSettings::PlayerOptions::None_pb { w.write_with_tag(48, |w| w.write_enum(*&self.options as i32))?; }
        if self.note_jump_duration_type_settings != proto::models::mod_PlayerSpecificSettings::NoteJumpDurationTypeSettings::Dynamic { w.write_with_tag(56, |w| w.write_enum(*&self.note_jump_duration_type_settings as i32))?; }
        Ok(())
    }
}

pub mod mod_PlayerSpecificSettings {


#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]pub enum PlayerOptions {
    None_pb = 0,
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

impl Default for PlayerOptions {
    fn default() -> Self {
        PlayerOptions::None_pb
    }
}

impl From<i32> for PlayerOptions {
    fn from(i: i32) -> Self {
        match i {
            0 => PlayerOptions::None_pb,
            1 => PlayerOptions::LeftHanded,
            2 => PlayerOptions::StaticLights,
            4 => PlayerOptions::NoHud,
            8 => PlayerOptions::AdvancedHud,
            16 => PlayerOptions::ReduceDebris,
            32 => PlayerOptions::AutoPlayerHeight,
            64 => PlayerOptions::NoFailEffects,
            128 => PlayerOptions::AutoRestart,
            256 => PlayerOptions::HideNoteSpawnEffect,
            512 => PlayerOptions::AdaptiveSfx,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for PlayerOptions {
    fn from(s: &'a str) -> Self {
        match s {
            "None_pb" => PlayerOptions::None_pb,
            "LeftHanded" => PlayerOptions::LeftHanded,
            "StaticLights" => PlayerOptions::StaticLights,
            "NoHud" => PlayerOptions::NoHud,
            "AdvancedHud" => PlayerOptions::AdvancedHud,
            "ReduceDebris" => PlayerOptions::ReduceDebris,
            "AutoPlayerHeight" => PlayerOptions::AutoPlayerHeight,
            "NoFailEffects" => PlayerOptions::NoFailEffects,
            "AutoRestart" => PlayerOptions::AutoRestart,
            "HideNoteSpawnEffect" => PlayerOptions::HideNoteSpawnEffect,
            "AdaptiveSfx" => PlayerOptions::AdaptiveSfx,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]pub enum NoteJumpDurationTypeSettings {
    Dynamic = 0,
    Static = 1,
}

impl Default for NoteJumpDurationTypeSettings {
    fn default() -> Self {
        NoteJumpDurationTypeSettings::Dynamic
    }
}

impl From<i32> for NoteJumpDurationTypeSettings {
    fn from(i: i32) -> Self {
        match i {
            0 => NoteJumpDurationTypeSettings::Dynamic,
            1 => NoteJumpDurationTypeSettings::Static,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for NoteJumpDurationTypeSettings {
    fn from(s: &'a str) -> Self {
        match s {
            "Dynamic" => NoteJumpDurationTypeSettings::Dynamic,
            "Static" => NoteJumpDurationTypeSettings::Static,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct GameplayParameters<'a> {
    pub beatmap: Option<proto::models::Beatmap<'a>>,
    pub player_settings: Option<proto::models::PlayerSpecificSettings>,
    pub gameplay_modifiers: Option<proto::models::GameplayModifiers>,
}

impl<'a> MessageRead<'a> for GameplayParameters<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.beatmap = Some(r.read_message::<proto::models::Beatmap>(bytes)?),
                Ok(18) => msg.player_settings = Some(r.read_message::<proto::models::PlayerSpecificSettings>(bytes)?),
                Ok(26) => msg.gameplay_modifiers = Some(r.read_message::<proto::models::GameplayModifiers>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GameplayParameters<'a> {
    fn get_size(&self) -> usize {
        0
        + self.beatmap.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.player_settings.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.gameplay_modifiers.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.beatmap { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.player_settings { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.gameplay_modifiers { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct Team<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Team<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Team<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct ServerSettings<'a> {
    pub server_name: Cow<'a, str>,
    pub password: Cow<'a, str>,
    pub enable_teams: bool,
    pub teams: Vec<proto::models::Team<'a>>,
    pub score_update_frequency: i32,
    pub banned_mods: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ServerSettings<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.server_name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.password = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.enable_teams = r.read_bool(bytes)?,
                Ok(34) => msg.teams.push(r.read_message::<proto::models::Team>(bytes)?),
                Ok(40) => msg.score_update_frequency = r.read_int32(bytes)?,
                Ok(50) => msg.banned_mods.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ServerSettings<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.server_name == "" { 0 } else { 1 + sizeof_len((&self.server_name).len()) }
        + if self.password == "" { 0 } else { 1 + sizeof_len((&self.password).len()) }
        + if self.enable_teams == false { 0 } else { 1 + sizeof_varint(*(&self.enable_teams) as u64) }
        + self.teams.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.score_update_frequency == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.score_update_frequency) as u64) }
        + self.banned_mods.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.server_name != "" { w.write_with_tag(10, |w| w.write_string(&**&self.server_name))?; }
        if self.password != "" { w.write_with_tag(18, |w| w.write_string(&**&self.password))?; }
        if self.enable_teams != false { w.write_with_tag(24, |w| w.write_bool(*&self.enable_teams))?; }
        for s in &self.teams { w.write_with_tag(34, |w| w.write_message(s))?; }
        if self.score_update_frequency != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.score_update_frequency))?; }
        for s in &self.banned_mods { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct SongList<'a> {
    pub levels: Vec<proto::models::PreviewBeatmapLevel<'a>>,
}

impl<'a> MessageRead<'a> for SongList<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.levels.push(r.read_message::<proto::models::PreviewBeatmapLevel>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SongList<'a> {
    fn get_size(&self) -> usize {
        0
        + self.levels.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.levels { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct User<'a> {
    pub guid: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub user_id: Cow<'a, str>,
    pub client_type: proto::models::mod_User::ClientTypes,
    pub team: Option<proto::models::Team<'a>>,
    pub play_state: proto::models::mod_User::PlayStates,
    pub download_state: proto::models::mod_User::DownloadStates,
    pub mod_list: Vec<Cow<'a, str>>,
    pub stream_screen_coordinates: Option<proto::models::mod_User::Point>,
    pub stream_delay_ms: i64,
    pub stream_sync_start_ms: i64,
}

impl<'a> MessageRead<'a> for User<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.guid = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.user_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.client_type = r.read_enum(bytes)?,
                Ok(42) => msg.team = Some(r.read_message::<proto::models::Team>(bytes)?),
                Ok(48) => msg.play_state = r.read_enum(bytes)?,
                Ok(56) => msg.download_state = r.read_enum(bytes)?,
                Ok(66) => msg.mod_list.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(74) => msg.stream_screen_coordinates = Some(r.read_message::<proto::models::mod_User::Point>(bytes)?),
                Ok(80) => msg.stream_delay_ms = r.read_int64(bytes)?,
                Ok(88) => msg.stream_sync_start_ms = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for User<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.guid == "" { 0 } else { 1 + sizeof_len((&self.guid).len()) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.user_id == "" { 0 } else { 1 + sizeof_len((&self.user_id).len()) }
        + if self.client_type == proto::models::mod_User::ClientTypes::Player { 0 } else { 1 + sizeof_varint(*(&self.client_type) as u64) }
        + self.team.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.play_state == proto::models::mod_User::PlayStates::Waiting { 0 } else { 1 + sizeof_varint(*(&self.play_state) as u64) }
        + if self.download_state == proto::models::mod_User::DownloadStates::None_pb { 0 } else { 1 + sizeof_varint(*(&self.download_state) as u64) }
        + self.mod_list.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.stream_screen_coordinates.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.stream_delay_ms == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.stream_delay_ms) as u64) }
        + if self.stream_sync_start_ms == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.stream_sync_start_ms) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.guid != "" { w.write_with_tag(10, |w| w.write_string(&**&self.guid))?; }
        if self.name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if self.user_id != "" { w.write_with_tag(26, |w| w.write_string(&**&self.user_id))?; }
        if self.client_type != proto::models::mod_User::ClientTypes::Player { w.write_with_tag(32, |w| w.write_enum(*&self.client_type as i32))?; }
        if let Some(ref s) = self.team { w.write_with_tag(42, |w| w.write_message(s))?; }
        if self.play_state != proto::models::mod_User::PlayStates::Waiting { w.write_with_tag(48, |w| w.write_enum(*&self.play_state as i32))?; }
        if self.download_state != proto::models::mod_User::DownloadStates::None_pb { w.write_with_tag(56, |w| w.write_enum(*&self.download_state as i32))?; }
        for s in &self.mod_list { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.stream_screen_coordinates { w.write_with_tag(74, |w| w.write_message(s))?; }
        if self.stream_delay_ms != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.stream_delay_ms))?; }
        if self.stream_sync_start_ms != 0i64 { w.write_with_tag(88, |w| w.write_int64(*&self.stream_sync_start_ms))?; }
        Ok(())
    }
}

pub mod mod_User {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl<'a> MessageRead<'a> for Point {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.x = r.read_int32(bytes)?,
                Ok(16) => msg.y = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Point {
    fn get_size(&self) -> usize {
        0
        + if self.x == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.x) as u64) }
        + if self.y == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.y) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.x != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.x))?; }
        if self.y != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.y))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]pub enum PlayStates {
    Waiting = 0,
    InGame = 1,
}

impl Default for PlayStates {
    fn default() -> Self {
        PlayStates::Waiting
    }
}

impl From<i32> for PlayStates {
    fn from(i: i32) -> Self {
        match i {
            0 => PlayStates::Waiting,
            1 => PlayStates::InGame,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for PlayStates {
    fn from(s: &'a str) -> Self {
        match s {
            "Waiting" => PlayStates::Waiting,
            "InGame" => PlayStates::InGame,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]pub enum DownloadStates {
    None_pb = 0,
    Downloading = 1,
    Downloaded = 2,
    DownloadError = 3,
}

impl Default for DownloadStates {
    fn default() -> Self {
        DownloadStates::None_pb
    }
}

impl From<i32> for DownloadStates {
    fn from(i: i32) -> Self {
        match i {
            0 => DownloadStates::None_pb,
            1 => DownloadStates::Downloading,
            2 => DownloadStates::Downloaded,
            3 => DownloadStates::DownloadError,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for DownloadStates {
    fn from(s: &'a str) -> Self {
        match s {
            "None_pb" => DownloadStates::None_pb,
            "Downloading" => DownloadStates::Downloading,
            "Downloaded" => DownloadStates::Downloaded,
            "DownloadError" => DownloadStates::DownloadError,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum ClientTypes {
    Player = 0,
    Coordinator = 1,
    TemporaryConnection = 2,
    WebsocketConnection = 3,
}

impl Default for ClientTypes {
    fn default() -> Self {
        ClientTypes::Player
    }
}

impl From<i32> for ClientTypes {
    fn from(i: i32) -> Self {
        match i {
            0 => ClientTypes::Player,
            1 => ClientTypes::Coordinator,
            2 => ClientTypes::TemporaryConnection,
            3 => ClientTypes::WebsocketConnection,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ClientTypes {
    fn from(s: &'a str) -> Self {
        match s {
            "Player" => ClientTypes::Player,
            "Coordinator" => ClientTypes::Coordinator,
            "TemporaryConnection" => ClientTypes::TemporaryConnection,
            "WebsocketConnection" => ClientTypes::WebsocketConnection,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct Match<'a> {
    pub guid: Cow<'a, str>,
    pub associated_users: Vec<Cow<'a, str>>,
    pub leader: Cow<'a, str>,
    pub selected_level: Option<proto::models::PreviewBeatmapLevel<'a>>,
    pub selected_characteristic: Option<proto::models::Characteristic<'a>>,
    pub selected_difficulty: i32,
    pub start_time: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Match<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.guid = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.associated_users.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.leader = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.selected_level = Some(r.read_message::<proto::models::PreviewBeatmapLevel>(bytes)?),
                Ok(50) => msg.selected_characteristic = Some(r.read_message::<proto::models::Characteristic>(bytes)?),
                Ok(56) => msg.selected_difficulty = r.read_int32(bytes)?,
                Ok(66) => msg.start_time = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Match<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.guid == "" { 0 } else { 1 + sizeof_len((&self.guid).len()) }
        + self.associated_users.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.leader == "" { 0 } else { 1 + sizeof_len((&self.leader).len()) }
        + self.selected_level.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.selected_characteristic.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.selected_difficulty == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.selected_difficulty) as u64) }
        + if self.start_time == "" { 0 } else { 1 + sizeof_len((&self.start_time).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.guid != "" { w.write_with_tag(10, |w| w.write_string(&**&self.guid))?; }
        for s in &self.associated_users { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if self.leader != "" { w.write_with_tag(26, |w| w.write_string(&**&self.leader))?; }
        if let Some(ref s) = self.selected_level { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.selected_characteristic { w.write_with_tag(50, |w| w.write_message(s))?; }
        if self.selected_difficulty != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.selected_difficulty))?; }
        if self.start_time != "" { w.write_with_tag(66, |w| w.write_string(&**&self.start_time))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct QualifierEvent<'a> {
    pub guid: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub guild: Option<proto::discord::Guild<'a>>,
    pub info_channel: Option<proto::discord::Channel<'a>>,
    pub qualifier_maps: Vec<proto::models::GameplayParameters<'a>>,
    pub send_scores_to_info_channel: bool,
    pub flags: i32,
}

impl<'a> MessageRead<'a> for QualifierEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.guid = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.guild = Some(r.read_message::<proto::discord::Guild>(bytes)?),
                Ok(34) => msg.info_channel = Some(r.read_message::<proto::discord::Channel>(bytes)?),
                Ok(42) => msg.qualifier_maps.push(r.read_message::<proto::models::GameplayParameters>(bytes)?),
                Ok(48) => msg.send_scores_to_info_channel = r.read_bool(bytes)?,
                Ok(56) => msg.flags = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QualifierEvent<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.guid == "" { 0 } else { 1 + sizeof_len((&self.guid).len()) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + self.guild.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.info_channel.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.qualifier_maps.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.send_scores_to_info_channel == false { 0 } else { 1 + sizeof_varint(*(&self.send_scores_to_info_channel) as u64) }
        + if self.flags == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.flags) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.guid != "" { w.write_with_tag(10, |w| w.write_string(&**&self.guid))?; }
        if self.name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if let Some(ref s) = self.guild { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.info_channel { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.qualifier_maps { w.write_with_tag(42, |w| w.write_message(s))?; }
        if self.send_scores_to_info_channel != false { w.write_with_tag(48, |w| w.write_bool(*&self.send_scores_to_info_channel))?; }
        if self.flags != 0i32 { w.write_with_tag(56, |w| w.write_int32(*&self.flags))?; }
        Ok(())
    }
}

pub mod mod_QualifierEvent {


#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize, serde::Deserialize)]pub enum EventSettings {
    None_pb = 0,
    HideScoresFromPlayers = 1,
    DisableScoresaberSubmission = 2,
    EnableLeaderboardMessage = 4,
}

impl Default for EventSettings {
    fn default() -> Self {
        EventSettings::None_pb
    }
}

impl From<i32> for EventSettings {
    fn from(i: i32) -> Self {
        match i {
            0 => EventSettings::None_pb,
            1 => EventSettings::HideScoresFromPlayers,
            2 => EventSettings::DisableScoresaberSubmission,
            4 => EventSettings::EnableLeaderboardMessage,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for EventSettings {
    fn from(s: &'a str) -> Self {
        match s {
            "None_pb" => EventSettings::None_pb,
            "HideScoresFromPlayers" => EventSettings::HideScoresFromPlayers,
            "DisableScoresaberSubmission" => EventSettings::DisableScoresaberSubmission,
            "EnableLeaderboardMessage" => EventSettings::EnableLeaderboardMessage,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct CoreServer<'a> {
    pub name: Cow<'a, str>,
    pub address: Cow<'a, str>,
    pub port: i32,
}

impl<'a> MessageRead<'a> for CoreServer<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.address = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.port = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CoreServer<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.address == "" { 0 } else { 1 + sizeof_len((&self.address).len()) }
        + if self.port == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.port) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != "" { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        if self.address != "" { w.write_with_tag(18, |w| w.write_string(&**&self.address))?; }
        if self.port != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.port))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct State<'a> {
    pub server_settings: Option<proto::models::ServerSettings<'a>>,
    pub users: Vec<proto::models::User<'a>>,
    pub matches: Vec<proto::models::Match<'a>>,
    pub events: Vec<proto::models::QualifierEvent<'a>>,
    pub known_hosts: Vec<proto::models::CoreServer<'a>>,
}

impl<'a> MessageRead<'a> for State<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.server_settings = Some(r.read_message::<proto::models::ServerSettings>(bytes)?),
                Ok(18) => msg.users.push(r.read_message::<proto::models::User>(bytes)?),
                Ok(34) => msg.matches.push(r.read_message::<proto::models::Match>(bytes)?),
                Ok(42) => msg.events.push(r.read_message::<proto::models::QualifierEvent>(bytes)?),
                Ok(50) => msg.known_hosts.push(r.read_message::<proto::models::CoreServer>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for State<'a> {
    fn get_size(&self) -> usize {
        0
        + self.server_settings.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.users.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.matches.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.events.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.known_hosts.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.server_settings { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.users { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.matches { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.events { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.known_hosts { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct LeaderboardScore<'a> {
    pub event_id: Cow<'a, str>,
    pub parameters: Option<proto::models::GameplayParameters<'a>>,
    pub user_id: Cow<'a, str>,
    pub username: Cow<'a, str>,
    pub score: i32,
    pub full_combo: bool,
    pub color: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for LeaderboardScore<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.event_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.parameters = Some(r.read_message::<proto::models::GameplayParameters>(bytes)?),
                Ok(26) => msg.user_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.username = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.score = r.read_int32(bytes)?,
                Ok(48) => msg.full_combo = r.read_bool(bytes)?,
                Ok(58) => msg.color = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LeaderboardScore<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.event_id == "" { 0 } else { 1 + sizeof_len((&self.event_id).len()) }
        + self.parameters.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.user_id == "" { 0 } else { 1 + sizeof_len((&self.user_id).len()) }
        + if self.username == "" { 0 } else { 1 + sizeof_len((&self.username).len()) }
        + if self.score == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.score) as u64) }
        + if self.full_combo == false { 0 } else { 1 + sizeof_varint(*(&self.full_combo) as u64) }
        + if self.color == "" { 0 } else { 1 + sizeof_len((&self.color).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.event_id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.event_id))?; }
        if let Some(ref s) = self.parameters { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.user_id != "" { w.write_with_tag(26, |w| w.write_string(&**&self.user_id))?; }
        if self.username != "" { w.write_with_tag(34, |w| w.write_string(&**&self.username))?; }
        if self.score != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.score))?; }
        if self.full_combo != false { w.write_with_tag(48, |w| w.write_bool(*&self.full_combo))?; }
        if self.color != "" { w.write_with_tag(58, |w| w.write_string(&**&self.color))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct ModalOption<'a> {
    pub label: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ModalOption<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.label = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ModalOption<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.label == "" { 0 } else { 1 + sizeof_len((&self.label).len()) }
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.label != "" { w.write_with_tag(10, |w| w.write_string(&**&self.label))?; }
        if self.value != "" { w.write_with_tag(18, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct ScoreTrackerHand<'a> {
    pub hit: i32,
    pub miss: i32,
    pub badCut: i32,
    pub avgCut: Cow<'a, [f32]>,
}

impl<'a> MessageRead<'a> for ScoreTrackerHand<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.hit = r.read_int32(bytes)?,
                Ok(16) => msg.miss = r.read_int32(bytes)?,
                Ok(24) => msg.badCut = r.read_int32(bytes)?,
                Ok(34) => msg.avgCut = r.read_packed_fixed(bytes)?.into(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ScoreTrackerHand<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.hit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.hit) as u64) }
        + if self.miss == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.miss) as u64) }
        + if self.badCut == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.badCut) as u64) }
        + if self.avgCut.is_empty() { 0 } else { 1 + sizeof_len(self.avgCut.len() * 4) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.hit != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.hit))?; }
        if self.miss != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.miss))?; }
        if self.badCut != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.badCut))?; }
        w.write_packed_fixed_with_tag(34, &self.avgCut)?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize, serde::Deserialize)]

pub struct ScoreTracker<'a> {
    pub notesMissed: i32,
    pub badCuts: i32,
    pub bombHits: i32,
    pub wallHits: i32,
    pub maxCombo: i32,
    pub leftHand: Option<proto::models::ScoreTrackerHand<'a>>,
    pub rightHand: Option<proto::models::ScoreTrackerHand<'a>>,
}

impl<'a> MessageRead<'a> for ScoreTracker<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.notesMissed = r.read_int32(bytes)?,
                Ok(16) => msg.badCuts = r.read_int32(bytes)?,
                Ok(24) => msg.bombHits = r.read_int32(bytes)?,
                Ok(32) => msg.wallHits = r.read_int32(bytes)?,
                Ok(40) => msg.maxCombo = r.read_int32(bytes)?,
                Ok(50) => msg.leftHand = Some(r.read_message::<proto::models::ScoreTrackerHand>(bytes)?),
                Ok(58) => msg.rightHand = Some(r.read_message::<proto::models::ScoreTrackerHand>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ScoreTracker<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.notesMissed == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.notesMissed) as u64) }
        + if self.badCuts == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.badCuts) as u64) }
        + if self.bombHits == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.bombHits) as u64) }
        + if self.wallHits == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.wallHits) as u64) }
        + if self.maxCombo == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.maxCombo) as u64) }
        + self.leftHand.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.rightHand.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.notesMissed != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.notesMissed))?; }
        if self.badCuts != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.badCuts))?; }
        if self.bombHits != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.bombHits))?; }
        if self.wallHits != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.wallHits))?; }
        if self.maxCombo != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.maxCombo))?; }
        if let Some(ref s) = self.leftHand { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.rightHand { w.write_with_tag(58, |w| w.write_message(s))?; }
        Ok(())
    }
}

