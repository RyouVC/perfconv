//! C2S Chart Format
//!
//! note: This format is TSV-based, with each tab-separated value representing a different field.
use crate::formats::chuni::{AirDirection, ChuniNoteType};

use chumsky::prelude::*;

const DEFAULT_RESOLUTION: u32 = 384;

/// Static mapping between note type strings and ChuniNoteType variants
static NOTE_TYPE_PAIRS: &[(&str, ChuniNoteType)] = &[
    ("DEF", ChuniNoteType::Default),
    ("TAP", ChuniNoteType::Tap),
    ("CHR", ChuniNoteType::ExTap),
    ("HLD", ChuniNoteType::Hold),
    ("HXD", ChuniNoteType::ExHold),
    ("SLD", ChuniNoteType::Slide),
    ("SXD", ChuniNoteType::ExSlide),
    ("SLC", ChuniNoteType::SlideControlPoint),
    ("SXC", ChuniNoteType::ExSlideControlPoint),
    ("FLK", ChuniNoteType::Flick),
    ("AIR", ChuniNoteType::Air),
    ("AUR", ChuniNoteType::AirDirectional(AirDirection::UpRight)),
    ("AUL", ChuniNoteType::AirDirectional(AirDirection::UpLeft)),
    ("ADW", ChuniNoteType::AirDirectional(AirDirection::Down)),
    (
        "ADR",
        ChuniNoteType::AirDirectional(AirDirection::DownRight),
    ),
    ("ADL", ChuniNoteType::AirDirectional(AirDirection::DownLeft)),
    ("AHD", ChuniNoteType::AirHold),
    ("MNE", ChuniNoteType::Mine),
    // Note: AirAction is intentionally omitted as the format is unknown
];

/// Convert a note type string to ChuniNoteType
pub fn string_to_note_type(s: &str) -> Result<ChuniNoteType, String> {
    NOTE_TYPE_PAIRS
        .iter()
        .find(|(key, _)| *key == s)
        .map(|(_, note_type)| note_type.clone())
        .ok_or_else(|| format!("Unknown note type: {}", s))
}

pub fn c2s_note_type_to_string(note_type: &ChuniNoteType) -> String {
    // Find the string representation by reverse lookup in the slice
    for &(string_repr, ref mapped_type) in NOTE_TYPE_PAIRS.iter() {
        if mapped_type == note_type {
            return string_repr.to_string();
        }
    }

    // Handle special cases not in the map
    match note_type {
        ChuniNoteType::AirAction => todo!("AIR-ACTION note format unknown"),
        _ => panic!("Unknown note type: {:?}", note_type),
    }
}

pub struct C2SChart {
    pub metadata: C2SMetadata,
    pub notes: Vec<Note>,
}

pub struct C2SMetadata {
    pub version: String,
    /// Song ID, unused in C2S, is declared in Music.xml instead
    pub music: u32,
    /// Sequence ID, unused in C2S, is declared in Music.xml instead
    pub sequence_id: u32,
    /// Difficulty level, unused in C2S, is declared in Music.xml instead
    // `DIFFICULT`
    pub difficulty: u32,
    /// Level of the chart, unused in C2S, is declared in Music.xml instead
    pub level: u32,
    /// Creator of the chart. Will be displayed in-game.
    pub creator: String,
    /// Default BPM (Beats Per Minute) of the song.
    // `BPM_DEF`
    pub bpm_default: f32,
    /// Metronome definition?
    // `MET_DEF`
    pub metronome_def: Option<String>,
    /// Resolution of the chart, defaults to 384 per measure.
    // `RESOLUTION`
    pub resolution: u32,
    /// Clock offset
    // `CLK_DEF`
    pub clock_default: f32,
    /// PROGJUDGE_BPM
    /// Usually set to 240.000
    pub progjudge_bpm: f32,
    /// PROGJUDGE_AER
    /// Usually set to 0.999
    pub progjudge_aer: f32,
    /// Whether this chart is a Tutorial chart.
    // `TUTORIAL`
    pub tutorial: bool,
    /// BPM changes throughout the chart.
    pub bpm: Vec<Bpm>,

