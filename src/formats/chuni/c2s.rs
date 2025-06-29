//! C2S Chart Format
//!
//! note: This format is TSV-based, with each tab-separated value representing a different field.
use crate::formats::chuni::{AirDirection, ChuniNoteType};

// Special Thanks:
// - Yukopi, for composing [*Kyoufuu All Back*](https://youtu.be/D6DVTLvOupE)
// - アミノハバキリ, for charting the Expert-level version in prod
// - SEGA for actually deciding to put this song in-game for LUMINOUS PLUS
//
// This chart effectively served as our "Rosetta Stone" for decoding the
// CHUNITHM NEW (v1.13.00+) C2S format structure.

// todo: implement end tags

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
    ("AHX", ChuniNoteType::AirHoldGround), // AIR-Hold with green ground bar (hybrid air/ground)?
    ("ALD", ChuniNoteType::AirSlide),
    ("ASC", ChuniNoteType::AirSlideControlPoint),
    ("MNE", ChuniNoteType::Mine),
    // Note: ASD is handled specially as a wrapper format, not a standalone note type
];

/// Convert a note type string to ChuniNoteType
pub fn string_to_note_type(s: &str) -> Result<ChuniNoteType, String> {
    NOTE_TYPE_PAIRS
        .iter()
        .find(|(key, _)| *key == s)
        .map(|(_, note_type)| note_type.clone())
        .or_else(|| {
            // For unknown types, wrap them in Unknown variant
            Some(ChuniNoteType::Unknown(s.to_string()))
        })
        .ok_or_else(|| format!("Failed to create note type for: {}", s))
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
        ChuniNoteType::Unknown(s) => s.clone(),
        _ => panic!("Unknown note type: {:?}", note_type),
    }
}

pub struct C2SChart {
    pub metadata: C2SMetadata,
    pub notes: Vec<Note>,
}

pub struct C2SMetadata {
    pub version: [String; 2],
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
    pub bpm_default: [f32; 4],
    /// Metronome definition?
    // `MET_DEF`
    pub metronome_def: Option<[u32; 4]>,
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

/// Information about a note that was wrapped in ASD/ASC format
/// Both ASD and ASC are wrapper formats that can contain any note type
#[derive(Debug, Clone, PartialEq)]
pub struct WrappedNoteInfo {
    /// The original format type ("ASD" or "ASC")
    pub original_format: String,
    /// The wrapped note type string (e.g., "CHR", "SLD", "TAP", "ASC")
    pub wrapped_type: String,
    /// First parameter from ASD format (usually 5.0)
    pub param1: f32,
    /// Second parameter from ASD format (usually 5.0)
    pub param2: f32,
    /// Third parameter from ASD format (usually "DEF")
    pub param3: String,
}

/// An individual note in a C2S chart
///
/// This struct represents a single note in a C2S chart, including its type, position,
/// and any additional properties.
#[derive(Debug, Clone, PartialEq)]
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
    /// The end cell of the note, if applicable (SLD, SLC, ALD, ASC)
    /// For air slides, this can be a floating point value
    pub end_cell: Option<f32>,
    /// The end width of the note, if applicable (SLD, SLC, ALD, ASC)
    /// For air slides, this can be a floating point value
    pub end_width: Option<f32>,
    /// Target note for air notes (AIR, AUR, AUL, AHD, ADW, ADR, ADL)
    /// This specifies what note the air note "leeches" off of
    pub target_note: Option<String>,
    /// Unknown field for CHR notes (usually "UP", "CE", or "DW")
    pub chr_modifier: Option<String>,
    /// Unknown field for FLK notes (always "L")
    // note: Presumably always "L" because the game assumes the default flick direction is left
    pub flick_modifier: Option<String>,
    /// Information about the wrapped note if this was parsed from ASD/ASC format
    pub wrapped_note_info: Option<WrappedNoteInfo>,
}

