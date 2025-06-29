//! A certain edgy rhythm game.
pub mod c2s;
pub mod sus;
pub mod ugc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AirDirection {
    UpRight,
    UpLeft,
    Down,
    DownRight,
    DownLeft,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChuniNoteType {
    /// A normal note, requiring the player to tap the screen
    Tap,
    /// A note that requires the player to tap the screen,
    /// but always registers as a perfect regardless of timing (CHR)
    ExTap,
    /// A note that requires the player to hold down (HLD)
    Hold,
    /// A hold note with an ExTap at the start (HXD)
    ExHold,
    /// A slide note that starts with a straight line (SLD)
    Slide,
    /// A slide note that starts with an ExTap (SXD)
    ExSlide,

    /// A slide control point that starts immediately with movement (SLC)
    SlideControlPoint,
    /// A slide control point that starts with an ExTap (SXC)
    ExSlideControlPoint,

    /// Similar to a normal tap note, but requires the player to swipe
    /// their hand across the screen in either direction (FLK)
    Flick,

    /// An air note, requiring the player to hold their hand in the air
    /// triggering the IR sensor (AIR)
    Air,
    /// An air hold note (AHD)
    AirHold,
    /// AIR-Hold with green ground bar (AHX)? - hybrid air/ground hold note
    /// Appears as a green bar on the ground with air sensor activation
    /// ? = inferred from gameplay analysis, not officially documented  
    AirHoldGround,
    /// Directional air notes (AUR, AUL, ADW, ADR, ADL)
    AirDirectional(AirDirection),
    /// Air slide notes (ALD) - like slides but in the air sensor region
    /// When used with "NON" parameter, creates AIR-ACTION notes (purple floating bars)
    /// Multiple simultaneous ALD+NON notes create AIR CRUSH patterns (e.g., "melon pattern")
    /// ALD+NON requires hand movement in air sensor, used in clap patterns and complex formations
    AirSlide,
    /// Air slide control points (ASC) - like slide control points but in the air sensor region
    AirSlideControlPoint,

    /// A mine note that must not be touched (MNE)
    Mine,
    /// A default placeholder note (DEF)
    /// Used as invisible placeholder to maintain chart structure
    Default,
    /// Unknown note type that needs investigation
    Unknown(String),
}

/// Calculates the offset of a note or timing point from the measure based on the resolution.
///
/// # Arguments
/// * `resolution` - The resolution of the song (e.g., 384 for a measure).
/// * `beat` - The desired beat (e.g., 1 for the first beat, 2 for the second beat, etc.).
/// * `fraction` - A fraction of the beat (e.g., 0.5 for half a beat).
///
/// # Returns
/// The offset from the measure as an integer.
pub fn calculate_offset(resolution: u32, beat: u32, fraction: f32) -> u32 {
    let beat_offset = (beat * (resolution / 4)) - (resolution / 4);
    let fraction_offset = (fraction * (resolution as f32 / 4.0)) as u32;
    beat_offset + fraction_offset
}

/// a CHUNITHM-style chart, for games with freestyle sliders and IR jump notes
pub trait ChuniChart {}
