//! SUS format parser for CHUNITHM sim charts
//!
//! This module provides parsing and data structures for the SUS (Sliding Universal Score) format.
//!
//! SUS is a text-based format used for SUSPlayer and Seaurchin charts, a CHUNITHM simulator.
//!
//! # Builder and Helper API
//!
//! ## SusLine Helper Constructors
//!
//! The `SusLine` enum provides ergonomic constructors for all common variants:
//!
//! ```rust
//! SusLine::metadata("TITLE", "Song Title")
//! SusLine::comment("This is a comment")
//! SusLine::note(Note { lane: 1, tick: 480, note_type: "tap".to_string(), width: 1 })
//! SusLine::tap_notes(0, 1, "14141414")
//! SusLine::hold_notes(0, 1, 2, "14002400")
//! SusLine::slide_notes(0, 3, 1, 2, "14340024")
//! SusLine::directional_notes(0, 1, "14241424")
//! SusLine::bpm_definition("01", 140.0)
//! SusLine::attribute_definition("01", "pr:100, h:1.5")
//! SusLine::hi_speed_definition("01", "0'0:1.0, 0'960:2.0")
//! SusLine::measure_length(0, 4.0)
//! SusLine::bpm_change(0, 0, "01")
//! ```
//!
//! ## SusChart Builder Methods
//!
//! The `SusChart` struct provides builder-style methods for ergonomic chart construction and modification:
//!
//! ```rust
//! let mut chart = SusChart {
//!     metadata: HashMap::new(),
//!     lines: Vec::new(),
//! };
//!
//! chart
//!     .metadata("TITLE", "Test Song")
//!     .note(Note { lane: 1, tick: 480, note_type: "tap".to_string(), width: 1 })
//!     .new_line(SusLine::comment("A comment"));
//! ```
//!
//! - `.new_line(SusLine)` — Add any line (updates metadata map if it's metadata)
//! - `.note(Note)` — Add a note line
//! - `.metadata(key, value)` — Add a metadata line (updates both lines and metadata map)
//! - `.comment(content)` — Add a comment line
//!

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SusLine {
    Metadata {
        key: String,
        value: String,
    },
    Comment {
        content: String,
    },
    Note(Note),
    BpmDefinition {
        id: String,
        bpm: f64,
    },
    AttributeDefinition {
        id: String,
        attributes: String,
    },
    HiSpeedDefinition {
        id: String,
        definition: String,
    },
    MeasureLength {
        measure: u32,
        length: f64,
    },
    BpmChange {
        measure: u32,
        lane: u8,
        data: String,
    },
    TapNotes {
        measure: u32,
        lane: u8,
        data: String,
    },
    HoldNotes {
        measure: u32,
        lane: u8,
        channel: u8,
        data: String,
    },
    SlideNotes {
        measure: u32,
        slide_type: u8,
        lane: u8,
        channel: u8,
        data: String,
    },
    DirectionalNotes {
        measure: u32,
        lane: u8,
        data: String,
    },
    Unknown {
        content: String,
    },
}

/// Represents a parsed SUS chart.
#[derive(Debug)]
pub struct SusChart {
    /// Chart metadata (title, artist, etc.)
    pub metadata: HashMap<String, String>,
    /// Parsed note/event data
    pub lines: Vec<SusLine>,
}