    /// Time signatures throughout the chart.
    pub time_signatures: Vec<TimeSignature>,
    /// Speed changes throughout the chart.
    pub sfl: Vec<Sfl>,
}

pub struct Sfl {
    /// Beginning measure where this speed change takes effect
    pub measure: u32,
    /// Offset within the measure where this speed change takes effect
    pub offset: u32,
    /// Duration of the speed change in ticks/measure
    pub duration: u32,
    /// The speed multiplier for the specified measure
    pub multiplier: f32,
}
pub struct Bpm {
    /// Beginning measure where this BPM change takes effect
    pub measure: u32,
    /// Offset within the measure where this BPM change takes effect
    pub offset: u32,
    /// The BPM value for the specified measure
    pub bpm: f32,
}

pub struct TimeSignature {
    /// Beginning measure where this time signature takes effect
    pub measure: u32,
    /// Offset within the measure where this time signature takes effect
    pub offset: u32,
    /// The numerator of the time signature (e.g., 4 in 4/4)
    pub numerator: u32,
    /// The denominator of the time signature (e.g., 4 in 4/4)
    pub denominator: u32,
}

/// An individual note in a C2S chart
///
/// This struct represents a single note in a C2S chart, including its type, position,
/// and any additional properties.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    /// The type of the note, such as TAP, HLD, SLD, etc.
    pub note_type: ChuniNoteType,
    /// Measure where this note starts
    pub measure: u32,
    /// Offset within the measure where this note starts, in ticks
    pub offset: u32,
    /// Cell position of the note, in 1-16 cells(?)
    pub cell: u32,
    /// Width of the note, in 1-16 cells.
    pub width: u32,
    /// Duration of the note in ticks/measure, if applicable (HLD, SLD, SLC, AHD)
    pub duration: Option<u32>,
    /// The end cell of the note, if applicable (SLD, SLC)
    pub end_cell: Option<u32>,
    /// The end width of the note, if applicable (SLD, SLC)
    pub end_width: Option<u32>,
    /// Target note for air notes (AIR, AUR, AUL, AHD, ADW, ADR, ADL)
    /// This specifies what note the air note "leeches" off of
    pub target_note: Option<String>,
    /// Unknown field for CHR notes (usually "UP", "CE", or "DW")
    pub chr_modifier: Option<String>,
    /// Unknown field for FLK notes (always "L")
    // note: Presumably always "L" because the game assumes the default flick direction is left
    pub flick_modifier: Option<String>,
}
impl Note {
    pub fn from_line(line: &str) -> Result<Self, String> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // todo: port to chumsky
        if parts.is_empty() {
            return Err("Empty line".to_string());
        }

        let note_type = string_to_note_type(&parts[0].to_uppercase())?;

        if parts.len() < 5 {
            return Err("Not enough fields".to_string());
        }

        let measure = parts[1].parse::<u32>().map_err(|e| e.to_string())?;
        let offset = parts[2].parse::<u32>().map_err(|e| e.to_string())?;
        let cell = parts[3].parse::<u32>().map_err(|e| e.to_string())?;
        let width = parts[4].parse::<u32>().map_err(|e| e.to_string())?;

        let mut duration = None;
        let mut end_cell = None;
        let mut end_width = None;
        let mut target_note = None;
        let mut chr_modifier = None;
        let mut flick_modifier = None;

        match note_type {
            ChuniNoteType::Hold | ChuniNoteType::ExHold => {
                if parts.len() > 5 {
                    duration = parts[5].parse::<u32>().ok();
                }
            }
            ChuniNoteType::AirHold => {
                if parts.len() > 5 {
                    target_note = parts[5].to_string().into();
                }
                if parts.len() > 6 {
                    duration = parts[6].parse::<u32>().ok();
                }
            }
            ChuniNoteType::Slide
            | ChuniNoteType::ExSlide
            | ChuniNoteType::SlideControlPoint
            | ChuniNoteType::ExSlideControlPoint => {
                if parts.len() > 5 {
                    duration = parts[5].parse::<u32>().ok();
                }
                if parts.len() > 6 {
                    end_cell = parts[6].parse::<u32>().ok();
                }
                if parts.len() > 7 {
                    end_width = parts[7].parse::<u32>().ok();
                }
            }
            ChuniNoteType::ExTap => {
                if parts.len() > 5 {
                    chr_modifier = parts[5].to_string().into();
                }
            }
            ChuniNoteType::Flick => {
                flick_modifier = Some("L".to_string());
            }
            ChuniNoteType::Air | ChuniNoteType::AirDirectional(_) => {
                if parts.len() > 5 {
                    target_note = parts[5].to_string().into();
                }
            }
            ChuniNoteType::AirAction => {
                // TODO: Unknown field structure for AIR-ACTION notes
                // Need to investigate what fields AAC notes use
            }
            _ => {}
        }