impl Note {
    pub fn from_line(line: &str) -> Result<Self, String> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            return Err("Empty line".to_string());
        }

        // Handle ASD and ASC notes (both are wrapper formats with 12 fields) by converting them to regular notes
        if (parts[0].to_uppercase() == "ASD" || parts[0].to_uppercase() == "ASC")
            && parts.len() == 12
        {
            return Note::from_asd_line(line);
        }

        let note_type = string_to_note_type(&parts[0].to_uppercase())?;

        if parts.len() < 5 {
            return Err(format!(
                "Not enough fields: expected at least 5, got {}. Line: '{}'",
                parts.len(),
                line
            ));
        }

        let measure = parts[1]
            .parse::<u32>()
            .map_err(|e| format!("Invalid measure '{}': {}", parts[1], e))?;
        let offset = parts[2]
            .parse::<u32>()
            .map_err(|e| format!("Invalid offset '{}': {}", parts[2], e))?;
        let cell = parts[3]
            .parse::<u32>()
            .map_err(|e| format!("Invalid cell '{}': {}", parts[3], e))?;
        let width = parts[4]
            .parse::<u32>()
            .map_err(|e| format!("Invalid width '{}': {}", parts[4], e))?;

        let mut duration = None;
        let mut end_cell = None;
        let mut end_width = None;
        let mut target_note = None;
        let mut chr_modifier = None;
        let mut flick_modifier = None;
        let mut wrapped_note_info = None;

        // Parse additional fields based on note type
        match note_type {
            ChuniNoteType::Hold | ChuniNoteType::ExHold => {
                if parts.len() > 5 {
                    duration = Some(
                        parts[5]
                            .parse::<u32>()
                            .map_err(|e| format!("Invalid hold duration '{}': {}", parts[5], e))?,
                    );
                }
            }
            ChuniNoteType::AirHold => {
                if parts.len() > 5 {
                    target_note = Some(parts[5].to_string());
                }
                if parts.len() > 6 {
                    duration =
                        Some(parts[6].parse::<u32>().map_err(|e| {
                            format!("Invalid air hold duration '{}': {}", parts[6], e)
                        })?);
                }
            }
            ChuniNoteType::Slide
            | ChuniNoteType::ExSlide
            | ChuniNoteType::SlideControlPoint
            | ChuniNoteType::ExSlideControlPoint => {
                if parts.len() > 5 {
                    duration =
                        Some(parts[5].parse::<u32>().map_err(|e| {
                            format!("Invalid slide duration '{}': {}", parts[5], e)
                        })?);
                }
                if parts.len() > 6 {
                    end_cell =
                        Some(parts[6].parse::<f32>().map_err(|e| {
                            format!("Invalid slide end_cell '{}': {}", parts[6], e)
                        })?);
                }
                if parts.len() > 7 {
                    end_width =
                        Some(parts[7].parse::<f32>().map_err(|e| {
                            format!("Invalid slide end_width '{}': {}", parts[7], e)
                        })?);
                }
            }
            ChuniNoteType::ExTap => {
                if parts.len() > 5 {
                    chr_modifier = Some(parts[5].to_string());
                }
            }
            ChuniNoteType::Flick => {
                flick_modifier = Some("L".to_string());
            }
            ChuniNoteType::Air | ChuniNoteType::AirDirectional(_) => {
                if parts.len() > 5 {
                    target_note = Some(parts[5].to_string());
                }
            }
            ChuniNoteType::AirSlide | ChuniNoteType::AirSlideControlPoint => {
                // Assume ALD/ASC behave like regular slides but in air sensor region
                if parts.len() > 5 {
                    duration = Some(parts[5].parse::<u32>().map_err(|e| {
                        format!("Invalid air slide duration '{}': {}", parts[5], e)
                    })?);
                }
                if parts.len() > 6 {
                    end_cell = Some(parts[6].parse::<f32>().map_err(|e| {
                        format!("Invalid air slide end_cell '{}': {}", parts[6], e)
                    })?);
                }
                if parts.len() > 7 {
                    end_width = Some(parts[7].parse::<f32>().map_err(|e| {
                        format!("Invalid air slide end_width '{}': {}", parts[7], e)
                    })?);
                }
            }
            ChuniNoteType::AirHoldGround => {
                // AHX - AIR-Hold with green ground bar (hybrid air/ground hold note)?
                // Format appears to be: AHX [measure] [tick] [cell] [width] [target_note] [duration] [modifier]
                if parts.len() > 5 {
                    target_note = Some(parts[5].to_string());
                }
                if parts.len() > 6 {
                    duration = Some(parts[6].parse::<u32>().map_err(|e| {
                        format!("Invalid air hold ground duration '{}': {}", parts[6], e)
                    })?);
                }
                // parts[7] appears to be a modifier (usually "DEF")
            }
            ChuniNoteType::Unknown(_) => {
                // For unknown types, we don't know the field structure
                // Just parse basic fields and ignore additional ones
            }
            _ => {}
        }

        // If the note is of type ASD or ASC (both are wrapper formats), parse the additional wrapped note information
        if (parts[0].to_uppercase() == "ASD" || parts[0].to_uppercase() == "ASC")
            && parts.len() == 12
        {
            wrapped_note_info = Some(WrappedNoteInfo {
                original_format: parts[0].to_string(),
                wrapped_type: parts[5].to_string(),
                param1: parts[6]
                    .parse::<f32>()
                    .map_err(|e| format!("Invalid param1 for ASD/ASC note '{}': {}", line, e))?,
                param2: parts[7]
                    .parse::<f32>()
                    .map_err(|e| format!("Invalid param2 for ASD/ASC note '{}': {}", line, e))?,
                param3: parts[11].to_string(),
            });
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
            wrapped_note_info,
        })
    }

    /// Converts an ASD (Air Special Data) or ASC (Air Slide Control) wrapper line into a regular Note
    /// Both ASD and ASC use the same 12-field wrapper format:
    /// ASD/ASC measure tick cell width type param1 duration end_cell end_width param2 param3
    pub fn from_asd_line(line: &str) -> Result<Self, String> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() != 12 {
            return Err(format!(
                "ASD/ASC line must have exactly 12 fields, got {}. Line: '{}'",
                parts.len(),
                line
            ));
        }

        let note_type_prefix = parts[0].to_uppercase();
        let measure = parts[1]
            .parse::<u32>()
            .map_err(|_| format!("Invalid measure: '{}'", parts[1]))?;
        let offset = parts[2]
            .parse::<u32>()
            .map_err(|_| format!("Invalid offset: '{}'", parts[2]))?;
        let cell = parts[3]
            .parse::<u32>()
            .map_err(|_| format!("Invalid cell: '{}'", parts[3]))?;
        let width = parts[4]
            .parse::<u32>()
            .map_err(|_| format!("Invalid width: '{}'", parts[4]))?;

        let wrapped_type = parts[5].to_string();
        let param1 = parts[6]
            .parse::<f32>()
            .map_err(|_| format!("Invalid param1: '{}'", parts[6]))?;
        let duration = parts[7]
            .parse::<u32>()
            .map_err(|_| format!("Invalid duration: '{}'", parts[7]))?;
        let end_cell = parts[8]
            .parse::<u32>()
            .map_err(|_| format!("Invalid end_cell: '{}'", parts[8]))?;
        let end_width = parts[9]
            .parse::<u32>()
            .map_err(|_| format!("Invalid end_width: '{}'", parts[9]))?;
        let param2 = parts[10]
            .parse::<f32>()
            .map_err(|_| format!("Invalid param2: '{}'", parts[10]))?;
        let param3 = parts[11].to_string();

        // Determine the note type based on the prefix and wrapped type
        let note_type = if note_type_prefix == "ASD" {
            // For ASD notes, use the wrapped note type
            string_to_note_type(&wrapped_type.to_uppercase())?
        } else if note_type_prefix == "ASC" {
            // For ASC notes, also use the wrapped note type
            // ASC can wrap any note type, not just AirSlideControlPoint
            string_to_note_type(&wrapped_type.to_uppercase())?
        } else {
            return Err(format!(
                "Invalid note type prefix for ASD/ASC format: {}",
                note_type_prefix
            ));
        };

        // For ASD/ASC notes, we preserve the wrapped note type but with ASD timing/positioning
        Ok(Note {
            note_type,
            measure,
            offset,
            cell,
            width,
            duration: Some(duration),
            end_cell: Some(end_cell as f32),
            end_width: Some(end_width as f32),
            target_note: None,
            chr_modifier: None,
            flick_modifier: None,
            wrapped_note_info: Some(WrappedNoteInfo {
                original_format: note_type_prefix,
                wrapped_type,
                param1,
                param2,
                param3,
            }),
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
            wrapped_note_info: None,
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
            wrapped_note_info: None,
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
            wrapped_note_info: None,
        }
    }

    /// Creates an SLD (slide) note
    pub fn slide(
        measure: u32,
        offset: u32,
        cell: u32,
        width: u32,
        duration: u32,
        end_cell: f32,
        end_width: f32,
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
            wrapped_note_info: None,
        }
    }

    /// Creates an SLC (slide control point) note
    pub fn slide_control(
        measure: u32,
        offset: u32,
        cell: u32,
        width: u32,
        duration: u32,
        end_cell: f32,
        end_width: f32,
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
            wrapped_note_info: None,
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
            wrapped_note_info: None,
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
            wrapped_note_info: None,
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
            wrapped_note_info: None,
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
            wrapped_note_info: None,
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
            wrapped_note_info: None,
        }
    }

    /// Returns true if this note was parsed from ASD or ASC with ASD format
    pub fn is_wrapped_note(&self) -> bool {
        self.wrapped_note_info.is_some()
    }

    /// Returns true if this note is an air action or a crush (ALD+NON)
    pub fn is_air_action(&self) -> bool {
        matches!(
            self.note_type,
            ChuniNoteType::AirSlide | ChuniNoteType::AirSlideControlPoint
        ) && self
            .wrapped_note_info
            .as_ref()
            .map(|info| info.param3 == "NON")
            .unwrap_or(false)
    }

    /// Returns the original format if this was a wrapped note ("ASD" or "ASC")
    pub fn get_original_format(&self) -> Option<&str> {
        self.wrapped_note_info
            .as_ref()
            .map(|info| info.original_format.as_str())
    }

    /// Returns the wrapped note type if this was a wrapped note
    pub fn get_wrapped_type(&self) -> Option<&str> {
        self.wrapped_note_info
            .as_ref()
            .map(|info| info.wrapped_type.as_str())
    }
}

