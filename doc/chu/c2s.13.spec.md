# C2S Chart Documentation (v1.13.00)

This is a revised version of the C2S unofficial specifications, which includes updates and corrections to the original document. The changes are based on the latest C2S chart version 1.13.00, used since *CHUNITHM NEW* and later.

## Note Types

### Tap Notes (`TAP`)

A basic tap note that requires a single tap to hit. It is represented by a red bar.

![Screenshot of the tutorial, Intro to Tap Notes](https://chunithm.org/basic/images/tap.png)

**Schema:**

```text
TAP [measure] [tick] [cell] [width]
```

Example: `TAP 8 0 6 4`

### ExTap Notes (`CHR`)

A special tap note that will always return a "Justice" (Perfect/Marvelous) judgement regardless of timing. It is represented by a golden bar.

There are also variants of standard notes with ExTap properties, usually represented by its normal note type internally but with `X` as the second character in the note type string. These are apparently called "Fake ExTap" notes, and they are used in some charts to create a visual effect of a tap note that is not actually an ExTap.

- `SXD` - Fake ExTap Slide
- `SXC` - Fake ExTap Slide (Control Point)
- `HXD` - Fake ExTap Hold

> "Fake ExTaps" are introduced in C2S 1.13.00.

![Fake ExTap slide notes](https://chunithm.org/basic/images/fake-extap.png)

**Schema:**

```text
CHR [measure] [tick] [cell] [width] [modifier]
```

- `modifier`: Direction modifier ("UP", "CE", "DW")

Example: `CHR 9 0 4 8 CE`

### Hold Notes (`HLD`)

A hold note that requires the player to hold down the button for a certain duration. It is represented by an orange bar with a blue-yellow gradient tail.

![Screenshot of the tutorial, Intro to Hold Notes](https://chunithm.org/basic/images/hold.png)

**Schema:**

```text
HLD [measure] [tick] [cell] [width] [duration]
```

- `duration`: Hold duration in ticks

Example: `HLD 18 0 0 3 192`

**ExHold Notes (`HXD`):**

```text
HXD [measure] [tick] [cell] [width] [duration]
```

Example: `HXD 12 96 8 4 288`

### Slide Notes (`SLD`)

A note similar to a hold note, but it requires the player to slide their finger across the slider bar. It is represented by a blue bar with a magenta-blue gradient tail.

![Screenshot of the tutorial, Intro to Slide Notes](https://chunithm.org/basic/images/slide.png)

**Schema:**

```text
SLD [measure] [tick] [cell] [width] [duration] [end_cell] [end_width]
```

- `duration`: Slide duration in ticks
- `end_cell`: Final horizontal position (can be decimal)
- `end_width`: Final width in cells (can be decimal)

Example: `SLD 8 148 0 4 236 0 4`

**ExSlide Notes (`SXD`):**

```text
SXD [measure] [tick] [cell] [width] [duration] [end_cell] [end_width]
```

Example: `SXD 12 379 9 6 5 10 6`

#### Slide Control Points (`SLC`)

Slide control points are waypoints that define the path of a slide note. They control the direction and curvature of the slide path, allowing for complex slide patterns.

- `SLC` - Standard slide control point that starts immediately with movement
- `SXC` - Slide control point that starts with an ExTap (Fake ExTap variant)

**SLC Schema:**

```text
SLC [measure] [tick] [cell] [width] [duration] [end_cell] [end_width]
```

Example: `SLC 8 96 4 4 7 3 4`

**SXC Schema:**

```text
SXC [measure] [tick] [cell] [width] [duration] [end_cell] [end_width] [target] [modifier]
```

- `target`: Reference to slide note ("SLD")
- `modifier`: Direction modifier ("UP")

Example: `SXC 12 0 10 6 9 9 6 SLD UP`

### AIR Notes (`AIR`, `A*`)

Air notes are special notes that require the player to wave to the IR sensors without touching any buttons. They are usually represented by an animated green arrow.

![An image of a basic AIR note alongside a Hold](https://chunithm.org/basic/images/air.png)

**Basic AIR Schema:**

```text
AIR [measure] [tick] [cell] [width] [target_note]
```

- `target_note`: Referenced note type ("CHR", "TAP", etc.)

Example: `AIR 9 0 4 8 CHR`

**Directional Air Notes Schema (AUR, AUL, ADW, ADR, ADL):**

```text
A[direction] [measure] [tick] [cell] [width] [target_note] [modifier]
```

- `direction`: UR (up-right), UL (up-left), DW (down), DR (down-right), DL (down-left)
- `target_note`: Referenced note type
- `modifier`: Usually "DEF" - a default modifier indicating no special behavior

Example: `AUL 14 288 0 4 FLK DEF`

There are also other cosmetic variants of air notes with diagonal hints, which are used in some charts to hint to the player that they should hover in a specific direction. The game technically only keeps track of vertical velocity for these notes, so they can still be hit by simply waving your hand up and triggering the IR motion sensor.

> [!NOTE]
> Sometimes you may not even need to use your hand to trigger the IR sensor, as any body part that blocks the IR sensors may trigger it.
> This is an intended gimmick in some harder charts,
>
> ![A player headbanging and clapping to trigger AIR notes](https://chunithm.org/basic/images/headbang.gif)

#### AIR Down Notes (`AD*`)

AIR down notes are a variant of air notes that require the player to trigger the sensor by moving their hand downwards. They are represented by a pink arrow pointing downwards. The triggering mechanic is the same as for regular AIR notes, but inverted.

![An image of an AIR up and down note](https://chunithm.org/basic/images/down-air.png)

#### AIR-ACTION and AIR-Hold Notes

AIR-ACTION notes are extensions of air notes, and they go hand in hand with "air-holds".

![Screenshot of the tutorial, Intro to AIR-ACTION Notes](https://chunithm.org/basic/images/air-action-1.png)

![Screenshot of the tutorial, Intro to AIR-ACTION Notes and how to trigger them](https://chunithm.org/basic/images/air-action-2.png)

The green line stemming from the initial air note indicates that you
need to keep your hand hovering within the air sensor region (the "air-hold"),
and purple bars that fall towards the judgement line (the air-action) are again triggered by movement within the air sensor region.
This can be movement up through the sensor, down through the sensor, or even waving up and down. Timing windows are similarly generous.

When a player misses an AIR-ACTION note, the game will display a message on the bottom right corner of the screen, hinting that the player should try to hover their hand in the air sensor region.

![A GIF showing a player missing an AIR-ACTION note, and the in-game hint](https://chunithm.org/basic/images/air-miss.gif)

**AHD - Air Hold Schema:**

```text
AHD [measure] [tick] [cell] [width] [target_note] [duration]
```

- `target_note`: Referenced note type ("SLD", "CHR", etc.)
- `duration`: Hold duration in ticks

Example: `AHD 9 0 0 4 SLD 192`

**AHX - AIR-ACTION Schema** *(Undocumented)*?

```text
AHX [measure] [tick] [cell] [width] [target_note] [duration] [modifier]
```

- `target_note`: Referenced note type ("SLD", "TAP", etc.)
- `duration`: Duration in ticks (typically 96)
- `modifier`: Usually "DEF"

AIR-ACTION notes are movement-triggered purple bars in the air sensor region. They require hand movement within the air sensor region rather than just hovering. **There are two types of AIR-ACTION implementations:**

1. **AHX notes** - Individual AIR-ACTION with purple bar visual
2. **ALD+NON notes** - Individual AIR-ACTION, or multiple simultaneous notes creating AIR CRUSH patterns

**Example alternating pattern (2/4 clap, alternate between ExTap-AirAction):**

```text
AHX 65 192 12 4 SLD 96 DEF      # Single AIR-ACTION (purple bar)
CHR 65 240 4 8 RS               # ExTap (1/8 offbeat)
ALD 65 240 12 4 38400 5.0 1 12 4 5.0 NON  # Single AIR-ACTION
CHR 65 288 4 8 RS               # ExTap (1/8 offbeat)
ALD 65 288 12 4 38400 5.0 1 12 4 5.0 NON  # Single AIR-ACTION
```

**AIR CRUSH patterns:** Multiple ALD+NON notes placed simultaneously create geometric patterns like the "melon pattern" (6 simultaneous AIR-ACTIONs).

Example: `AHX 33 192 0 4 SLD 96 DEF`

**ALD - Air Slide Schema** *(Undocumented)*?

```text
ALD [measure] [tick] [cell] [width] [duration] [param1] [param2] [end_cell] [end_width] [param3] [param4]
```

**Parameter Meanings:**

- `param4`: Visual style parameter
  - `"DEF"` - Regular air slide notes
  - `"NON"` - AIR CRUSH voxel effects (multiple notes create 3D shapes)
  - `"BLK"` - Black/invisible air slides

**Examples:**

- `ALD 66 0 7 2 38400 5.0 2 7 2 1.0 NON` (AIR CRUSH)
- `ALD 10 0 11 2 0 1.0 1 13 2 6.0 DEF` (Regular air slide)

**ASC - Air Slide Control Point** *(Undocumented)*?

```text
ASC [measure] [tick] [cell] [width] [duration] [end_cell] [end_width]
```

Example: `ASC 14 288 4 4 21 3 4`

**Note:** ASC notes can also be wrapped in the ASD format (see ASD Wrapper Format section), which would appear as:
`ASD [measure] [tick] [cell] [width] ASC [param1] [duration] [end_cell] [end_width] [param2] [param3]`

#### AIR CRUSH Notes

> These are introduced in C2S 1.13.00.

AIR CRUSH notes are **AIR-ACTION notes (ALD+NON) arranged in specific patterns and played simultaneously**. They create complex 3D voxel-based visual effects in the air sensor region when multiple ALD+NON notes are triggered at the same time.

**Key Understanding:** AIR CRUSH is not a separate note type, but rather a **visual/gameplay effect** created when multiple ALD+NON notes (AIR-ACTIONs) are arranged in specific geometric patterns.

**Implementation:**

AIR CRUSH effects are created by placing multiple ALD notes with the "NON" parameter at the same timing:

```text
Multiple ALD [measure] [tick] [cell] [width] [duration] [param1] [param2] [end_cell] [end_width] [param3] NON
```

**Key characteristics:**

- Uses "NON" parameter (AIR-ACTION) instead of "DEF" (regular air slides)
- **Multiple notes placed simultaneously** to create 3D geometric shapes
- Parameter values (param1, param3) control visual depth/layering (typically 1.0-6.0)
- Cell positions and widths define the overall shape pattern
- All notes in the pattern require hand movement in the air sensor region

**Example - "Melon Pattern" (6 simultaneous AIR-ACTIONs):**

```text
ALD 42 0 5 6 6 3.0 1 5 6 3.0 NON
ALD 42 0 5 6 6 4.0 1 5 6 4.0 NON
ALD 42 0 6 4 6 2.0 1 6 4 2.0 NON
ALD 42 0 6 4 6 5.0 1 6 4 5.0 NON
ALD 42 0 7 2 6 1.0 1 7 2 1.0 NON
ALD 42 0 7 2 6 6.0 1 7 2 6.0 NON
```

This creates a symmetrical spherical pattern with:

- **6 pairs** of notes creating depth
- **Varying widths** (6, 4, 2) for curved appearance
- **Different parameter values** (1.0-6.0) for layering effects

![A screenshot of AIR CRUSH notes](https://chunithm.org/basic/images/air-crush.png)

---

Note: The types marked with `?` are not fully documented and may require further investigation.

The following air note types have been observed and documented based on actual chart data:

- `AIR` - Regular AIR (up) note
- `AUR` - AIR Up (with right diagonal hint)
- `AUL` - AIR Up (with left diagonal hint)
- `ADW` - AIR Down
- `ADL` - AIR Down (with left diagonal hint)
- `ADR` - AIR Down (with right diagonal hint)
- `AHD` - AIR-ACTION Hover
- `ASD` - AIR Slide (wrapper format)

The following types require further investigation:

- `ALD` - ??? (Undocumented air note variant)?
- `ASC` - AIR Slide (with control point)? (Undocumented)
- `AIR-ACTION` - AIR CRUSH notes? (TBD, needs documentation)

> **Note**: AIR-ACTION notes are triggered by any movement within the air sensor region - this includes moving up, down, or waving back and forth. The timing windows for these notes are generally more generous than standard notes.

---

## Flick Notes (`FLK`)

Flick notes are notes that require a flick in either horizontal direction to hit. These notes only appear in MASTER and harder difficulties, and they are represented by a silver bar with a blue center.

![A screenshot of flick notes](https://chunithm.org/basic/images/flick-1.png)

**Schema:**

```text
FLK [measure] [tick] [cell] [width] L
```

- Always ends with "L" (presumably for "Left" default direction)

Example: `FLK 14 0 0 4 L`

## Mine Notes (`MNE`)

Mine notes are special notes that require the player to avoid hitting them. They are represented by a dark-blue bar with electric sparks.

- Hitting a mine note results in a "Miss" judgement and breaks the player's combo.
- Ignoring (not hitting) a mine note counts as a successful avoidance, granting a "Justice" judgement and contributing to the player's score and combo.

**Schema:**

```text
MNE [measure] [tick] [cell] [width]
```

Example: `MNE 15 192 8 2`

## Special Note Types

### Default/Placeholder note (`DEF`) ?

This is a special placeholder note type added in C2S 1.13.00. It is used to represent an empty note in the chart, such as an independent AIR note with no other notes bound to it. It's invisible and is used to maintain the structure of the chart without adding any visible elements.

**Schema:**

None, this note type is only used as a null pointer(?) for some AIR or other notes.

```text
AUL 14 288 0 4 FLK DEF
```

### ASD Wrapper Format

ASD is a special 12-field wrapper format that can encapsulate any other note type with additional air-related parameters:

**Schema:**

```text
ASD [measure] [tick] [cell] [width] [wrapped_type] [param1] [duration] [end_cell] [end_width] [param2] [param3]
```

**Fields:**

- `wrapped_type`: The note type being wrapped (TAP, CHR, SLD, HLD, ASC, even ASD)
- `param1`: First air parameter (usually 5.0)
- `duration`: Duration in ticks
- `end_cell`: End cell position (integer)
- `end_width`: End width (integer)
- `param2`: Second air parameter (usually 5.0)
- `param3`: Third air parameter (usually "DEF")

**Examples:**

- `ASD 12 0 0 6 CHR 5.0 384 0 3 5.0 DEF` (wraps CHR)
- `ASD 13 0 10 6 SLD 5.0 384 13 3 5.0 DEF` (wraps SLD)
- `ASD 18 0 0 3 HLD 5.0 192 3 3 5.0 DEF` (wraps HLD)
- `ASD 56 240 8 4 ASD 5.0 144 8 4 5.0 DEF` (wraps ASD recursively)

**Note:** ASC notes can also use the ASD wrapper format, maintaining their AirSlideControlPoint behavior while using the extended 12-field structure.

---

## Basic Note Format

All notes in C2S format follow a tab-separated value structure with the following base fields:

```text
[TYPE] [measure] [tick] [cell] [width] [additional_fields...]
```

**Base Fields (required for all notes):**

- `TYPE`: Note type identifier (TAP, CHR, SLD, etc.)
- `measure`: Measure number where the note appears (integer)
- `tick`: Offset within the measure in ticks (integer)
- `cell`: Horizontal position (0-15, representing 16 cells)
- `width`: Width of the note in cells (integer)

---

## Implementation Notes

> **NOTE:**
> All the types marked with `?` (ALD, ASC, DEF, AirAction) are not fully documented and may differ from actual specifications.
> Testing with real chart data is required to confirm the exact format.

### Enum Variants in Rust Implementation

- `Tap` (TAP)
- `ExTap` (CHR)
- `Hold` (HLD)
- `ExHold` (HXD)
- `Slide` (SLD)
- `ExSlide` (SXD)
- `SlideControlPoint` (SLC)
- `ExSlideControlPoint` (SXC)
- `Flick` (FLK)
- `Air` (AIR)
- `AirHold` (AHD)
- `AirSlide` (ALD)? *- Undocumented, schema inferred*
- `AirSlideControlPoint` (ASC)? *- Undocumented, schema inferred*
- `AirDirectional` with variants for (AUR, AUL, ADW, ADR, ADL)
- `Default` (DEF)? *- Placeholder note type*
- `AirAction` for AIR-ACTION notes? *- Format unknown*
- `Mine` (MNE)
- `Unknown(String)` for unrecognized note types

### ASD Wrapper Implementation

The ASD wrapper is implemented as a special parsing mode that:

1. Extracts the wrapped note type from field 5
2. Uses the wrapped type for note behavior (except ASC which maintains its type)
3. Preserves all ASD parameters in a `WrappedNoteInfo` structure
4. Allows detection of originally wrapped notes for conversion/export

### Unknown Note Types

Unknown note types are parsed as `Unknown(String)` variants to ensure forward compatibility. This allows the parser to handle future note types that may be introduced in later versions of the C2S format without breaking existing functionality.

## Summary

This documentation represents our current understanding of the C2S v1.13.00 format based on reverse engineering and analysis of actual chart files. Key findings include:

### Major Discoveries

1. **ASD Wrapper Format**: ASD is not a standalone note type but a 12-field wrapper that can encapsulate any other note type with additional air parameters.

2. **ASC Dual Format**: ASC notes can use either standard 8-field format or the extended 12-field ASD wrapper format.

3. **Recursive Wrapping**: ASD can wrap other ASD notes, creating nested wrapper structures.

4. **Parameter Patterns**: Most ASD notes use consistent parameters (param1=5.0, param2=5.0, param3="DEF").

5. **AIR CRUSH Implementation**: AIR CRUSH notes are implemented using ALD notes with "NON" parameter, allowing complex 3D voxel-based visual effects through multiple simultaneous notes.

6. **🔥 AIR-ACTION Dual System**: **MAJOR BREAKTHROUGH** - AIR-ACTION notes have two implementations that can be used interchangeably:
   - **AHX notes**: Standard AIR-ACTION with purple bar visual
   - **ALD+NON notes**: AIR-ACTION with voxel-based CRUSH effects
   - Both are movement-triggered and can alternate in complex patterns (e.g., 2/4 clap sequences)

### Implementation Status

- ✅ **Fully Implemented**: TAP, CHR, FLK, HLD, HXD, SLD, SXD, SLC, SXC, AIR, AHD, AUR/AUL/ADW/ADR/ADL, MNE, DEF
- ✅ **ASD Wrapper**: Complete implementation with metadata preservation
- ✅ **AIR-ACTION**: AHX format documented, ALD+NON dual implementation discovered
- ⚠️ **Partially Documented**: ALD (schema inferred), ASC (Wrapper format, schema inferred)

### Future Work

- Validate AIR-ACTION dual system with more chart samples
- Research exact visual differences between AHX and ALD+NON AIR-ACTIONs
- Investigate AIR CRUSH note variants in newer chart versions
- Validate ALD field interpretations with more chart samples
- Research param1/param2/param3 semantic meanings in ASD format

---

*This specification is based on analysis of CHUNITHM chart files and may be updated as new information becomes available.*