        Ok(Note {
            note_type,
            measure,
            offset,
            cell,
            width,
            duration,
            end_cell,
            end_width,
            target_note,
            chr_modifier,
            flick_modifier,
        })
    }
}

impl Note {
    /// Creates a basic TAP note
    pub fn tap(measure: u32, offset: u32, cell: u32, width: u32) -> Self {
        Self {
            note_type: ChuniNoteType::Tap,
            measure,
            offset,
            cell,
            width,
            duration: None,
            end_cell: None,
            end_width: None,
            target_note: None,
            chr_modifier: None,
            flick_modifier: None,
        }
    }

    /// Creates a CHR (ex-note) with modifier
    pub fn chr(measure: u32, offset: u32, cell: u32, width: u32, modifier: String) -> Self {
        Self {
            note_type: ChuniNoteType::ExTap,
            measure,
            offset,
            cell,
            width,
            duration: None,
            end_cell: None,
            end_width: None,
            target_note: None,
            chr_modifier: Some(modifier),
            flick_modifier: None,
        }
    }

    /// Creates a HLD (hold) note
    pub fn hold(measure: u32, offset: u32, cell: u32, width: u32, duration: u32) -> Self {
        Self {
            note_type: ChuniNoteType::Hold,
            measure,
            offset,
            cell,
            width,
            duration: Some(duration),
            end_cell: None,
            end_width: None,
            target_note: None,
            chr_modifier: None,
            flick_modifier: None,
        }
    }

    /// Creates an SLD (slide) note
    pub fn slide(
        measure: u32,
        offset: u32,
        cell: u32,
        width: u32,
        duration: u32,
        end_cell: u32,
        end_width: u32,
    ) -> Self {
        Self {
            note_type: ChuniNoteType::Slide,
            measure,
            offset,
            cell,
            width,
            duration: Some(duration),
            end_cell: Some(end_cell),
            end_width: Some(end_width),
            target_note: None,
            chr_modifier: None,
            flick_modifier: None,
        }
    }

    /// Creates an SLC (slide control point) note
    pub fn slide_control(
        measure: u32,
        offset: u32,
        cell: u32,
        width: u32,
        duration: u32,
        end_cell: u32,
        end_width: u32,
    ) -> Self {
        Self {
            note_type: ChuniNoteType::SlideControlPoint,
            measure,
            offset,
            cell,
            width,
            duration: Some(duration),
            end_cell: Some(end_cell),
            end_width: Some(end_width),
            target_note: None,
            chr_modifier: None,
            flick_modifier: None,
        }
    }

    /// Creates a FLK (flick) note
    pub fn flick(measure: u32, offset: u32, cell: u32, width: u32) -> Self {
        Self {
            note_type: ChuniNoteType::Flick,
            measure,
            offset,
            cell,
            width,
            duration: None,
            end_cell: None,
            end_width: None,
            target_note: None,
            chr_modifier: None,
            flick_modifier: Some("L".to_string()),
        }
    }

    /// Creates an AIR note
    pub fn air(measure: u32, offset: u32, cell: u32, width: u32, target_note: String) -> Self {
        Self {
            note_type: ChuniNoteType::Air,
            measure,
            offset,
            cell,
            width,
            duration: None,
            end_cell: None,
            end_width: None,
            target_note: Some(target_note),
            chr_modifier: None,
            flick_modifier: None,
        }
    }