impl C2SChart {
    /// Parse a complete C2S chart from a string containing both metadata and notes
    pub fn from_string(content: &str) -> Result<Self, String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut metadata = C2SMetadata::default();
        let mut notes = Vec::new();
        let mut in_notes_section = false;

        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            // Parse metadata
            match parts[0] {
                "VERSION" => {
                    if parts.len() >= 3 {
                        metadata.version = [parts[1].to_string(), parts[2].to_string()];
                    }
                }
                "MUSIC" => {
                    if parts.len() >= 2 {
                        metadata.music = parts[1].parse().unwrap_or(0);
                    }
                }
                "SEQUENCEID" => {
                    if parts.len() >= 2 {
                        metadata.sequence_id = parts[1].parse().unwrap_or(0);
                    }
                }
                "DIFFICULT" => {
                    if parts.len() >= 2 {
                        metadata.difficulty = parts[1].parse().unwrap_or(0);
                    }
                }
                "LEVEL" => {
                    if parts.len() >= 2 {
                        metadata.level = parts[1].parse::<f32>().unwrap_or(0.0) as u32;
                    }
                }
                "CREATOR" => {
                    if parts.len() >= 2 {
                        metadata.creator = parts[1..].join(" ");
                    }
                }
                "BPM_DEF" => {
                    if parts.len() >= 5 {
                        metadata.bpm_default = [
                            parts[1].parse().unwrap_or(120.0),
                            parts[2].parse().unwrap_or(120.0),
                            parts[3].parse().unwrap_or(120.0),
                            parts[4].parse().unwrap_or(120.0),
                        ];
                    }
                }
                "MET_DEF" => {
                    if parts.len() >= 3 {
                        metadata.metronome_def = Some([
                            parts[1].parse().unwrap_or(4),
                            parts[2].parse().unwrap_or(4),
                            0,
                            0,
                        ]);
                    }
                }
                "RESOLUTION" => {
                    if parts.len() >= 2 {
                        metadata.resolution = parts[1].parse().unwrap_or(384);
                    }
                }
                "CLK_DEF" => {
                    if parts.len() >= 2 {
                        metadata.clock_default = parts[1].parse().unwrap_or(384.0);
                    }
                }
                "PROGJUDGE_BPM" => {
                    if parts.len() >= 2 {
                        metadata.progjudge_bpm = parts[1].parse().unwrap_or(240.0);
                    }
                }
                "PROGJUDGE_AER" => {
                    if parts.len() >= 2 {
                        metadata.progjudge_aer = parts[1].parse().unwrap_or(0.999);
                    }
                }
                "TUTORIAL" => {
                    if parts.len() >= 2 {
                        metadata.tutorial = parts[1] == "1";
                    }
                }
                "BPM" => {
                    if parts.len() >= 4 {
                        metadata.bpm.push(Bpm {
                            measure: parts[1].parse().unwrap_or(0),
                            offset: parts[2].parse().unwrap_or(0),
                            bpm: parts[3].parse().unwrap_or(120.0),
                        });
                    }
                }
                "MET" => {
                    if parts.len() >= 5 {
                        metadata.time_signatures.push(TimeSignature {
                            measure: parts[1].parse().unwrap_or(0),
                            offset: parts[2].parse().unwrap_or(0),
                            numerator: parts[3].parse().unwrap_or(4),
                            denominator: parts[4].parse().unwrap_or(4),
                        });
                    }
                }
                "SFL" => {
                    if parts.len() >= 5 {
                        metadata.sfl.push(Sfl {
                            measure: parts[1].parse().unwrap_or(0),
                            offset: parts[2].parse().unwrap_or(0),
                            duration: parts[3].parse().unwrap_or(0),
                            multiplier: parts[4].parse().unwrap_or(1.0),
                        });
                    }
                }
                // If it's not a metadata field, try to parse it as a note
                _ => {
                    in_notes_section = true;
                    match Note::from_line(line) {
                        Ok(note) => notes.push(note),
                        Err(_) => {
                            // Skip lines that can't be parsed as notes (might be comments or other metadata)
                        }
                    }
                }
            }
        }

        Ok(C2SChart { metadata, notes })
    }
}

