//! SUS format parser for CHUNITHM sim charts
//!
//! This module provides parsing and data structures for the SUS (Sliding Universal Score) format.
//!
//! SUS is a text-based format used for SUSPlayer and Seaurchin charts, a CHUNITHM simulator.

use std::collections::HashMap;
use chumsky::prelude::*;

pub enum SusLine {
    Metadata {
        key: String,
        value: String,
    },
    Comment {
        content: String,
    },
    Note(Note)
}

/// Represents a parsed SUS chart.
pub struct SusChart {
    /// Chart metadata (title, artist, etc.)
    pub metadata: HashMap<String, String>,
    /// Parsed note/event data
    pub notes: Vec<Note>,
}
/// Parses a single line of comment, which is any line that doesn't start with a `#`.
fn parse_comment_line<'a>() -> impl Parser<'a, &'a str, SusLine> {
    // A comment line is any line that doesn't start with '#', up to but not including the newline (CRLF or LF)
    // We parse until we hit '\n' or '\r', but do not include them in the comment content.
    any()
        .filter(|c| *c != '#')
        .then(
            any()
                .filter(|c| *c != '\n' && *c != '\r')
                .repeated()
                .collect::<String>(),
        )
        .map(|(first, rest)| {
            let mut content = String::new();
            content.push(first);
            content.push_str(&rest);
            SusLine::Comment { content }
        })
}

/// Chumsky parser for SUS metadata commands
fn parse_metadata_line<'a>() -> impl Parser<'a, &'a str, SusLine> {
    just('#')
        .ignore_then(
            none_of(" \n")
                .repeated()
                .collect::<String>()
                .then_ignore(just(' '))
                .then(
                    just('"')
                        .ignore_then(none_of("\"").repeated().collect::<String>())
                        .then_ignore(just('"'))
                        .or(none_of("\n").repeated().collect::<String>())
                )
                .map(|(key, value)| SusLine::Metadata {
                    key: key.trim().to_string(),
                    value: value.trim().to_string(),
                }),
        )
}

fn parse_note_line<'a>() -> impl Parser<'a, &'a str, SusLine> {
    // Parse a line if it contains ':'
    // For simplicity, we parse: <lane>:<tick>:<note_type>
    // Example: 01:0480:1
    // Dummy output: only check if there's a ':' in the line, otherwise fail to parse
    any()
        .repeated()
        .collect::<String>()
        .try_map(|line, _span| {
            if line.contains(':') {
                Ok(SusLine::Note(Note {
                    lane: 0,
                    tick: 0,
                    note_type: "dummy".to_string(),
                }))
            } else {
                Err(chumsky::error::EmptyErr::default())
            }
        })
}

/// Represents a single note or event in SUS.
pub struct Note {
    // TODO: Add fields for lane, timing, type, etc.
    pub lane: u8,
    pub tick: u32,
    pub note_type: String,
}

/// Parses a SUS file from a string.
pub fn parse_sus<'a>() -> impl Parser<'a, &'a str, Vec<SusLine>> {
    // A SUS file is a sequence of lines, each of which may be metadata, comment, or note.
    // We'll parse each line, skipping empty lines.
    let line_parser = parse_comment_line()
        .or(parse_note_line())
        .or(parse_metadata_line());

    // Parse each line, optionally followed by a newline, and collect into a Vec
    line_parser
        .then_ignore(text::newline().or_not())
        .repeated()
        .collect()
        .then_ignore(end())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::Parser;

    #[test]
    fn test_parse_metadata_line() {
        let input = "#TITLE \"Test Song\"";
        let result = parse_metadata_line().parse(input).into_result();
        match result {
            Ok(SusLine::Metadata { key, value }) => {
                assert_eq!(key, "TITLE");
                assert_eq!(value, "Test Song");
            }
            _ => panic!("Failed to parse metadata line"),
        }
    }

    #[test]
    fn test_parse_comment_line() {
        let input = "This is a comment";
        let result = parse_comment_line().parse(input).into_result();
        match result {
            Ok(SusLine::Comment { content }) => {
                assert_eq!(content, "This is a comment");
            }
            _ => panic!("Failed to parse comment line"),
        }
    }

    #[test]
    fn test_parse_sus_multiple_lines() {
        let input = "#ARTIST \"Composer\"\nThis is a comment\n";
        let result = parse_sus().parse(input).into_result();
        assert!(result.is_ok());
        let lines = result.unwrap();
        assert_eq!(lines.len(), 2);
        match &lines[0] {
            SusLine::Metadata { key, value } => {
                assert_eq!(key, "ARTIST");
                assert_eq!(value, "Composer");
            }
            _ => panic!("First line should be metadata"),
        }
        match &lines[1] {
            SusLine::Comment { content } => {
                assert_eq!(content, "This is a comment");
            }
            _ => panic!("Second line should be comment"),
        }
    }
}