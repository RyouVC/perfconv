//! A format for a certain spiky marine simulator

use std::collections::HashMap;

pub struct UGCChart {
    pub metadata: HashMap<String, String>,
    pub timelines: Vec<Vec<UGCParentNote>>,
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

pub struct UGCChildNote {
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
    Tap,
    ExTap {
        direction: ExTapEffectDirection,
    },
    Flick {
        direction: FlickEffectDirection,
    },
    Damage,
    Hold {
        child: UGCChildNote,
    },
    Slide {
        children: Vec<UGCChildNote>,
    },
    Air {
        direction: AirDirection,
        color: AirColor,
    },
    AirHold {
        color: AirColor,
        children: Vec<UGCChildNote>,
    },
    AirSlide {
        height: u16,
        color: AirColor,
        children: Vec<UGCChildNote>,
    },
    AirCrush {
        height: u16,
        color: AirCrushColor,
        interval: u8,
        child: UGCChildNote,
    },
}

pub struct UGCParentNote {
    pub note_type: ParentNoteType,
    pub bar: u64,
    pub tick: u64,
    pub lane: u8,
    pub width: u8,
}