impl Default for C2SMetadata {
    fn default() -> Self {
        Self {
            version: ["1.00.00".to_string(), "1.00.00".to_string()],
            music: 0,
            sequence_id: 0,
            difficulty: 0,
            level: 0,
            creator: "Unknown".to_string(),
            bpm_default: [120.0, 120.0, 120.0, 120.0],
            metronome_def: Some([4, 4, 0, 0]),
            resolution: 384,
            clock_default: 384.0,
            progjudge_bpm: 240.0,
            progjudge_aer: 0.999,
            tutorial: false,
            bpm: Vec::new(),
            time_signatures: Vec::new(),
            sfl: Vec::new(),
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
            println!("Parsing line: {}", line);
            match Note::from_line(line) {
                Ok(note) => {
                    println!("{:?}", note);
                    notes.push(note);
                }
                Err(e) => panic!("Failed to parse note: {}", e),
            }
        }
    }

    // #[test]
    // fn test_chronomia_mas_1130() {
    //     let lines: Vec<&str> =
    //         include_str!("../../../test/chuni/c2s/chronomia_master.notesonly.c2s")
    //             .lines()
    //             .collect();
    //     let mut notes = Vec::new();

    //     for line in lines {
    //         println!("Parsing line: {}", line);
    //         match Note::from_line(line) {
    //             Ok(note) => {
    //                 println!("{:?}", note);
    //                 notes.push(note);
    //             }
    //             Err(e) => panic!("Failed to parse note: {}", e),
    //         }
    //     }
    // }

    // #[test]
    // fn test_kyoufu_expert_1130() {
    //     let lines: Vec<&str> = include_str!("../../../test/chuni/c2s/kyoufu_expert.notesonly.c2s")
    //         .lines()
    //         .collect();
    //     let mut notes = Vec::new();

