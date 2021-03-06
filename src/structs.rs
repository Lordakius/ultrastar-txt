use std::collections::HashMap;
use std::path::PathBuf;

/// Describes the Header of an Ultrastar Song
#[derive(PartialEq, Clone, Debug)]
pub struct Header {
    // mandatory data from headers
    /// the artist of the song
    pub artist: String,
    /// the title of the song
    pub title: String,
    /// the beats per minute of the song
    pub bpm: f32,
    /// the path to the music file
    pub audio_path: std::string::String,

    // optional data from headers
    /// the gap between the start of the audio file and the first note in milliseconds
    pub gap: Option<f32>,
    /// the path to the cover file of the song
    pub cover_path: Option<String>,
    /// the path to the background file of the song
    pub background_path: Option<String>,
    /// the path to the video file of the song
    pub video_path: Option<String>,
    /// the time offset of the video file to the audio file
    pub video_gap: Option<f32>,
    /// the genre of the song
    pub genre: Option<String>,
    /// the edition/category of the song
    pub edition: Option<String>,
    /// the language the song is in
    pub language: Option<String>,
    /// the year the song is from
    pub year: Option<u32>,

    /* header fields todo
    // these are header fields parsed by ultrastar deluxe
    // they might be added if the need arises or my understanding of them grows
    pub creator: Option<String>,  // CREATOR
    pub start: Option<f32>,       // START
    pub end: Option<i32>,         // END
    pub resolution: Option<i32>,  // RESOLUTION
    pub notes_gap: Option<i32>,   // NOTESGAP
    pub encoding: Option<String>, // ENCODING
    pub preview_start: Option<i32>, // PREVIEWSTART
    pub medley_start_beat: Option<i32>, // MEDLEYSTARTBEAT
    pub medley_end_beat: Option<i32>,   // MEDLEYENDBEAT
    pub calc_medley: Option<Bool>,      // CALCMEDLEY
    pub duet_singer_p1: Option<String>, // DUETSINGERP1 / P1
    pub duet_singer_p2: Option<String>, // DUETSINGERP2 / P2:
    */
    /// is the timing format of the song relative
    pub relative: Option<bool>,
    /// a hashmap that contains all tags that are unknown to the parser
    pub unknown: Option<HashMap<String, String>>,
}

/// Describes an Ultrastar song as the combination of its Header and its Lines
#[derive(PartialEq, Clone, Debug)]
pub struct TXTSong {
    /// the header of the song
    pub header: Header,
    /// the lines of the song
    pub lines: Vec<Line>,
}

/// Describes the different types of notes the parser might encounter
#[derive(PartialEq, Clone, Debug)]
pub enum Note {
    /// a regular note
    Regular {
        /// start of the note
        start: i32,
        /// duration of the note
        duration: i32,
        /// pitch of the note (in semitones with C2 being 0)
        pitch: i32,
        /// text or syllable of the note
        text: String,
    },
    /// a golden note (2x points)
    Golden {
        /// start of the note
        start: i32,
        /// duration of the note
        duration: i32,
        /// pitch of the note (in semitones with C2 being 0)
        pitch: i32,
        /// text or syllable of the note
        text: String,
    },
    /// a freestyle note (note that does not award points)
    Freestyle {
        /// start of the note
        start: i32,
        /// duration of the note
        duration: i32,
        /// pitch of the note (in semitones with C2 being 0)
        pitch: i32, //pitch might not be needed but not including it might lose data from orig file
        /// text or syllable of the note
        text: String,
    },
    /// player change indicator for duet mode
    PlayerChange {
        /// player to change to
        /// 1 = Player1
        /// 2 = Player2
        /// 3 = Both
        player: i32,
    },
}

impl Note {
    /// returns the start value of the note
    pub fn start(&self) -> Option<i32> {
        match *self {
            Note::Regular { start, .. }
            | Note::Golden { start, .. }
            | Note::Freestyle { start, .. } => Some(start),
            Note::PlayerChange { .. } => None,
        }
    }

    /// returns the duration value of the note
    pub fn duration(&self) -> Option<i32> {
        match *self {
            Note::Regular { duration, .. }
            | Note::Golden { duration, .. }
            | Note::Freestyle { duration, .. } => Some(duration),
            Note::PlayerChange { .. } => None,
        }
    }

    /// returns the pitch value of the note
    pub fn pitch(&self) -> Option<i32> {
        match *self {
            Note::Regular { pitch, .. }
            | Note::Golden { pitch, .. }
            | Note::Freestyle { pitch, .. } => Some(pitch),
            Note::PlayerChange { .. } => None,
        }
    }

    /// returns a refernece to the text of the note
    pub fn text(&self) -> Option<&str> {
        match *self {
            Note::Regular { ref text, .. }
            | Note::Golden { ref text, .. }
            | Note::Freestyle { ref text, .. } => Some(text),
            Note::PlayerChange { .. } => None,
        }
    }

    /// returns player change number for duett mode
    pub fn player(&self) -> Option<i32> {
        match *self {
            Note::PlayerChange { player, .. } => Some(player),
            Note::Regular { .. } | Note::Golden { .. } | Note::Freestyle { .. } => None,
        }
    }
}

/// Describes a line or sentence that is made up of notes their syllables
#[derive(PartialEq, Clone, Debug)]
pub struct Line {
    /// the start of the line in beats
    pub start: i32,
    /// the second value needed for relative timing
    pub rel: Option<i32>,
    /// the notes the line contains
    pub notes: Vec<Note>,
}
