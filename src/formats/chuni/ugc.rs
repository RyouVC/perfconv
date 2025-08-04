//! A format for a certain spiky marine simulator

use std::collections::HashMap;

use chumsky::extra::Err;
use eyre::OptionExt;

pub struct UGCChart {
    pub metadata: HashMap<String, String>,
    pub timelines: HashMap<u32, Vec<ParentNote>>,
}

pub enum ChildNoteType {
    HoldEndPoint,
    SlideRelayPoint {
        lane: u8,
        width: u8,
    },
    /// Line is Omitted
    SlideControlPoint {
        lane: u8,
        width: u8,
    },
    AirHoldRelayPoint,
    /// Air Action is omitted
    AirHoldControlPoint,
    AirSlideRelayPoint {
        lane: u8,
        width: u8,
        height: u16,
    },
    /// Air Action is omitted
    AirSlideControlPoint {
        lane: u8,
        width: u8,
        height: u16,
    },
    AirCrushEndPoint {
        lane: u8,
        width: u8,
        height: u16,
    },
}

pub struct ChildNote {
    pub note_type: ChildNoteType,
    pub offset_tick: u64,
}

pub enum ExTapEffectDirection {
    Up,
    Down,
    Center,
    Clockwise,
    Counterclockwise,
    Right,
    Left,
    InOut,
}

pub enum FlickEffectDirection {
    Auto,
    Right,
    Left,
}

pub enum AirDirection {
    Up,
    UpRight,
    UpLeft,
    Down,
    DownRight,
    DownLeft,
}

pub enum AirColor {
    Normal,
    Inverted,
}

pub enum AirCrushColor {
    Normal,
    Red,
    Orange,
    Yellow,
    YellowGreen,
    Green,
    Cyan,
    Sky,
    Light,
    Blue,
    BluePurple,
    Magenta,
    Pink,
    White,
    Black,
    Transparent,
}

pub enum ParentNoteType {
    Click,
    Tap {
        lane: u8,
        width: u8,
    },
    ExTap {
        lane: u8,
        width: u8,
        direction: ExTapEffectDirection,
    },
    Flick {
        lane: u8,
        width: u8,
        direction: FlickEffectDirection,
    },
    Damage {
        lane: u8,
        width: u8,
    },
    Hold {
        lane: u8,
        width: u8,
        children: Vec<ChildNote>,
    },
    Slide {
        lane: u8,
        width: u8,
        children: Vec<ChildNote>,
    },
    Air {
        lane: u8,
        width: u8,
        direction: AirDirection,
        color: AirColor,
    },
    AirHold {
        lane: u8,
        width: u8,
        color: AirColor,
        children: Vec<ChildNote>,
    },
    AirSlide {
        lane: u8,
        width: u8,
        height: u16,
        color: AirColor,
        children: Vec<ChildNote>,
    },
    AirCrush {
        lane: u8,
        width: u8,
        height: u16,
        color: AirCrushColor,
        interval: Option<f32>,
        children: Vec<ChildNote>,
    },
}

pub struct ParentNote {
    pub note_type: ParentNoteType,
    pub bar: u64,
    pub tick: u64,
}

fn parse_timing(string: &str) -> eyre::Result<(u64, u64)> {
    let (bar, tick) = string.split_once('\'').ok_or_eyre("failed to split")?;
    Ok((bar.parse::<u64>()?, tick.parse::<u64>()?))
}