impl SusLine {
    pub fn metadata<K: Into<String>, V: Into<String>>(key: K, value: V) -> Self {
        SusLine::Metadata {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn comment<S: Into<String>>(content: S) -> Self {
        SusLine::Comment {
            content: content.into(),
        }
    }

    pub fn note(note: Note) -> Self {
        SusLine::Note(note)
    }

    pub fn tap_notes(measure: u32, lane: u8, data: impl Into<String>) -> Self {
        SusLine::TapNotes {
            measure,
            lane,
            data: data.into(),
        }
    }

    pub fn hold_notes(measure: u32, lane: u8, channel: u8, data: impl Into<String>) -> Self {
        SusLine::HoldNotes {
            measure,
            lane,
            channel,
            data: data.into(),
        }
    }

    pub fn slide_notes(
        measure: u32,
        slide_type: u8,
        lane: u8,
        channel: u8,
        data: impl Into<String>,
    ) -> Self {
        SusLine::SlideNotes {
            measure,
            slide_type,
            lane,
            channel,
            data: data.into(),
        }
    }

    pub fn directional_notes(measure: u32, lane: u8, data: impl Into<String>) -> Self {
        SusLine::DirectionalNotes {
            measure,
            lane,
            data: data.into(),
        }
    }

    pub fn bpm_definition(id: impl Into<String>, bpm: f64) -> Self {
        SusLine::BpmDefinition { id: id.into(), bpm }
    }

    pub fn attribute_definition(id: impl Into<String>, attributes: impl Into<String>) -> Self {
        SusLine::AttributeDefinition {
            id: id.into(),
            attributes: attributes.into(),
        }
    }

    pub fn hi_speed_definition(id: impl Into<String>, definition: impl Into<String>) -> Self {
        SusLine::HiSpeedDefinition {
            id: id.into(),
            definition: definition.into(),
        }
    }

    pub fn measure_length(measure: u32, length: f64) -> Self {
        SusLine::MeasureLength { measure, length }
    }

    pub fn bpm_change(measure: u32, lane: u8, data: impl Into<String>) -> Self {
        SusLine::BpmChange {
            measure,
            lane,
            data: data.into(),
        }
    }
}

/// Map SUS type digit to ChuniNoteType
fn sus_type_to_chuni_note_type(type_digit: u8, slide_type: Option<u8>) -> ChuniNoteType {
    match slide_type {
        Some(3) | Some(4) => ChuniNoteType::Slide,
        _ => match type_digit {
            1 | 2 | 3 | 4 | 5 | 6 => ChuniNoteType::Tap,
            // Hold: 2xy
            0x20..=0x2F => ChuniNoteType::Hold,
            // Slide: 3xy, 4xy
            0x30..=0x3F | 0x40..=0x4F => ChuniNoteType::Slide,
            // Directional: 5x
            0x50..=0x5F => ChuniNoteType::Flick,
            _ => ChuniNoteType::Unknown(format!("SUS type {:X}", type_digit)),
        },
    }
}

impl SusChart {
    /// Add any SusLine
    pub fn new_line(&mut self, line: SusLine) -> &mut Self {
        if let SusLine::Metadata { ref key, ref value } = line {
            self.metadata.insert(key.clone(), value.clone());
        }
        self.lines.push(line);
        self
    }

    /// Add a Note line
    pub fn note(&mut self, note: Note) -> &mut Self {
        self.lines.push(SusLine::Note(note));
        self
    }

    /// Add a metadata line
    pub fn metadata(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        let key = key.into();
        let value = value.into();
        self.metadata.insert(key.clone(), value.clone());
        self.lines.push(SusLine::Metadata { key, value });
        self
    }

    /// Add a comment line
    pub fn comment(&mut self, content: impl Into<String>) -> &mut Self {
        self.lines.push(SusLine::Comment {
            content: content.into(),
        });
        self
    }
}

/// Represents a single note or event in SUS.
use super::ChuniNoteType;

#[derive(Debug, Clone)]
pub struct Note {
    pub lane: u8,
    pub tick: u32,
    pub note_type: ChuniNoteType,
    pub width: u8,
}

/// Parse a hexadecimal character to its numeric value
fn parse_hex_char(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        'a'..='z' => Some(c as u8 - b'a' + 10),
        'A'..='Z' => Some(c as u8 - b'A' + 10),
        _ => None,
    }
}

/// Parse a 3-digit measure number (with support for hex)
fn parse_measure(s: &str) -> Option<u32> {
    if s.len() != 3 {
        return None;
    }

    let mut result = 0u32;
    for c in s.chars() {
        if let Some(digit) = parse_hex_char(c) {
            result = result * 36 + digit as u32;
        } else {
            return None;
        }
    }
    Some(result)
}

/// Parse a single hex digit lane identifier
fn parse_lane(c: char) -> Option<u8> {
    parse_hex_char(c)
}

/// Parse metadata line starting with #
fn parse_metadata_line(line: &str) -> Option<SusLine> {
    if !line.starts_with('#') {
        return None;
    }

    let content = &line[1..];

    // Find the first space to separate command from value
    if let Some(space_pos) = content.find(' ') {
        let key = content[..space_pos].trim();
        let value = content[space_pos + 1..].trim();

        // Remove quotes if present
        let value = if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
            &value[1..value.len() - 1]
        } else {
            value
        };

        Some(SusLine::Metadata {
            key: key.to_string(),
            value: value.to_string(),
        })
    } else {
        // Handle lines without values (like #NOATTRIBUTE)
        Some(SusLine::Metadata {
            key: content.trim().to_string(),
            value: String::new(),
        })
    }
}

