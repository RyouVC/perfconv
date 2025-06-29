//! SUS format parser for CHUNITHM/maimai/ongeki charts
//!
//! This module provides parsing and data structures for the SUS (Sliding Universal Score) format.
//!
//! SUS is a text-based format used for SUSPlayer and Seaurchin charts, a CHUNITHM simulator.

use std::collections::HashMap;

/// Represents a parsed SUS chart.
pub struct SusChart {
    /// Chart metadata (title, artist, etc.)
    pub metadata: HashMap<String, String>,
    /// Parsed note/event data
    pub notes: Vec<SusNote>,
}

/// Represents a single note or event in SUS.
pub struct SusNote {
    // TODO: Add fields for lane, timing, type, etc.
    pub lane: u8,
    pub tick: u32,
    pub note_type: String,
    // ...other fields as needed
}

/// Parses a SUS file from a string.
pub fn parse_sus(input: &str) -> Result<SusChart, String> {
    let mut metadata = HashMap::new();
    let mut notes = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        // Only process lines starting with '#'
        if !line.starts_with('#') {
            continue;
        }
        // Metadata lines: #KEY ...
        if let Some((key, rest)) = line[1..].split_once(' ') {
            let key_upper = key.to_ascii_uppercase();
            // Remove quotes if present
            let value = rest.trim().trim_matches('"');
            // Only parse known metadata keys for now
            match key_upper.as_str() {
                "TITLE" | "SUBTITLE" | "ARTIST" | "GENRE" | "DESIGNER" | "DIFFICULTY"
                | "PLAYLEVEL" | "SONGID" | "WAVE" | "WAVEOFFSET" | "JACKET" | "BACKGROUND"
                | "MOVIE" | "MOVIEOFFSET" | "BASEBPM" | "REQUEST" => {
                    metadata.insert(key_upper, value.to_string());
                }
                _ => {
                    // Not a recognized metadata key, ignore for now
                }
            }
            continue;
        }
        // Chart data lines and other commands (stub for now)
        // TODO: Implement chart data parsing
    }

    Ok(SusChart { metadata, notes })
}