    /// Creates a directional air note (AUR, AUL, ADW, ADR, ADL)
    pub fn air_directional(
        measure: u32,
        offset: u32,
        cell: u32,
        width: u32,
        target_note: String,
        direction: AirDirection,
    ) -> Self {
        Self {
            note_type: ChuniNoteType::AirDirectional(direction),
            measure,
            offset,
            cell,
            width,
            duration: None,
            end_cell: None,
            end_width: None,
            target_note: Some(target_note),
            chr_modifier: None,
            flick_modifier: None,
        }
    }

    /// Creates an AHD (air hold) note
    pub fn air_hold(
        measure: u32,
        offset: u32,
        cell: u32,
        width: u32,
        target_note: String,
        duration: u32,
    ) -> Self {
        Self {
            note_type: ChuniNoteType::AirHold,
            measure,
            offset,
            cell,
            width,
            duration: Some(duration),
            end_cell: None,
            end_width: None,
            target_note: Some(target_note),
            chr_modifier: None,
            flick_modifier: None,
        }
    }

    /// Creates a MNE (mine) note
    pub fn mine(measure: u32, offset: u32, cell: u32, width: u32) -> Self {
        Self {
            note_type: ChuniNoteType::Mine,
            measure,
            offset,
            cell,
            width,
            duration: None,
            end_cell: None,
            end_width: None,
            target_note: None,
            chr_modifier: None,
            flick_modifier: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const CYAEGHA_NOTES: &str = r#"TAP 8 0 6 4
TAP 8 0 12 4
SLC 8 96 4 4 7 3 4
TAP 8 96 10 4
SLC 8 103 3 4 10 2 4
SLC 8 113 2 4 13 1 4
SLC 8 126 1 4 22 0 4
SLD 8 148 0 4 236 0 4
TAP 8 192 8 4
SLC 8 288 6 4 4 7 4
SLC 8 292 7 4 12 9 4
SLC 8 304 9 4 9 10 4
SLC 8 313 10 4 12 11 4
SLC 8 325 11 4 23 12 4
SLD 8 348 12 4 36 12 4
AHD 9 0 0 4 SLD 192
CHR 9 0 4 8 CE
AIR 9 0 4 8 CHR
AHD 9 0 12 4 SLD 192
TAP 9 288 3 4
TAP 9 288 9 4"#;

    #[test]
    fn test_c2s_note_parsing_1080() {
        let lines: Vec<&str> = CYAEGHA_NOTES.lines().collect();
        let mut notes = Vec::new();

        for line in lines {
            match Note::from_line(line) {
                Ok(note) => {
                    println!("{:?}", note);
                    notes.push(note);
                }
                Err(e) => panic!("Failed to parse note: {}", e),
            }
        }

        // assert_eq!(notes.len(), 20);
        assert_eq!(notes[0].note_type, ChuniNoteType::Tap);
        assert_eq!(notes[1].note_type, ChuniNoteType::Tap);
        assert_eq!(notes[2].note_type, ChuniNoteType::SlideControlPoint);
        assert_eq!(notes[3].note_type, ChuniNoteType::Tap);
        assert_eq!(notes[4].note_type, ChuniNoteType::SlideControlPoint);
        assert_eq!(notes[5].note_type, ChuniNoteType::SlideControlPoint);
        assert_eq!(notes[6].note_type, ChuniNoteType::SlideControlPoint);
        assert_eq!(notes[7].note_type, ChuniNoteType::Slide);
        assert_eq!(notes[8].note_type, ChuniNoteType::Tap);
        assert_eq!(notes[9].note_type, ChuniNoteType::SlideControlPoint);
    }

    #[test]
    fn test_chronomia_adv_1130() {
        let lines: Vec<&str> =
            include_str!("../../../test/chuni/c2s/chronomia_advanced.notesonly.c2s")
                .lines()
                .collect();
        let mut notes = Vec::new();

        for line in lines {
            match Note::from_line(line) {
                Ok(note) => {
                    println!("{:?}", note);
                    notes.push(note);
                }
                Err(e) => panic!("Failed to parse note: {}", e),
            }
        }
    }
}
