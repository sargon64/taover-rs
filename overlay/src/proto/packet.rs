// Automatically generated rust module for 'packets.proto' file

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
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]
pub struct Command<'a> {
    pub type_pb: proto::packet::mod_Command::OneOftype_pb<'a>,
}

impl<'a> MessageRead<'a> for Command<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = proto::packet::mod_Command::OneOftype_pb::heartbeat(r.read_bool(bytes)?),
                Ok(16) => msg.type_pb = proto::packet::mod_Command::OneOftype_pb::return_to_menu(r.read_bool(bytes)?),
                Ok(24) => msg.type_pb = proto::packet::mod_Command::OneOftype_pb::delay_test_finish(r.read_bool(bytes)?),
                Ok(32) => msg.type_pb = proto::packet::mod_Command::OneOftype_pb::stream_sync_show_image(r.read_bool(bytes)?),
                Ok(42) => msg.type_pb = proto::packet::mod_Command::OneOftype_pb::load_song(r.read_message::<proto::packet::mod_Command::LoadSong>(bytes)?),
                Ok(50) => msg.type_pb = proto::packet::mod_Command::OneOftype_pb::play_song(r.read_message::<proto::packet::mod_Command::PlaySong>(bytes)?),
                Ok(58) => msg.type_pb = proto::packet::mod_Command::OneOftype_pb::send_bot_message(r.read_message::<proto::packet::mod_Command::SendBotMessage>(bytes)?),
                Ok(66) => msg.type_pb = proto::packet::mod_Command::OneOftype_pb::show_modal(r.read_message::<proto::packet::mod_Command::ShowModal>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Command<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.type_pb {
            proto::packet::mod_Command::OneOftype_pb::heartbeat(ref m) => 1 + sizeof_varint(*(m) as u64),
            proto::packet::mod_Command::OneOftype_pb::return_to_menu(ref m) => 1 + sizeof_varint(*(m) as u64),
            proto::packet::mod_Command::OneOftype_pb::delay_test_finish(ref m) => 1 + sizeof_varint(*(m) as u64),
            proto::packet::mod_Command::OneOftype_pb::stream_sync_show_image(ref m) => 1 + sizeof_varint(*(m) as u64),
            proto::packet::mod_Command::OneOftype_pb::load_song(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Command::OneOftype_pb::play_song(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Command::OneOftype_pb::send_bot_message(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Command::OneOftype_pb::show_modal(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Command::OneOftype_pb::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.type_pb {            proto::packet::mod_Command::OneOftype_pb::heartbeat(ref m) => { w.write_with_tag(8, |w| w.write_bool(*m))? },
            proto::packet::mod_Command::OneOftype_pb::return_to_menu(ref m) => { w.write_with_tag(16, |w| w.write_bool(*m))? },
            proto::packet::mod_Command::OneOftype_pb::delay_test_finish(ref m) => { w.write_with_tag(24, |w| w.write_bool(*m))? },
            proto::packet::mod_Command::OneOftype_pb::stream_sync_show_image(ref m) => { w.write_with_tag(32, |w| w.write_bool(*m))? },
            proto::packet::mod_Command::OneOftype_pb::load_song(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            proto::packet::mod_Command::OneOftype_pb::play_song(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            proto::packet::mod_Command::OneOftype_pb::send_bot_message(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            proto::packet::mod_Command::OneOftype_pb::show_modal(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            proto::packet::mod_Command::OneOftype_pb::None => {},
    }        Ok(())
    }
}

pub mod mod_Command {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct LoadSong<'a> {
    pub level_id: Cow<'a, str>,
    pub custom_host_url: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for LoadSong<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.level_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.custom_host_url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LoadSong<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.level_id == "" { 0 } else { 1 + sizeof_len((&self.level_id).len()) }
        + if self.custom_host_url == "" { 0 } else { 1 + sizeof_len((&self.custom_host_url).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.level_id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.level_id))?; }
        if self.custom_host_url != "" { w.write_with_tag(18, |w| w.write_string(&**&self.custom_host_url))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct PlaySong<'a> {
    pub gameplay_parameters: Option<proto::models::GameplayParameters<'a>>,
    pub floating_scoreboard: bool,
    pub stream_sync: bool,
    pub disable_fail: bool,
    pub disable_pause: bool,
    pub disable_scoresaber_submission: bool,
    pub show_normal_notes_on_stream: bool,
}

impl<'a> MessageRead<'a> for PlaySong<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.gameplay_parameters = Some(r.read_message::<proto::models::GameplayParameters>(bytes)?),
                Ok(24) => msg.floating_scoreboard = r.read_bool(bytes)?,
                Ok(32) => msg.stream_sync = r.read_bool(bytes)?,
                Ok(40) => msg.disable_fail = r.read_bool(bytes)?,
                Ok(48) => msg.disable_pause = r.read_bool(bytes)?,
                Ok(56) => msg.disable_scoresaber_submission = r.read_bool(bytes)?,
                Ok(64) => msg.show_normal_notes_on_stream = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PlaySong<'a> {
    fn get_size(&self) -> usize {
        0
        + self.gameplay_parameters.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.floating_scoreboard == false { 0 } else { 1 + sizeof_varint(*(&self.floating_scoreboard) as u64) }
        + if self.stream_sync == false { 0 } else { 1 + sizeof_varint(*(&self.stream_sync) as u64) }
        + if self.disable_fail == false { 0 } else { 1 + sizeof_varint(*(&self.disable_fail) as u64) }
        + if self.disable_pause == false { 0 } else { 1 + sizeof_varint(*(&self.disable_pause) as u64) }
        + if self.disable_scoresaber_submission == false { 0 } else { 1 + sizeof_varint(*(&self.disable_scoresaber_submission) as u64) }
        + if self.show_normal_notes_on_stream == false { 0 } else { 1 + sizeof_varint(*(&self.show_normal_notes_on_stream) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.gameplay_parameters { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.floating_scoreboard != false { w.write_with_tag(24, |w| w.write_bool(*&self.floating_scoreboard))?; }
        if self.stream_sync != false { w.write_with_tag(32, |w| w.write_bool(*&self.stream_sync))?; }
        if self.disable_fail != false { w.write_with_tag(40, |w| w.write_bool(*&self.disable_fail))?; }
        if self.disable_pause != false { w.write_with_tag(48, |w| w.write_bool(*&self.disable_pause))?; }
        if self.disable_scoresaber_submission != false { w.write_with_tag(56, |w| w.write_bool(*&self.disable_scoresaber_submission))?; }
        if self.show_normal_notes_on_stream != false { w.write_with_tag(64, |w| w.write_bool(*&self.show_normal_notes_on_stream))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct SendBotMessage<'a> {
    pub channel: Option<proto::discord::Channel<'a>>,
    pub message: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for SendBotMessage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.channel = Some(r.read_message::<proto::discord::Channel>(bytes)?),
                Ok(18) => msg.message = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SendBotMessage<'a> {
    fn get_size(&self) -> usize {
        0
        + self.channel.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.message == "" { 0 } else { 1 + sizeof_len((&self.message).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.channel { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.message != "" { w.write_with_tag(18, |w| w.write_string(&**&self.message))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct ShowModal<'a> {
    pub modal_id: Cow<'a, str>,
    pub message_title: Cow<'a, str>,
    pub message_text: Cow<'a, str>,
    pub can_close: bool,
    pub option_1: Option<proto::models::ModalOption<'a>>,
    pub option_2: Option<proto::models::ModalOption<'a>>,
}

impl<'a> MessageRead<'a> for ShowModal<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.modal_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.message_title = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.message_text = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.can_close = r.read_bool(bytes)?,
                Ok(42) => msg.option_1 = Some(r.read_message::<proto::models::ModalOption>(bytes)?),
                Ok(50) => msg.option_2 = Some(r.read_message::<proto::models::ModalOption>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ShowModal<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.modal_id == "" { 0 } else { 1 + sizeof_len((&self.modal_id).len()) }
        + if self.message_title == "" { 0 } else { 1 + sizeof_len((&self.message_title).len()) }
        + if self.message_text == "" { 0 } else { 1 + sizeof_len((&self.message_text).len()) }
        + if self.can_close == false { 0 } else { 1 + sizeof_varint(*(&self.can_close) as u64) }
        + self.option_1.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.option_2.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.modal_id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.modal_id))?; }
        if self.message_title != "" { w.write_with_tag(18, |w| w.write_string(&**&self.message_title))?; }
        if self.message_text != "" { w.write_with_tag(26, |w| w.write_string(&**&self.message_text))?; }
        if self.can_close != false { w.write_with_tag(32, |w| w.write_bool(*&self.can_close))?; }
        if let Some(ref s) = self.option_1 { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.option_2 { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum OneOftype_pb<'a> {
    heartbeat(bool),
    return_to_menu(bool),
    delay_test_finish(bool),
    stream_sync_show_image(bool),
    load_song(proto::packet::mod_Command::LoadSong<'a>),
    play_song(proto::packet::mod_Command::PlaySong<'a>),
    send_bot_message(proto::packet::mod_Command::SendBotMessage<'a>),
    show_modal(proto::packet::mod_Command::ShowModal<'a>),
    None,
}

impl<'a> Default for OneOftype_pb<'a> {
    fn default() -> Self {
        OneOftype_pb::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct Push<'a> {
    pub data: proto::packet::mod_Push::OneOfdata<'a>,
}

impl<'a> MessageRead<'a> for Push<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.data = proto::packet::mod_Push::OneOfdata::leaderboard_score(r.read_message::<proto::packet::mod_Push::LeaderboardScore>(bytes)?),
                Ok(18) => msg.data = proto::packet::mod_Push::OneOfdata::realtime_score(r.read_message::<proto::packet::mod_Push::RealtimeScore>(bytes)?),
                Ok(26) => msg.data = proto::packet::mod_Push::OneOfdata::song_finished(r.read_message::<proto::packet::mod_Push::SongFinished>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Push<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.data {
            proto::packet::mod_Push::OneOfdata::leaderboard_score(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Push::OneOfdata::realtime_score(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Push::OneOfdata::song_finished(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Push::OneOfdata::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.data {            proto::packet::mod_Push::OneOfdata::leaderboard_score(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            proto::packet::mod_Push::OneOfdata::realtime_score(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            proto::packet::mod_Push::OneOfdata::song_finished(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            proto::packet::mod_Push::OneOfdata::None => {},
    }        Ok(())
    }
}

pub mod mod_Push {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct LeaderboardScore<'a> {
    pub score: Option<proto::models::LeaderboardScore<'a>>,
}

impl<'a> MessageRead<'a> for LeaderboardScore<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.score = Some(r.read_message::<proto::models::LeaderboardScore>(bytes)?),
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
        + self.score.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.score { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct RealtimeScore<'a> {
    pub user_guid: Cow<'a, str>,
    pub score: i32,
    pub score_with_modifiers: i32,
    pub max_score: i32,
    pub max_score_with_modifiers: i32,
    pub combo: i32,
    pub player_health: f32,
    pub accuracy: f32,
    pub song_position: f32,
    pub scoreTracker: Option<proto::models::ScoreTracker<'a>>,
}

impl<'a> MessageRead<'a> for RealtimeScore<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.user_guid = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.score = r.read_int32(bytes)?,
                Ok(24) => msg.score_with_modifiers = r.read_int32(bytes)?,
                Ok(32) => msg.max_score = r.read_int32(bytes)?,
                Ok(40) => msg.max_score_with_modifiers = r.read_int32(bytes)?,
                Ok(48) => msg.combo = r.read_int32(bytes)?,
                Ok(61) => msg.player_health = r.read_float(bytes)?,
                Ok(69) => msg.accuracy = r.read_float(bytes)?,
                Ok(77) => msg.song_position = r.read_float(bytes)?,
                Ok(82) => msg.scoreTracker = Some(r.read_message::<proto::models::ScoreTracker>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RealtimeScore<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.user_guid == "" { 0 } else { 1 + sizeof_len((&self.user_guid).len()) }
        + if self.score == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.score) as u64) }
        + if self.score_with_modifiers == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.score_with_modifiers) as u64) }
        + if self.max_score == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.max_score) as u64) }
        + if self.max_score_with_modifiers == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.max_score_with_modifiers) as u64) }
        + if self.combo == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.combo) as u64) }
        + if self.player_health == 0f32 { 0 } else { 1 + 4 }
        + if self.accuracy == 0f32 { 0 } else { 1 + 4 }
        + if self.song_position == 0f32 { 0 } else { 1 + 4 }
        + self.scoreTracker.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.user_guid != "" { w.write_with_tag(10, |w| w.write_string(&**&self.user_guid))?; }
        if self.score != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.score))?; }
        if self.score_with_modifiers != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.score_with_modifiers))?; }
        if self.max_score != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.max_score))?; }
        if self.max_score_with_modifiers != 0i32 { w.write_with_tag(40, |w| w.write_int32(*&self.max_score_with_modifiers))?; }
        if self.combo != 0i32 { w.write_with_tag(48, |w| w.write_int32(*&self.combo))?; }
        if self.player_health != 0f32 { w.write_with_tag(61, |w| w.write_float(*&self.player_health))?; }
        if self.accuracy != 0f32 { w.write_with_tag(69, |w| w.write_float(*&self.accuracy))?; }
        if self.song_position != 0f32 { w.write_with_tag(77, |w| w.write_float(*&self.song_position))?; }
        if let Some(ref s) = self.scoreTracker { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct SongFinished<'a> {
    pub player: Option<proto::models::User<'a>>,
    pub beatmap: Option<proto::models::Beatmap<'a>>,
    pub type_pb: proto::packet::mod_Push::mod_SongFinished::CompletionType,
    pub score: i32,
}

impl<'a> MessageRead<'a> for SongFinished<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.player = Some(r.read_message::<proto::models::User>(bytes)?),
                Ok(18) => msg.beatmap = Some(r.read_message::<proto::models::Beatmap>(bytes)?),
                Ok(24) => msg.type_pb = r.read_enum(bytes)?,
                Ok(32) => msg.score = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SongFinished<'a> {
    fn get_size(&self) -> usize {
        0
        + self.player.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.beatmap.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.type_pb == proto::packet::mod_Push::mod_SongFinished::CompletionType::Passed { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.score == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.score) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.player { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.beatmap { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.type_pb != proto::packet::mod_Push::mod_SongFinished::CompletionType::Passed { w.write_with_tag(24, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.score != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.score))?; }
        Ok(())
    }
}

pub mod mod_SongFinished {


#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize)]pub enum CompletionType {
    Passed = 0,
    Failed = 1,
    Quit = 2,
}

impl Default for CompletionType {
    fn default() -> Self {
        CompletionType::Passed
    }
}

impl From<i32> for CompletionType {
    fn from(i: i32) -> Self {
        match i {
            0 => CompletionType::Passed,
            1 => CompletionType::Failed,
            2 => CompletionType::Quit,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for CompletionType {
    fn from(s: &'a str) -> Self {
        match s {
            "Passed" => CompletionType::Passed,
            "Failed" => CompletionType::Failed,
            "Quit" => CompletionType::Quit,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum OneOfdata<'a> {
    leaderboard_score(proto::packet::mod_Push::LeaderboardScore<'a>),
    realtime_score(proto::packet::mod_Push::RealtimeScore<'a>),
    song_finished(proto::packet::mod_Push::SongFinished<'a>),
    None,
}

impl<'a> Default for OneOfdata<'a> {
    fn default() -> Self {
        OneOfdata::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct Request<'a> {
    pub type_pb: proto::packet::mod_Request::OneOftype_pb<'a>,
}

impl<'a> MessageRead<'a> for Request<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.type_pb = proto::packet::mod_Request::OneOftype_pb::connect(r.read_message::<proto::packet::mod_Request::Connect>(bytes)?),
                Ok(18) => msg.type_pb = proto::packet::mod_Request::OneOftype_pb::leaderboard_score(r.read_message::<proto::packet::mod_Request::LeaderboardScore>(bytes)?),
                Ok(26) => msg.type_pb = proto::packet::mod_Request::OneOftype_pb::preload_image_for_stream_sync(r.read_message::<proto::packet::mod_Request::PreloadImageForStreamSync>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Request<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.type_pb {
            proto::packet::mod_Request::OneOftype_pb::connect(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Request::OneOftype_pb::leaderboard_score(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Request::OneOftype_pb::preload_image_for_stream_sync(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Request::OneOftype_pb::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.type_pb {            proto::packet::mod_Request::OneOftype_pb::connect(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            proto::packet::mod_Request::OneOftype_pb::leaderboard_score(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            proto::packet::mod_Request::OneOftype_pb::preload_image_for_stream_sync(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            proto::packet::mod_Request::OneOftype_pb::None => {},
    }        Ok(())
    }
}

pub mod mod_Request {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct Connect<'a> {
    pub user: Option<proto::models::User<'a>>,
    pub password: Cow<'a, str>,
    pub client_version: i32,
}

impl<'a> MessageRead<'a> for Connect<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.user = Some(r.read_message::<proto::models::User>(bytes)?),
                Ok(18) => msg.password = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.client_version = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Connect<'a> {
    fn get_size(&self) -> usize {
        0
        + self.user.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.password == "" { 0 } else { 1 + sizeof_len((&self.password).len()) }
        + if self.client_version == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.client_version) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.user { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.password != "" { w.write_with_tag(18, |w| w.write_string(&**&self.password))?; }
        if self.client_version != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.client_version))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct LeaderboardScore<'a> {
    pub event_id: Cow<'a, str>,
    pub parameters: Option<proto::models::GameplayParameters<'a>>,
}

impl<'a> MessageRead<'a> for LeaderboardScore<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.event_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.parameters = Some(r.read_message::<proto::models::GameplayParameters>(bytes)?),
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
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.event_id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.event_id))?; }
        if let Some(ref s) = self.parameters { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct PreloadImageForStreamSync<'a> {
    pub file_id: Cow<'a, str>,
    pub compressed: bool,
    pub data: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for PreloadImageForStreamSync<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.file_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.compressed = r.read_bool(bytes)?,
                Ok(26) => msg.data = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PreloadImageForStreamSync<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.file_id == "" { 0 } else { 1 + sizeof_len((&self.file_id).len()) }
        + if self.compressed == false { 0 } else { 1 + sizeof_varint(*(&self.compressed) as u64) }
        + if self.data == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.file_id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.file_id))?; }
        if self.compressed != false { w.write_with_tag(16, |w| w.write_bool(*&self.compressed))?; }
        if self.data != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.data))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum OneOftype_pb<'a> {
    connect(proto::packet::mod_Request::Connect<'a>),
    leaderboard_score(proto::packet::mod_Request::LeaderboardScore<'a>),
    preload_image_for_stream_sync(proto::packet::mod_Request::PreloadImageForStreamSync<'a>),
    None,
}

impl<'a> Default for OneOftype_pb<'a> {
    fn default() -> Self {
        OneOftype_pb::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct Response<'a> {
    pub type_pb: proto::packet::mod_Response::ResponseType,
    pub responding_to_packet_id: Cow<'a, str>,
    pub details: proto::packet::mod_Response::OneOfdetails<'a>,
}

impl<'a> MessageRead<'a> for Response<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_enum(bytes)?,
                Ok(18) => msg.responding_to_packet_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.details = proto::packet::mod_Response::OneOfdetails::connect(r.read_message::<proto::packet::mod_Response::Connect>(bytes)?),
                Ok(34) => msg.details = proto::packet::mod_Response::OneOfdetails::leaderboard_scores(r.read_message::<proto::packet::mod_Response::LeaderboardScores>(bytes)?),
                Ok(42) => msg.details = proto::packet::mod_Response::OneOfdetails::loaded_song(r.read_message::<proto::packet::mod_Response::LoadedSong>(bytes)?),
                Ok(50) => msg.details = proto::packet::mod_Response::OneOfdetails::modal(r.read_message::<proto::packet::mod_Response::Modal>(bytes)?),
                Ok(58) => msg.details = proto::packet::mod_Response::OneOfdetails::modify_qualifier(r.read_message::<proto::packet::mod_Response::ModifyQualifier>(bytes)?),
                Ok(66) => msg.details = proto::packet::mod_Response::OneOfdetails::image_preloaded(r.read_message::<proto::packet::mod_Response::ImagePreloaded>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Response<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.type_pb == proto::packet::mod_Response::ResponseType::Fail { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.responding_to_packet_id == "" { 0 } else { 1 + sizeof_len((&self.responding_to_packet_id).len()) }
        + match self.details {
            proto::packet::mod_Response::OneOfdetails::connect(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Response::OneOfdetails::leaderboard_scores(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Response::OneOfdetails::loaded_song(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Response::OneOfdetails::modal(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Response::OneOfdetails::modify_qualifier(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Response::OneOfdetails::image_preloaded(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Response::OneOfdetails::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.type_pb != proto::packet::mod_Response::ResponseType::Fail { w.write_with_tag(8, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.responding_to_packet_id != "" { w.write_with_tag(18, |w| w.write_string(&**&self.responding_to_packet_id))?; }
        match self.details {            proto::packet::mod_Response::OneOfdetails::connect(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            proto::packet::mod_Response::OneOfdetails::leaderboard_scores(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            proto::packet::mod_Response::OneOfdetails::loaded_song(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            proto::packet::mod_Response::OneOfdetails::modal(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            proto::packet::mod_Response::OneOfdetails::modify_qualifier(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            proto::packet::mod_Response::OneOfdetails::image_preloaded(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            proto::packet::mod_Response::OneOfdetails::None => {},
    }        Ok(())
    }
}

pub mod mod_Response {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct Connect<'a> {
    pub state: Option<proto::models::State<'a>>,
    pub self_guid: Cow<'a, str>,
    pub server_version: i32,
    pub message: Cow<'a, str>,
    pub reason: proto::packet::mod_Response::ConnectFailReason,
}

impl<'a> MessageRead<'a> for Connect<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.state = Some(r.read_message::<proto::models::State>(bytes)?),
                Ok(18) => msg.self_guid = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.server_version = r.read_int32(bytes)?,
                Ok(34) => msg.message = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.reason = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Connect<'a> {
    fn get_size(&self) -> usize {
        0
        + self.state.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.self_guid == "" { 0 } else { 1 + sizeof_len((&self.self_guid).len()) }
        + if self.server_version == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.server_version) as u64) }
        + if self.message == "" { 0 } else { 1 + sizeof_len((&self.message).len()) }
        + if self.reason == proto::packet::mod_Response::ConnectFailReason::IncorrectVersion { 0 } else { 1 + sizeof_varint(*(&self.reason) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.state { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.self_guid != "" { w.write_with_tag(18, |w| w.write_string(&**&self.self_guid))?; }
        if self.server_version != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.server_version))?; }
        if self.message != "" { w.write_with_tag(34, |w| w.write_string(&**&self.message))?; }
        if self.reason != proto::packet::mod_Response::ConnectFailReason::IncorrectVersion { w.write_with_tag(40, |w| w.write_enum(*&self.reason as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct LeaderboardScores<'a> {
    pub scores: Vec<proto::models::LeaderboardScore<'a>>,
}

impl<'a> MessageRead<'a> for LeaderboardScores<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.scores.push(r.read_message::<proto::models::LeaderboardScore>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LeaderboardScores<'a> {
    fn get_size(&self) -> usize {
        0
        + self.scores.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.scores { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct LoadedSong<'a> {
    pub level: Option<proto::models::PreviewBeatmapLevel<'a>>,
}

impl<'a> MessageRead<'a> for LoadedSong<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.level = Some(r.read_message::<proto::models::PreviewBeatmapLevel>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LoadedSong<'a> {
    fn get_size(&self) -> usize {
        0
        + self.level.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.level { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct Modal<'a> {
    pub modal_id: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Modal<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.modal_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Modal<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.modal_id == "" { 0 } else { 1 + sizeof_len((&self.modal_id).len()) }
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.modal_id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.modal_id))?; }
        if self.value != "" { w.write_with_tag(18, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct ModifyQualifier<'a> {
    pub message: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ModifyQualifier<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.message = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ModifyQualifier<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.message == "" { 0 } else { 1 + sizeof_len((&self.message).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.message != "" { w.write_with_tag(10, |w| w.write_string(&**&self.message))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct ImagePreloaded<'a> {
    pub file_id: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ImagePreloaded<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.file_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ImagePreloaded<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.file_id == "" { 0 } else { 1 + sizeof_len((&self.file_id).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.file_id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.file_id))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize)]pub enum ResponseType {
    Fail = 0,
    Success = 1,
}

impl Default for ResponseType {
    fn default() -> Self {
        ResponseType::Fail
    }
}

impl From<i32> for ResponseType {
    fn from(i: i32) -> Self {
        match i {
            0 => ResponseType::Fail,
            1 => ResponseType::Success,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ResponseType {
    fn from(s: &'a str) -> Self {
        match s {
            "Fail" => ResponseType::Fail,
            "Success" => ResponseType::Success,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize)]pub enum ConnectFailReason {
    IncorrectVersion = 0,
    IncorrectPassword = 1,
}

impl Default for ConnectFailReason {
    fn default() -> Self {
        ConnectFailReason::IncorrectVersion
    }
}

impl From<i32> for ConnectFailReason {
    fn from(i: i32) -> Self {
        match i {
            0 => ConnectFailReason::IncorrectVersion,
            1 => ConnectFailReason::IncorrectPassword,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ConnectFailReason {
    fn from(s: &'a str) -> Self {
        match s {
            "IncorrectVersion" => ConnectFailReason::IncorrectVersion,
            "IncorrectPassword" => ConnectFailReason::IncorrectPassword,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum OneOfdetails<'a> {
    connect(proto::packet::mod_Response::Connect<'a>),
    leaderboard_scores(proto::packet::mod_Response::LeaderboardScores<'a>),
    loaded_song(proto::packet::mod_Response::LoadedSong<'a>),
    modal(proto::packet::mod_Response::Modal<'a>),
    modify_qualifier(proto::packet::mod_Response::ModifyQualifier<'a>),
    image_preloaded(proto::packet::mod_Response::ImagePreloaded<'a>),
    None,
}

impl<'a> Default for OneOfdetails<'a> {
    fn default() -> Self {
        OneOfdetails::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct Acknowledgement<'a> {
    pub packet_id: Cow<'a, str>,
    pub type_pb: proto::packet::mod_Acknowledgement::AcknowledgementType,
}

impl<'a> MessageRead<'a> for Acknowledgement<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.packet_id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.type_pb = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Acknowledgement<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.packet_id == "" { 0 } else { 1 + sizeof_len((&self.packet_id).len()) }
        + if self.type_pb == proto::packet::mod_Acknowledgement::AcknowledgementType::MessageReceived { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.packet_id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.packet_id))?; }
        if self.type_pb != proto::packet::mod_Acknowledgement::AcknowledgementType::MessageReceived { w.write_with_tag(16, |w| w.write_enum(*&self.type_pb as i32))?; }
        Ok(())
    }
}

pub mod mod_Acknowledgement {


#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Serialize)]
pub enum AcknowledgementType {
    MessageReceived = 0,
}

impl Default for AcknowledgementType {
    fn default() -> Self {
        AcknowledgementType::MessageReceived
    }
}

impl From<i32> for AcknowledgementType {
    fn from(i: i32) -> Self {
        match i {
            0 => AcknowledgementType::MessageReceived,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for AcknowledgementType {
    fn from(s: &'a str) -> Self {
        match s {
            "MessageReceived" => AcknowledgementType::MessageReceived,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct ForwardingPacket<'a> {
    pub forward_to: Vec<Cow<'a, str>>,
    pub packet: Option<Box<proto::packet::Packet<'a>>>,
}

impl<'a> MessageRead<'a> for ForwardingPacket<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.forward_to.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.packet = Some(Box::new(r.read_message::<proto::packet::Packet>(bytes)?)),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ForwardingPacket<'a> {
    fn get_size(&self) -> usize {
        0
        + self.forward_to.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.packet.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.forward_to { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.packet { w.write_with_tag(18, |w| w.write_message(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct Event<'a> {
    pub changed_object: proto::packet::mod_Event::OneOfchanged_object<'a>,
}

impl<'a> MessageRead<'a> for Event<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.changed_object = proto::packet::mod_Event::OneOfchanged_object::user_added_event(r.read_message::<proto::packet::mod_Event::UserAddedEvent>(bytes)?),
                Ok(18) => msg.changed_object = proto::packet::mod_Event::OneOfchanged_object::user_updated_event(r.read_message::<proto::packet::mod_Event::UserUpdatedEvent>(bytes)?),
                Ok(26) => msg.changed_object = proto::packet::mod_Event::OneOfchanged_object::user_left_event(r.read_message::<proto::packet::mod_Event::UserLeftEvent>(bytes)?),
                Ok(50) => msg.changed_object = proto::packet::mod_Event::OneOfchanged_object::match_created_event(r.read_message::<proto::packet::mod_Event::MatchCreatedEvent>(bytes)?),
                Ok(58) => msg.changed_object = proto::packet::mod_Event::OneOfchanged_object::match_updated_event(r.read_message::<proto::packet::mod_Event::MatchUpdatedEvent>(bytes)?),
                Ok(66) => msg.changed_object = proto::packet::mod_Event::OneOfchanged_object::match_deleted_event(r.read_message::<proto::packet::mod_Event::MatchDeletedEvent>(bytes)?),
                Ok(74) => msg.changed_object = proto::packet::mod_Event::OneOfchanged_object::qualifier_created_event(r.read_message::<proto::packet::mod_Event::QualifierCreatedEvent>(bytes)?),
                Ok(82) => msg.changed_object = proto::packet::mod_Event::OneOfchanged_object::qualifier_updated_event(r.read_message::<proto::packet::mod_Event::QualifierUpdatedEvent>(bytes)?),
                Ok(90) => msg.changed_object = proto::packet::mod_Event::OneOfchanged_object::qualifier_deleted_event(r.read_message::<proto::packet::mod_Event::QualifierDeletedEvent>(bytes)?),
                Ok(98) => msg.changed_object = proto::packet::mod_Event::OneOfchanged_object::host_added_event(r.read_message::<proto::packet::mod_Event::HostAddedEvent>(bytes)?),
                Ok(106) => msg.changed_object = proto::packet::mod_Event::OneOfchanged_object::host_deleted_event(r.read_message::<proto::packet::mod_Event::HostDeletedEvent>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Event<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.changed_object {
            proto::packet::mod_Event::OneOfchanged_object::user_added_event(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Event::OneOfchanged_object::user_updated_event(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Event::OneOfchanged_object::user_left_event(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Event::OneOfchanged_object::match_created_event(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Event::OneOfchanged_object::match_updated_event(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Event::OneOfchanged_object::match_deleted_event(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Event::OneOfchanged_object::qualifier_created_event(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Event::OneOfchanged_object::qualifier_updated_event(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Event::OneOfchanged_object::qualifier_deleted_event(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Event::OneOfchanged_object::host_added_event(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Event::OneOfchanged_object::host_deleted_event(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Event::OneOfchanged_object::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.changed_object {            proto::packet::mod_Event::OneOfchanged_object::user_added_event(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            proto::packet::mod_Event::OneOfchanged_object::user_updated_event(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            proto::packet::mod_Event::OneOfchanged_object::user_left_event(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            proto::packet::mod_Event::OneOfchanged_object::match_created_event(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            proto::packet::mod_Event::OneOfchanged_object::match_updated_event(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            proto::packet::mod_Event::OneOfchanged_object::match_deleted_event(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            proto::packet::mod_Event::OneOfchanged_object::qualifier_created_event(ref m) => { w.write_with_tag(74, |w| w.write_message(m))? },
            proto::packet::mod_Event::OneOfchanged_object::qualifier_updated_event(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            proto::packet::mod_Event::OneOfchanged_object::qualifier_deleted_event(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            proto::packet::mod_Event::OneOfchanged_object::host_added_event(ref m) => { w.write_with_tag(98, |w| w.write_message(m))? },
            proto::packet::mod_Event::OneOfchanged_object::host_deleted_event(ref m) => { w.write_with_tag(106, |w| w.write_message(m))? },
            proto::packet::mod_Event::OneOfchanged_object::None => {},
    }        Ok(())
    }
}

pub mod mod_Event {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct UserAddedEvent<'a> {
    pub user: Option<proto::models::User<'a>>,
}

impl<'a> MessageRead<'a> for UserAddedEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.user = Some(r.read_message::<proto::models::User>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UserAddedEvent<'a> {
    fn get_size(&self) -> usize {
        0
        + self.user.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.user { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct UserUpdatedEvent<'a> {
    pub user: Option<proto::models::User<'a>>,
}

impl<'a> MessageRead<'a> for UserUpdatedEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.user = Some(r.read_message::<proto::models::User>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UserUpdatedEvent<'a> {
    fn get_size(&self) -> usize {
        0
        + self.user.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.user { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct UserLeftEvent<'a> {
    pub user: Option<proto::models::User<'a>>,
}

impl<'a> MessageRead<'a> for UserLeftEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.user = Some(r.read_message::<proto::models::User>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UserLeftEvent<'a> {
    fn get_size(&self) -> usize {
        0
        + self.user.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.user { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct MatchCreatedEvent<'a> {
    pub match_pb: Option<proto::models::Match<'a>>,
}

impl<'a> MessageRead<'a> for MatchCreatedEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.match_pb = Some(r.read_message::<proto::models::Match>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MatchCreatedEvent<'a> {
    fn get_size(&self) -> usize {
        0
        + self.match_pb.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.match_pb { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct MatchUpdatedEvent<'a> {
    pub match_pb: Option<proto::models::Match<'a>>,
}

impl<'a> MessageRead<'a> for MatchUpdatedEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.match_pb = Some(r.read_message::<proto::models::Match>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MatchUpdatedEvent<'a> {
    fn get_size(&self) -> usize {
        0
        + self.match_pb.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.match_pb { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct MatchDeletedEvent<'a> {
    pub match_pb: Option<proto::models::Match<'a>>,
}

impl<'a> MessageRead<'a> for MatchDeletedEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.match_pb = Some(r.read_message::<proto::models::Match>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MatchDeletedEvent<'a> {
    fn get_size(&self) -> usize {
        0
        + self.match_pb.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.match_pb { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct QualifierCreatedEvent<'a> {
    pub event: Option<proto::models::QualifierEvent<'a>>,
}

impl<'a> MessageRead<'a> for QualifierCreatedEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.event = Some(r.read_message::<proto::models::QualifierEvent>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QualifierCreatedEvent<'a> {
    fn get_size(&self) -> usize {
        0
        + self.event.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.event { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct QualifierUpdatedEvent<'a> {
    pub event: Option<proto::models::QualifierEvent<'a>>,
}

impl<'a> MessageRead<'a> for QualifierUpdatedEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.event = Some(r.read_message::<proto::models::QualifierEvent>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QualifierUpdatedEvent<'a> {
    fn get_size(&self) -> usize {
        0
        + self.event.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.event { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct QualifierDeletedEvent<'a> {
    pub event: Option<proto::models::QualifierEvent<'a>>,
}

impl<'a> MessageRead<'a> for QualifierDeletedEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.event = Some(r.read_message::<proto::models::QualifierEvent>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for QualifierDeletedEvent<'a> {
    fn get_size(&self) -> usize {
        0
        + self.event.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.event { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct HostAddedEvent<'a> {
    pub server: Option<proto::models::CoreServer<'a>>,
}

impl<'a> MessageRead<'a> for HostAddedEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.server = Some(r.read_message::<proto::models::CoreServer>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HostAddedEvent<'a> {
    fn get_size(&self) -> usize {
        0
        + self.server.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.server { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct HostDeletedEvent<'a> {
    pub server: Option<proto::models::CoreServer<'a>>,
}

impl<'a> MessageRead<'a> for HostDeletedEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.server = Some(r.read_message::<proto::models::CoreServer>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for HostDeletedEvent<'a> {
    fn get_size(&self) -> usize {
        0
        + self.server.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.server { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum OneOfchanged_object<'a> {
    user_added_event(proto::packet::mod_Event::UserAddedEvent<'a>),
    user_updated_event(proto::packet::mod_Event::UserUpdatedEvent<'a>),
    user_left_event(proto::packet::mod_Event::UserLeftEvent<'a>),
    match_created_event(proto::packet::mod_Event::MatchCreatedEvent<'a>),
    match_updated_event(proto::packet::mod_Event::MatchUpdatedEvent<'a>),
    match_deleted_event(proto::packet::mod_Event::MatchDeletedEvent<'a>),
    qualifier_created_event(proto::packet::mod_Event::QualifierCreatedEvent<'a>),
    qualifier_updated_event(proto::packet::mod_Event::QualifierUpdatedEvent<'a>),
    qualifier_deleted_event(proto::packet::mod_Event::QualifierDeletedEvent<'a>),
    host_added_event(proto::packet::mod_Event::HostAddedEvent<'a>),
    host_deleted_event(proto::packet::mod_Event::HostDeletedEvent<'a>),
    None,
}

impl<'a> Default for OneOfchanged_object<'a> {
    fn default() -> Self {
        OneOfchanged_object::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone, serde::Serialize)]

pub struct Packet<'a> {
    pub id: Cow<'a, str>,
    pub from: Cow<'a, str>,
    pub packet: proto::packet::mod_Packet::OneOfpacket<'a>,
}

impl<'a> MessageRead<'a> for Packet<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.from = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.packet = proto::packet::mod_Packet::OneOfpacket::acknowledgement(r.read_message::<proto::packet::Acknowledgement>(bytes)?),
                Ok(34) => msg.packet = proto::packet::mod_Packet::OneOfpacket::forwarding_packet(Box::new(r.read_message::<proto::packet::ForwardingPacket>(bytes)?)),
                Ok(42) => msg.packet = proto::packet::mod_Packet::OneOfpacket::command(r.read_message::<proto::packet::Command>(bytes)?),
                Ok(50) => msg.packet = proto::packet::mod_Packet::OneOfpacket::push(r.read_message::<proto::packet::Push>(bytes)?),
                Ok(58) => msg.packet = proto::packet::mod_Packet::OneOfpacket::request(r.read_message::<proto::packet::Request>(bytes)?),
                Ok(66) => msg.packet = proto::packet::mod_Packet::OneOfpacket::response(r.read_message::<proto::packet::Response>(bytes)?),
                Ok(74) => msg.packet = proto::packet::mod_Packet::OneOfpacket::event(r.read_message::<proto::packet::Event>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Packet<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.from == "" { 0 } else { 1 + sizeof_len((&self.from).len()) }
        + match self.packet {
            proto::packet::mod_Packet::OneOfpacket::acknowledgement(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Packet::OneOfpacket::forwarding_packet(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Packet::OneOfpacket::command(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Packet::OneOfpacket::push(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Packet::OneOfpacket::request(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Packet::OneOfpacket::response(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Packet::OneOfpacket::event(ref m) => 1 + sizeof_len((m).get_size()),
            proto::packet::mod_Packet::OneOfpacket::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.from != "" { w.write_with_tag(18, |w| w.write_string(&**&self.from))?; }
        match self.packet {            proto::packet::mod_Packet::OneOfpacket::acknowledgement(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            proto::packet::mod_Packet::OneOfpacket::forwarding_packet(ref m) => { w.write_with_tag(34, |w| w.write_message(&**m))? },
            proto::packet::mod_Packet::OneOfpacket::command(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            proto::packet::mod_Packet::OneOfpacket::push(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            proto::packet::mod_Packet::OneOfpacket::request(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            proto::packet::mod_Packet::OneOfpacket::response(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            proto::packet::mod_Packet::OneOfpacket::event(ref m) => { w.write_with_tag(74, |w| w.write_message(m))? },
            proto::packet::mod_Packet::OneOfpacket::None => {},
    }        Ok(())
    }
}

pub mod mod_Packet {

use super::*;

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum OneOfpacket<'a> {
    acknowledgement(proto::packet::Acknowledgement<'a>),
    forwarding_packet(Box<proto::packet::ForwardingPacket<'a>>),
    command(proto::packet::Command<'a>),
    push(proto::packet::Push<'a>),
    request(proto::packet::Request<'a>),
    response(proto::packet::Response<'a>),
    event(proto::packet::Event<'a>),
    None,
}

impl<'a> Default for OneOfpacket<'a> {
    fn default() -> Self {
        OneOfpacket::None
    }
}

}