/// Parse chart data line with format: header:data
fn parse_chart_data_line(line: &str) -> Option<SusLine> {
    if !line.contains(':') {
        return None;
    }

    let parts: Vec<&str> = line.splitn(2, ':').collect();
    if parts.len() != 2 {
        return None;
    }

    let header = parts[0].trim().trim_start_matches('#');
    let data = parts[1].trim();

    // Handle special definitions (BPM, ATR, TIL, etc.)
    if header.starts_with("BPM") && header.len() >= 4 {
        let id = &header[3..];
        if let Ok(bpm) = data.parse::<f64>() {
            return Some(SusLine::BpmDefinition {
                id: id.to_string(),
                bpm,
            });
        }
    }

    if header.starts_with("ATR") && header.len() >= 4 {
        let id = &header[3..];
        return Some(SusLine::AttributeDefinition {
            id: id.to_string(),
            attributes: data.to_string(),
        });
    }

    if header.starts_with("TIL") && header.len() >= 4 {
        let id = &header[3..];
        return Some(SusLine::HiSpeedDefinition {
            id: id.to_string(),
            definition: data.to_string(),
        });
    }

    // Handle measure data lines
    if header.len() >= 5 {
        if let Some(measure) = parse_measure(&header[0..3]) {
            let type_and_lane = &header[3..];

            match type_and_lane.chars().next() {
                Some('0') if type_and_lane.len() >= 2 => {
                    // 02 = measure length
                    if type_and_lane == "02" {
                        if let Ok(length) = data.parse::<f64>() {
                            return Some(SusLine::MeasureLength { measure, length });
                        }
                    }
                    // 08 = BPM change
                    else if type_and_lane == "08" {
                        return Some(SusLine::BpmChange {
                            measure,
                            lane: 0,
                            data: data.to_string(),
                        });
                    }
                }
                Some('1') if type_and_lane.len() >= 2 => {
                    // Tap notes (10-16)
                    if let Some(lane) = parse_lane(type_and_lane.chars().nth(1).unwrap_or('0')) {
                        // Always use ChuniNoteType::Tap for tap lines
                        if data.len() >= 2 {
                            let width_digit = data.chars().nth(1).unwrap();
                            let width = width_digit.to_digit(36).unwrap_or(1) as u8;
                            let note = Note {
                                lane,
                                tick: 0,
                                note_type: ChuniNoteType::Tap,
                                width,
                            };
                            return Some(SusLine::Note(note));
                        }
                        return Some(SusLine::TapNotes {
                            measure,
                            lane,
                            data: data.to_string(),
                        });
                    }
                }
                Some('2') if type_and_lane.len() >= 3 => {
                    // Hold notes (2xy)
                    let chars: Vec<char> = type_and_lane.chars().collect();
                    if let (Some(lane), Some(channel)) =
                        (parse_lane(chars[1]), parse_lane(chars[2]))
                    {
                        // Always use ChuniNoteType::Hold for hold lines
                        if data.len() >= 2 {
                            let width_digit = data.chars().nth(1).unwrap();
                            let width = width_digit.to_digit(36).unwrap_or(1) as u8;
                            let note = Note {
                                lane,
                                tick: 0,
                                note_type: ChuniNoteType::Hold,
                                width,
                            };
                            return Some(SusLine::Note(note));
                        }
                        return Some(SusLine::HoldNotes {
                            measure,
                            lane,
                            channel,
                            data: data.to_string(),
                        });
                    }
                }
                Some('3') | Some('4') if type_and_lane.len() >= 3 => {
                    // Slide notes (3xy, 4xy)
                    let chars: Vec<char> = type_and_lane.chars().collect();
                    let slide_type = if chars[0] == '3' { 3 } else { 4 };
                    if let (Some(lane), Some(channel)) =
                        (parse_lane(chars[1]), parse_lane(chars[2]))
                    {
                        // Always use ChuniNoteType::Slide for slide lines
                        if data.len() >= 2 {
                            let width_digit = data.chars().nth(1).unwrap();
                            let width = width_digit.to_digit(36).unwrap_or(1) as u8;
                            let note = Note {
                                lane,
                                tick: 0,
                                note_type: ChuniNoteType::Slide,
                                width,
                            };
                            return Some(SusLine::Note(note));
                        }
                        return Some(SusLine::SlideNotes {
                            measure,
                            slide_type,
                            lane,
                            channel,
                            data: data.to_string(),
                        });
                    }
                }
                Some('5') if type_and_lane.len() >= 2 => {
                    // Directional notes (5x)
                    if let Some(lane) = parse_lane(type_and_lane.chars().nth(1).unwrap_or('0')) {
                        if data.len() >= 2 {
                            let type_digit = data.chars().nth(0).unwrap();
                            let width_digit = data.chars().nth(1).unwrap();
                            let type_digit = type_digit.to_digit(36).unwrap_or(5) as u8;
                            let width = width_digit.to_digit(36).unwrap_or(1) as u8;
                            let note_type = sus_type_to_chuni_note_type(type_digit, None);
                            let note = Note {
                                lane,
                                tick: 0,
                                note_type,
                                width,
                            };
                            return Some(SusLine::Note(note));
                        }
                        return Some(SusLine::DirectionalNotes {
                            measure,
                            lane,
                            data: data.to_string(),
                        });
                    }
                }
                _ => {}
            }
        }
    }

    Some(SusLine::Unknown {
        content: line.to_string(),
    })
}