fn parse_parent_note(line: &str) -> eyre::Result<ParentNote> {
    let (timing, data) = line.split_once(':').ok_or_eyre("failed to split")?;
    let (bar, tick) = parse_timing(timing)?;

    let note_type = match &data[0..1] {
        "c" => Some(ParentNoteType::Click),
        "t" => Some(ParentNoteType::Tap {
            lane: u8::from_str_radix(&data[1..2], 36)?,
            width: u8::from_str_radix(&data[2..3], 36)?,
        }),
        "x" => Some(ParentNoteType::ExTap {
            lane: u8::from_str_radix(&data[1..2], 36)?,
            width: u8::from_str_radix(&data[2..3], 36)?,
            direction: match &data[3..4] {
                "U" => ExTapEffectDirection::Up,
                "D" => ExTapEffectDirection::Down,
                "C" => ExTapEffectDirection::Center,
                "A" => ExTapEffectDirection::Clockwise,
                "W" => ExTapEffectDirection::Counterclockwise,
                "L" => ExTapEffectDirection::Right,
                "R" => ExTapEffectDirection::Left,
                "I" => ExTapEffectDirection::InOut,
                _ => Err(eyre::Report::msg(format!(
                    "unknown extap effect direction type {}",
                    &data[3..4]
                )))?,
            },
        }),
        "f" => Some(ParentNoteType::Flick {
            lane: u8::from_str_radix(&data[1..2], 36)?,
            width: u8::from_str_radix(&data[2..3], 36)?,
            direction: match &data[3..4] {
                "A" => FlickEffectDirection::Auto,
                "L" => FlickEffectDirection::Left,
                "R" => FlickEffectDirection::Right,
                _ => Err(eyre::Report::msg(format!(
                    "unknown flick effect direction type {}",
                    &data[3..4]
                )))?,
            },
        }),
        "d" => Some(ParentNoteType::Damage {
            lane: u8::from_str_radix(&data[1..2], 36)?,
            width: u8::from_str_radix(&data[2..3], 36)?,
        }),
        "h" => Some(ParentNoteType::Hold {
            lane: u8::from_str_radix(&data[1..2], 36)?,
            width: u8::from_str_radix(&data[2..3], 36)?,
            children: vec![],
        }),
        "s" => Some(ParentNoteType::Slide {
            lane: u8::from_str_radix(&data[1..2], 36)?,
            width: u8::from_str_radix(&data[2..3], 36)?,
            children: vec![],
        }),
        "a" => Some(ParentNoteType::Air {
            lane: u8::from_str_radix(&data[1..2], 36)?,
            width: u8::from_str_radix(&data[2..3], 36)?,
            direction: match &data[3..5] {
                "UC" => AirDirection::Up,
                "UL" => AirDirection::UpRight,
                "UR" => AirDirection::UpLeft,
                "DC" => AirDirection::Down,
                "DL" => AirDirection::DownRight,
                "DR" => AirDirection::DownLeft,
                _ => Err(eyre::Report::msg(format!(
                    "unknown air direction type {}",
                    &data[3..5]
                )))?,
            },
            color: match &data[5..6] {
                "N" => AirColor::Normal,
                "I" => AirColor::Inverted,
                _ => Err(eyre::Report::msg(format!(
                    "unknown air color {}",
                    &data[5..6]
                )))?,
            },
        }),
        "H" => Some(ParentNoteType::AirHold {
            lane: u8::from_str_radix(&data[1..2], 36)?,
            width: u8::from_str_radix(&data[2..3], 36)?,
            color: match &data[3..4] {
                "N" => AirColor::Normal,
                "I" => AirColor::Inverted,
                _ => Err(eyre::Report::msg(format!(
                    "unknown air hold color {}",
                    &data[3..4]
                )))?,
            },
            children: vec![],
        }),
        "S" => Some(ParentNoteType::AirSlide {
            lane: u8::from_str_radix(&data[1..2], 36)?,
            width: u8::from_str_radix(&data[2..3], 36)?,
            height: u16::from_str_radix(&data[3..5], 36)?,
            color: match &data[5..6] {
                "N" => AirColor::Normal,
                "I" => AirColor::Inverted,
                _ => Err(eyre::Report::msg(format!(
                    "unknown air slide color {}",
                    &data[5..6]
                )))?,
            },
            children: vec![],
        }),
        "C" => Some(ParentNoteType::AirCrush {
            lane: u8::from_str_radix(&data[1..2], 36)?,
            width: u8::from_str_radix(&data[2..3], 36)?,
            height: u16::from_str_radix(&data[3..5], 36)?,
            color: match &data[5..6] {
                "0" => AirCrushColor::Normal,
                "1" => AirCrushColor::Red,
                "2" => AirCrushColor::Orange,
                "3" => AirCrushColor::Yellow,
                "4" => AirCrushColor::YellowGreen,
                "5" => AirCrushColor::Green,
                "6" => AirCrushColor::Cyan,
                "7" => AirCrushColor::Sky,
                "8" => AirCrushColor::Light,
                "9" => AirCrushColor::Blue,
                "A" => AirCrushColor::BluePurple,
                "Y" => AirCrushColor::Magenta,
                "B" => AirCrushColor::Pink,
                "C" => AirCrushColor::White,
                "D" => AirCrushColor::Black,
                "Z" => AirCrushColor::Transparent,
                _ => Err(eyre::Report::msg(format!(
                    "unknown air crush color {}",
                    &data[5..6]
                )))?,
            },
            interval: if let Some((_, interval)) = data.split_once(',') {
                Some(interval.parse::<f32>()?)
            } else {
                None
            },
            children: vec![],
        }),
        _ => None,
    }
    .ok_or_eyre(format!("unknown note type {}", &data[0..1]))?;

    Ok(ParentNote {
        note_type,
        bar,
        tick,
    })
}

fn parse_child_note(line: &str) -> ChildNote {
    todo!()
}

impl<T: AsRef<str>> From<T> for UGCChart {
    fn from(value: T) -> Self {
        let mut metadata = HashMap::new();
        let mut timelines = HashMap::new();

        let mut lines = value.as_ref().lines();
        let mut current_timeline = 0u32;

        while let Some(line) = lines.next() {
            let line = line.trim();

            if line.is_empty() || !line.starts_with('@') && !line.starts_with('#') {
                continue;
            }

            if line.starts_with('@') {
                let parts = line.split(' ').collect::<Vec<_>>();

                let key = &parts[0][1..];
                let rest = &parts[1..].join(" ");

                match key {
                    "USETIL" => {
                        if let Ok(new_timeline_value) = rest.parse::<u32>() {
                            current_timeline = new_timeline_value;
                        }
                    }
                    _ => {
                        metadata.insert(String::from(key), rest.clone());
                    }
                }
            } else if line.starts_with('#') {
                let parent_note = parse_parent_note(&line[1..]);
            } else {
                unreachable!()
            }
        }

        Self {
            metadata,
            timelines,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_timing() {
        let (bar, tick) = parse_timing("69'420").unwrap();
        assert_eq!((bar, tick), (69, 420));
    }

    #[test]
    fn test_parse_timing_fail() {
        let result = parse_timing("69:420");
        assert_eq!(result.is_err(), true);
    }
}