    //     for line in lines {
    //         if line.trim().is_empty() {
    //             continue;
    //         }
    //         println!("Parsing line: {}", line);
    //         match Note::from_line(line) {
    //             Ok(note) => {
    //                 println!("{:?}", note);
    //                 notes.push(note);
    //             }
    //             Err(e) => {
    //                 println!("Failed to parse note: {} (line: {})", e, line);
    //                 // Don't panic, just continue to see all unknown types
    //             }
    //         }
    //     }

    //     println!("Total notes parsed: {}", notes.len());
    // }

    #[test]
    fn test_asd_wrapped_note_info() {
        // Test ASD note preservation
        let asd_line = "ASD 12 0 0 6 CHR 5.0 384 0 3 5.0 DEF";
        let note = Note::from_line(asd_line).unwrap();

        assert_eq!(note.note_type, ChuniNoteType::ExTap);
        assert_eq!(note.measure, 12);
        assert_eq!(note.offset, 0);
        assert_eq!(note.duration, Some(384));
        assert_eq!(note.end_cell, Some(0.0));
        assert_eq!(note.end_width, Some(3.0));

        // Check that ASD information is preserved
        assert!(note.wrapped_note_info.is_some());
        let wrapped_info = note.wrapped_note_info.unwrap();
        assert_eq!(wrapped_info.original_format, "ASD");
        assert_eq!(wrapped_info.wrapped_type, "CHR");
        assert_eq!(wrapped_info.param1, 5.0);
        assert_eq!(wrapped_info.param2, 5.0);
        assert_eq!(wrapped_info.param3, "DEF");

        // Test ASC note with wrapper format - wrapping SLD
        let asc_line = "ASC 2 96 12 4 SLD 5.0 12 6 4 5.0 DEF";
        let note = Note::from_line(asc_line).unwrap();

        assert_eq!(note.note_type, ChuniNoteType::Slide);
        assert!(note.wrapped_note_info.is_some());
        let wrapped_info = note.wrapped_note_info.unwrap();
        assert_eq!(wrapped_info.original_format, "ASC");
        assert_eq!(wrapped_info.wrapped_type, "SLD");

        // Test ASC note wrapping another ASC (yes, this happens!)
        let asc_asc_line = "ASC 5 336 9 4 ASC 5.0 22 9 5 5.0 DEF";
        let note = Note::from_line(asc_asc_line).unwrap();

        assert_eq!(note.note_type, ChuniNoteType::AirSlideControlPoint);
        assert!(note.wrapped_note_info.is_some());
        let wrapped_info = note.wrapped_note_info.unwrap();
        assert_eq!(wrapped_info.original_format, "ASC");
        assert_eq!(wrapped_info.wrapped_type, "ASC");

        // Test regular note has no wrapped info
        let regular_line = "TAP 8 0 6 4";
        let note = Note::from_line(regular_line).unwrap();
        assert_eq!(note.note_type, ChuniNoteType::Tap);
        assert!(note.wrapped_note_info.is_none());
    }

    #[test]
    fn test_full_chart_with_metadata() {
        let chart_content = r#"VERSION	1.12.00	1.12.00
MUSIC	0
SEQUENCEID	0
DIFFICULT	00
LEVEL	0.0
CREATOR	みぞれヤナギ
BPM_DEF	135.000	135.000	135.000	135.000
MET_DEF	4	4
RESOLUTION	384
CLK_DEF	384
PROGJUDGE_BPM	240.000
PROGJUDGE_AER	  0.999
TUTORIAL	0

BPM	0	0	135.000
MET	0	0	4	4

TAP	0	0	8	4
CHR	0	192	4	8	CE
HLD	1	0	0	4	192
SLD	2	0	12	4	192	8	4
AHD	3	0	6	4	TAP	96
AIR	3	0	6	4	TAP
TAP	4	0	0	16"#;

        let chart = C2SChart::from_string(chart_content).unwrap();

        // Test metadata parsing
        assert_eq!(chart.metadata.version[0], "1.12.00");
        assert_eq!(chart.metadata.version[1], "1.12.00");
        assert_eq!(chart.metadata.music, 0);
        assert_eq!(chart.metadata.sequence_id, 0);
        assert_eq!(chart.metadata.difficulty, 0);
        assert_eq!(chart.metadata.level, 0);
        assert_eq!(chart.metadata.creator, "みぞれヤナギ");
        assert_eq!(chart.metadata.bpm_default, [135.0, 135.0, 135.0, 135.0]);
        assert_eq!(chart.metadata.metronome_def, Some([4, 4, 0, 0]));
        assert_eq!(chart.metadata.resolution, 384);
        assert_eq!(chart.metadata.clock_default, 384.0);
        assert_eq!(chart.metadata.progjudge_bpm, 240.0);
        assert_eq!(chart.metadata.progjudge_aer, 0.999);
        assert!(!chart.metadata.tutorial);

        // Test BPM and MET changes
        assert_eq!(chart.metadata.bpm.len(), 1);
        assert_eq!(chart.metadata.bpm[0].measure, 0);
        assert_eq!(chart.metadata.bpm[0].offset, 0);
        assert_eq!(chart.metadata.bpm[0].bpm, 135.0);

        assert_eq!(chart.metadata.time_signatures.len(), 1);
        assert_eq!(chart.metadata.time_signatures[0].measure, 0);
        assert_eq!(chart.metadata.time_signatures[0].offset, 0);
        assert_eq!(chart.metadata.time_signatures[0].numerator, 4);
        assert_eq!(chart.metadata.time_signatures[0].denominator, 4);

        // Test notes parsing
        assert_eq!(chart.notes.len(), 7);
        assert_eq!(chart.notes[0].note_type, ChuniNoteType::Tap);
        assert_eq!(chart.notes[1].note_type, ChuniNoteType::ExTap);
        assert_eq!(chart.notes[2].note_type, ChuniNoteType::Hold);
        assert_eq!(chart.notes[3].note_type, ChuniNoteType::Slide);
        assert_eq!(chart.notes[4].note_type, ChuniNoteType::AirHold);
        assert_eq!(chart.notes[5].note_type, ChuniNoteType::Air);
        assert_eq!(chart.notes[6].note_type, ChuniNoteType::Tap);

        // Test specific note properties
        assert_eq!(chart.notes[1].chr_modifier, Some("CE".to_string()));
        assert_eq!(chart.notes[2].duration, Some(192));
        assert_eq!(chart.notes[3].end_cell, Some(8.0));
        assert_eq!(chart.notes[3].end_width, Some(4.0));
        assert_eq!(chart.notes[4].target_note, Some("TAP".to_string()));
        assert_eq!(chart.notes[4].duration, Some(96));
        assert_eq!(chart.notes[5].target_note, Some("TAP".to_string()));

        // Test the full-width tap note (cell 0, width 16)
        assert_eq!(chart.notes[6].cell, 0);
        assert_eq!(chart.notes[6].width, 16);
    }