/// Parse a single line of SUS data
fn parse_line(line: &str) -> SusLine {
    let trimmed = line.trim();

    // Empty lines are treated as comments
    if trimmed.is_empty() {
        return SusLine::Comment {
            content: String::new(),
        };
    }

    // Lines starting with # are metadata or chart data
    if trimmed.starts_with('#') {
        // Try parsing as chart data first (contains colon)
        if let Some(parsed) = parse_chart_data_line(trimmed) {
            return parsed;
        }
        // Fall back to metadata
        if let Some(parsed) = parse_metadata_line(trimmed) {
            return parsed;
        }
    }

    // Everything else is a comment
    SusLine::Comment {
        content: trimmed.to_string(),
    }
}

/// Parse a complete SUS file using an iterator-based approach
pub fn parse_sus(content: &str) -> SusChart {
    parse_sus_iter(content.lines())
}

/// Iterator-based SUS parser
pub fn parse_sus_iter<'a, I>(lines: I) -> SusChart
where
    I: IntoIterator<Item = &'a str>,
{
    let mut metadata = HashMap::new();
    let parsed_lines: Vec<SusLine> = lines
        .into_iter()
        .map(|line| {
            let parsed_line = parse_line(line);
            // Extract metadata for easy access
            if let SusLine::Metadata { key, value } = &parsed_line {
                metadata.insert(key.clone(), value.clone());
            }
            parsed_line
        })
        .collect();

    SusChart {
        metadata,
        lines: parsed_lines,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_metadata() {
        let line = "#TITLE \"Test Song\"";
        let result = parse_line(line);
        match result {
            SusLine::Metadata { key, value } => {
                assert_eq!(key, "TITLE");
                assert_eq!(value, "Test Song");
            }
            _ => panic!("Expected metadata, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_comment() {
        let line = "This is a comment";
        let result = parse_line(line);
        match result {
            SusLine::Comment { content } => {
                assert_eq!(content, "This is a comment");
            }
            _ => panic!("Expected comment, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_bpm_definition() {
        let line = "#BPM01: 140.0";
        let result = parse_line(line);
        match result {
            SusLine::BpmDefinition { id, bpm } => {
                assert_eq!(id, "01");
                assert_eq!(bpm, 140.0);
            }
            _ => panic!("Expected BPM definition, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_measure_length() {
        let line = "#00002: 4.0";
        let result = parse_line(line);
        match result {
            SusLine::MeasureLength { measure, length } => {
                assert_eq!(measure, 0);
                assert_eq!(length, 4.0);
            }
            _ => panic!("Expected measure length, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_tap_notes() {
        let line = "#00010: 14141414";
        let result = parse_line(line);
        match result {
            SusLine::Note(note) => {
                assert_eq!(note.lane, 0);
                assert_eq!(note.note_type, super::super::ChuniNoteType::Tap);
                assert_eq!(note.width, 4);
            }
            _ => panic!("Expected tap note, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_hold_notes() {
        let line = "#00020a: 14002400";
        let result = parse_line(line);
        match result {
            SusLine::Note(note) => {
                assert_eq!(note.lane, 0);
                assert_eq!(note.note_type, super::super::ChuniNoteType::Hold);
                assert_eq!(note.width, 4);
            }
            _ => panic!("Expected hold note, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_slide_notes() {
        let line = "#00030a: 14340024";
        let result = parse_line(line);
        match result {
            SusLine::Note(note) => {
                assert_eq!(note.lane, 0);
                assert_eq!(note.note_type, super::super::ChuniNoteType::Slide);
                assert_eq!(note.width, 4);
            }
            _ => panic!("Expected slide note, got {:?}", result),
        }
    }

    #[test]
    fn test_parse_full_sus() {
        let content = r#"#TITLE "Test Song"
#ARTIST "Test Artist"
#BPM01: 120.0
This is a comment
#00002: 4.0
#00008: 01
#00010: 14141414"#;

        // Test both parse_sus and parse_sus_iter
        let chart = parse_sus(content);
        assert_eq!(chart.metadata.get("TITLE"), Some(&"Test Song".to_string()));
        assert_eq!(
            chart.metadata.get("ARTIST"),
            Some(&"Test Artist".to_string())
        );
        assert_eq!(chart.lines.len(), 7);

        let chart_iter = parse_sus_iter(content.lines());
        assert_eq!(
            chart_iter.metadata.get("TITLE"),
            Some(&"Test Song".to_string())
        );
        assert_eq!(
            chart_iter.metadata.get("ARTIST"),
            Some(&"Test Artist".to_string())
        );
        assert_eq!(chart_iter.lines.len(), 7);
    }

    #[test]
    fn test_parse_hex_measure() {
        // Test parsing measure numbers in base 36
        assert_eq!(parse_measure("000"), Some(0));
        assert_eq!(parse_measure("001"), Some(1));
        assert_eq!(parse_measure("00a"), Some(10));
        assert_eq!(parse_measure("010"), Some(36));
    }
}
