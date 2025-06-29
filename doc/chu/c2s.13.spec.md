# C2S Chart Documentation (v1.13.00)

This is a revised version of the C2S unofficial specifications, which includes updates and corrections to the original document. The changes are based on the latest C2S chart version 1.13.00, used since *CHUNITHM NEW* and later.

## Note Types

### Tap Notes (`TAP`)

A basic tap note that requires a single tap to hit. It is represented by a red bar.

![Screenshot of the tutorial, Intro to Tap Notes](https://chunithm.org/basic/images/tap.png)

### ExTap Notes (`CHR`)

A special tap note that will always return a "Justice" (Perfect/Marvelous) judgement regardless of timing. It is represented by a golden bar.

There are also variants of standard notes with ExTap properties, usually represented by its normal note type internally but with `X` as the second character in the note type string. These are apparently called "Fake ExTap" notes, and they are used in some charts to create a visual effect of a tap note that is not actually an ExTap.

- `SXD` - Fake ExTap Slide
- `SXC` - Fake ExTap Slide (Control Point)
- `HXD` - Fake ExTap Hold

> "Fake ExTaps" are introduced in C2S 1.13.00.

![Fake ExTap slide notes](https://chunithm.org/basic/images/fake-extap.png)

### Hold Notes (`HLD`)

A hold note that requires the player to hold down the button for a certain duration. It is represented by an orange bar with a blue-yellow gradient tail.

![Screenshot of the tutorial, Intro to Hold Notes](https://chunithm.org/basic/images/hold.png)

### Slide Notes (`SLD`)

A note similar to a hold note, but it requires the player to slide their finger across the slider bar. It is represented by a blue bar with a magenta-blue gradient tail.

![Screenshot of the tutorial, Intro to Slide Notes](https://chunithm.org/basic/images/slide.png)

#### Slide Control Points (`SLC`)

Slide control points are waypoints that define the path of a slide note. They control the direction and curvature of the slide path, allowing for complex slide patterns.

- `SLC` - Standard slide control point that starts immediately with movement
- `SXC` - Slide control point that starts with an ExTap (Fake ExTap variant)

### AIR Notes (`AIR`, `A*`)

Air notes are special notes that require the player to wave to the IR sensors without touching any buttons. They are usually represented by an animated green arrow.

![An image of a basic AIR note alongside a Hold](https://chunithm.org/basic/images/air.png)

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

#### AIR CRUSH Notes

> These are introduced in C2S 1.13.00.

AIR CRUSH notes are a special type of AIR-ACTION note that has cosmetic hints, allowing the charter to customize the appearance of the note through a voxel-based system.

These notes are visually represented by a composition of multiple AIR-ACTION blocks, arranged in a specific pattern to create a unique visual effect.

![A screenshot of AIR CRUSH notes](https://chunithm.org/basic/images/air-crush.png)

---

Note: The types marked with `???` are not fully documented and may require further investigation.

- `AIR` - Regular AIR (up) note
- `AUR` - AIR Up (with right diagonal hint)
- `AUL` - AIR Up (with left diagonal hint)
- `ADW` - AIR Down
- `ADL` - AIR Down (with left diagonal hint)
- `ADR` - AIR Down (with right diagonal hint)
- `AHD` - AIR-ACTION Hover
- `ALD` - ??? (Undocumented air note variant)
- `ASC` - AIR Slide (with control point) ??? (Undocumented)
- `ASD` - AIR Slide ??? (Undocumented)
- ??? - AIR CRUSH notes (TBD, needs documentation)

> **Note**: AIR-ACTION notes are triggered by any movement within the air sensor region - this includes moving up, down, or waving back and forth. The timing windows for these notes are generally more generous than standard notes.

---

## Flick Notes (`FLK`)

Flick notes are notes that require a flick in either horizontal direction to hit. These notes only appear in MASTER and harder difficulties, and they are represented by a silver bar with a blue center.

![A screenshot of flick notes](https://chunithm.org/basic/images/flick-1.png)

## Mine Notes (`MNE`)

Mine notes are special notes that require the player to avoid hitting them. They are represented by a dark-blue bar with electric sparks.

- Hitting a mine note results in a "Miss" judgement and breaks the player's combo.
- Ignoring (not hitting) a mine note counts as a successful avoidance, granting a "Justice" judgement and contributing to the player's score and combo.

## Special Note Types

### Default/Placeholder note (`DEF`)

This is a special placeholder note type added in C2S 1.13.00. It is used to represent an empty note in the chart, such as an independent AIR note with no other notes bound to it. It's invisible and is used to maintain the structure of the chart without adding any visible elements.

---

## Implementation Notes

The corresponding enum variants in the Rust implementation include:

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
- `AirDirectional` with variants for (AUR, AUL, ADW, ADR, ADL)
- `AirAction` for AIR-ACTION notes
- `Mine` (MNE)
- `Default` (DEF)

Unknown or undocumented note types (ALD, ASC, ASD, AIR CRUSH variants) are not yet implemented in the enum and require further investigation.