    #[test]
    fn test_air_action_and_air_crush_notes() {
        let chart_content = r#"VERSION	1.13.00	1.13.00
MUSIC	2699
SEQUENCEID	3
DIFFICULT	03
LEVEL	13.0
CREATOR	SOMEONE
BPM_DEF	175.000	175.000	175.000	175.000
MET_DEF	4	4
RESOLUTION	384
CLK_DEF	384
PROGJUDGE_BPM	240.000
PROGJUDGE_AER	0.999
TUTORIAL	0

BPM	0	0	175.000
MET	0	0	4	4

TAP	5	0	8	4
AHX	5	96	8	4	SLD	96	DEF
CHR	6	0	4	8	RS
ALD	6	96	4	8	38400	5.0	1	4	8	5.0	NON
ALD	6	192	5	6	6	3.0	1	5	6	3.0	NON
ALD	6	192	6	4	6	2.0	1	6	4	2.0	NON
ALD	6	192	7	2	6	1.0	1	7	2	1.0	NON"#;

        let chart = C2SChart::from_string(chart_content).unwrap();

        // Test metadata for CHUNITHM NEW version
        assert_eq!(chart.metadata.version[0], "1.13.00");
        assert_eq!(chart.metadata.music, 2699);
        assert_eq!(chart.metadata.sequence_id, 3);
        assert_eq!(chart.metadata.difficulty, 3);
        assert_eq!(chart.metadata.level, 13);
        assert_eq!(chart.metadata.creator, "SOMEONE");
        assert_eq!(chart.metadata.bpm_default[0], 175.0);

        // Test notes
        assert_eq!(chart.notes.len(), 7);

        // TAP note
        assert_eq!(chart.notes[0].note_type, ChuniNoteType::Tap);

        // AHX - AIR-Hold with green ground bar
        assert_eq!(chart.notes[1].note_type, ChuniNoteType::AirHoldGround);
        assert_eq!(chart.notes[1].target_note, Some("SLD".to_string()));
        assert_eq!(chart.notes[1].duration, Some(96));

        // CHR - ExTap
        assert_eq!(chart.notes[2].note_type, ChuniNoteType::ExTap);
        assert_eq!(chart.notes[2].chr_modifier, Some("RS".to_string()));

        // ALD+NON - AIR-ACTION (single purple floating bar)
        assert_eq!(chart.notes[3].note_type, ChuniNoteType::AirSlide);
        assert_eq!(chart.notes[3].duration, Some(38400));

        // ALD+NON - AIR CRUSH pattern (3 simultaneous notes creating a geometric shape)
        assert_eq!(chart.notes[4].note_type, ChuniNoteType::AirSlide);
        assert_eq!(chart.notes[5].note_type, ChuniNoteType::AirSlide);
        assert_eq!(chart.notes[6].note_type, ChuniNoteType::AirSlide);

        // All three ALD+NON notes should be at the same timing (measure 6, offset 192)
        assert_eq!(chart.notes[4].measure, 6);
        assert_eq!(chart.notes[4].offset, 192);
        assert_eq!(chart.notes[5].measure, 6);
        assert_eq!(chart.notes[5].offset, 192);
        assert_eq!(chart.notes[6].measure, 6);
        assert_eq!(chart.notes[6].offset, 192);

        // Different cell positions and widths for the geometric pattern
        assert_eq!(chart.notes[4].cell, 5);
        assert_eq!(chart.notes[4].width, 6);
        assert_eq!(chart.notes[5].cell, 6);
        assert_eq!(chart.notes[5].width, 4);
        assert_eq!(chart.notes[6].cell, 7);
        assert_eq!(chart.notes[6].width, 2);
    }

    #[test]
    fn test_comprehensive_metadata_parsing() {
        let chart_content = r#"VERSION	1.13.00	1.13.00
MUSIC	2699
SEQUENCEID	3
DIFFICULT	03
LEVEL	13.2
CREATOR	SOMEONE
BPM_DEF	120.000	120.000	120.000	120.000
MET_DEF	4	4
RESOLUTION	384
CLK_DEF	384
PROGJUDGE_BPM	240.000
PROGJUDGE_AER	0.999
TUTORIAL	0

BPM	0	0	120.000
BPM	4	0	160.000
BPM	8	192	140.000
BPM	16	0	180.000

MET	0	0	4	4
MET	8	0	3	4
MET	12	0	4	4

SFL	0	0	192	1.0
SFL	4	0	384	1.5
SFL	8	0	192	0.8
SFL	12	0	96	2.0

TAP	0	0	8	4
HLD	1	0	0	4	192
CHR	2	0	12	4	UP
SLD	3	0	4	4	192	8	4
FLK	4	0	0	2
AIR	4	0	0	2	FLK
SLC	4	96	8	4	96	12	4
SLD	4	192	12	4	192	4	4
AHD	5	0	4	4	SLD	192
MNE	6	0	2	2
TAP	7	0	14	2
AUR	7	96	14	2	TAP
AUL	7	192	14	2	TAP
ADW	7	288	14	2	TAP
ALD	8	0	6	4	192	10	4	5.0	NON
ASC	8	96	10	4	96	8	4	3.0	DEF
ASD	9	0	0	6	CHR	5.0	384	0	3	5.0	DEF
AHX	10	0	8	4	TAP	192	DEF"#;

        let chart = C2SChart::from_string(chart_content).unwrap();

        // Test basic metadata
        assert_eq!(chart.metadata.version[0], "1.13.00");
        assert_eq!(chart.metadata.version[1], "1.13.00");
        assert_eq!(chart.metadata.music, 2699);
        assert_eq!(chart.metadata.sequence_id, 3);
        assert_eq!(chart.metadata.difficulty, 3);
        assert_eq!(chart.metadata.level, 13); // Should be truncated to integer
        assert_eq!(chart.metadata.creator, "SOMEONE");
        assert_eq!(chart.metadata.bpm_default, [120.0, 120.0, 120.0, 120.0]);
        assert_eq!(chart.metadata.metronome_def, Some([4, 4, 0, 0]));
        assert_eq!(chart.metadata.resolution, 384);
        assert_eq!(chart.metadata.clock_default, 384.0);
        assert_eq!(chart.metadata.progjudge_bpm, 240.0);
        assert_eq!(chart.metadata.progjudge_aer, 0.999);
        assert!(!chart.metadata.tutorial);

        // Test BPM changes
        assert_eq!(chart.metadata.bpm.len(), 4);
        assert_eq!(chart.metadata.bpm[0].measure, 0);
        assert_eq!(chart.metadata.bpm[0].offset, 0);
        assert_eq!(chart.metadata.bpm[0].bpm, 120.0);
        assert_eq!(chart.metadata.bpm[1].measure, 4);
        assert_eq!(chart.metadata.bpm[1].offset, 0);
        assert_eq!(chart.metadata.bpm[1].bpm, 160.0);
        assert_eq!(chart.metadata.bpm[2].measure, 8);
        assert_eq!(chart.metadata.bpm[2].offset, 192);
        assert_eq!(chart.metadata.bpm[2].bpm, 140.0);
        assert_eq!(chart.metadata.bpm[3].measure, 16);
        assert_eq!(chart.metadata.bpm[3].offset, 0);
        assert_eq!(chart.metadata.bpm[3].bpm, 180.0);

        // Test time signature changes
        assert_eq!(chart.metadata.time_signatures.len(), 3);
        assert_eq!(chart.metadata.time_signatures[0].measure, 0);
        assert_eq!(chart.metadata.time_signatures[0].offset, 0);
        assert_eq!(chart.metadata.time_signatures[0].numerator, 4);
        assert_eq!(chart.metadata.time_signatures[0].denominator, 4);
        assert_eq!(chart.metadata.time_signatures[1].measure, 8);
        assert_eq!(chart.metadata.time_signatures[1].offset, 0);
        assert_eq!(chart.metadata.time_signatures[1].numerator, 3);
        assert_eq!(chart.metadata.time_signatures[1].denominator, 4);
        assert_eq!(chart.metadata.time_signatures[2].measure, 12);
        assert_eq!(chart.metadata.time_signatures[2].offset, 0);
        assert_eq!(chart.metadata.time_signatures[2].numerator, 4);
        assert_eq!(chart.metadata.time_signatures[2].denominator, 4);

        // Test speed changes (SFL)
        assert_eq!(chart.metadata.sfl.len(), 4);
        assert_eq!(chart.metadata.sfl[0].measure, 0);
        assert_eq!(chart.metadata.sfl[0].offset, 0);
        assert_eq!(chart.metadata.sfl[0].duration, 192);
        assert_eq!(chart.metadata.sfl[0].multiplier, 1.0);
        assert_eq!(chart.metadata.sfl[1].measure, 4);
        assert_eq!(chart.metadata.sfl[1].offset, 0);
        assert_eq!(chart.metadata.sfl[1].duration, 384);
        assert_eq!(chart.metadata.sfl[1].multiplier, 1.5);
        assert_eq!(chart.metadata.sfl[2].measure, 8);
        assert_eq!(chart.metadata.sfl[2].offset, 0);
        assert_eq!(chart.metadata.sfl[2].duration, 192);
        assert_eq!(chart.metadata.sfl[2].multiplier, 0.8);
        assert_eq!(chart.metadata.sfl[3].measure, 12);
        assert_eq!(chart.metadata.sfl[3].offset, 0);
        assert_eq!(chart.metadata.sfl[3].duration, 96);
        assert_eq!(chart.metadata.sfl[3].multiplier, 2.0);

        // Test comprehensive note parsing
        assert_eq!(chart.notes.len(), 18);

        // Test variety of note types
        assert_eq!(chart.notes[0].note_type, ChuniNoteType::Tap);
        assert_eq!(chart.notes[1].note_type, ChuniNoteType::Hold);
        assert_eq!(chart.notes[2].note_type, ChuniNoteType::ExTap);
        assert_eq!(chart.notes[3].note_type, ChuniNoteType::Slide);
        assert_eq!(chart.notes[4].note_type, ChuniNoteType::Flick);
        assert_eq!(chart.notes[5].note_type, ChuniNoteType::Air);
        assert_eq!(chart.notes[6].note_type, ChuniNoteType::SlideControlPoint);
        assert_eq!(chart.notes[7].note_type, ChuniNoteType::Slide);
        assert_eq!(chart.notes[8].note_type, ChuniNoteType::AirHold);
        assert_eq!(chart.notes[9].note_type, ChuniNoteType::Mine);
        assert_eq!(chart.notes[10].note_type, ChuniNoteType::Tap);
        assert_eq!(
            chart.notes[11].note_type,
            ChuniNoteType::AirDirectional(AirDirection::UpRight)
        );
        assert_eq!(
            chart.notes[12].note_type,
            ChuniNoteType::AirDirectional(AirDirection::UpLeft)
        );
        assert_eq!(
            chart.notes[13].note_type,
            ChuniNoteType::AirDirectional(AirDirection::Down)
        );
        assert_eq!(chart.notes[14].note_type, ChuniNoteType::AirSlide);
        assert_eq!(
            chart.notes[15].note_type,
            ChuniNoteType::AirSlideControlPoint
        );
        assert_eq!(chart.notes[16].note_type, ChuniNoteType::ExTap); // ASD wrapped CHR
        assert_eq!(chart.notes[17].note_type, ChuniNoteType::AirHoldGround);

        // Test specific note properties
        assert_eq!(chart.notes[1].duration, Some(192)); // HLD duration
        assert_eq!(chart.notes[2].chr_modifier, Some("UP".to_string())); // CHR modifier
        assert_eq!(chart.notes[3].end_cell, Some(8.0)); // SLD end position
        assert_eq!(chart.notes[3].end_width, Some(4.0)); // SLD end width
        assert_eq!(chart.notes[4].flick_modifier, Some("L".to_string())); // FLK modifier
        assert_eq!(chart.notes[5].target_note, Some("FLK".to_string())); // AIR target
        assert_eq!(chart.notes[8].target_note, Some("SLD".to_string())); // AHD target
        assert_eq!(chart.notes[8].duration, Some(192)); // AHD duration

        // Test directional air notes target
        assert_eq!(chart.notes[11].target_note, Some("TAP".to_string()));
        assert_eq!(chart.notes[12].target_note, Some("TAP".to_string()));
        assert_eq!(chart.notes[13].target_note, Some("TAP".to_string()));

        // Test ALD (AIR-ACTION) properties
        assert_eq!(chart.notes[14].duration, Some(192));
        assert_eq!(chart.notes[14].end_cell, Some(10.0));
        assert_eq!(chart.notes[14].end_width, Some(4.0));

        // Test ASC (AIR slide control point) properties
        assert_eq!(chart.notes[15].duration, Some(96));
        assert_eq!(chart.notes[15].end_cell, Some(8.0));
        assert_eq!(chart.notes[15].end_width, Some(4.0));

        // Test ASD wrapped note
        assert!(chart.notes[16].wrapped_note_info.is_some());
        let wrapped_info = chart.notes[16].wrapped_note_info.as_ref().unwrap();
        assert_eq!(wrapped_info.original_format, "ASD");
        assert_eq!(wrapped_info.wrapped_type, "CHR");
        assert_eq!(wrapped_info.param1, 5.0);
        assert_eq!(wrapped_info.param2, 5.0);
        assert_eq!(wrapped_info.param3, "DEF");

        // Test AHX (AIR-Hold with green ground bar)
        assert_eq!(chart.notes[17].target_note, Some("TAP".to_string()));
        assert_eq!(chart.notes[17].duration, Some(192));
    }
}
