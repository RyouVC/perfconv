# Umiguri Chart v8 Specification

- Basic Information

  - The file extension is `.ugc`.
  - Character encoding is UTF-8; line endings may be LF or CRLF.
  - One command or note per line.
  - Lines not starting with `@` or `#` are ignored.
  - Lines with missing parameters are ignored.
  - Comment lines start with `'`.
  - Time specification uses the `Bar'Tick` format, hereafter referred to as BarTick format.
    Example: Bar 0, Tick 240 → `0'240`

- Header Lines

  - Parameters are separated by horizontal tabs.
  - Commands and their arguments are as follows:

  - VER Version Specification

    - Version (always 8)

  - EXVER Extended Version Specification

    - ExVersion 0 or 1
      - When set to `1`, behaves as if `FLAG:EXLONG` is forcibly `TRUE`.
      - In UMIGURI NEXT, this is always `1`.

  - TITLE Song Title Specification

    - Title of the song

  - SORT Sort Key Specification

    - Key for sorting order
      - Used when sorting by song title; specify a key converted using these rules:
        1. Convert Latin letters to uppercase.  
           `magnet` → `MAGNET`
        2. Remove symbols and whitespace.  
           `Miracle∞Hinacle` → `MIRACLEHINACLE`
        3. Convert hiragana to katakana, remove dakuten/handakuten, uppercase any small kana, and replace long vowel marks with `ウ`.  
           `かーてんこーる!!!!!` → `カウテンコウル`
        4. Convert kanji to their readings, then apply the above rules.  
           `幻想症候群` → `ケンソウシントロウム`
        5. Convert leet speak (including symbols) to its original representation, then apply the above rules.  
           `^/7(L?[_(L#<>+&l^(o)` → `NYARLATHOTEP`
        6. For non-Japanese/English languages, transliterate to Japanese, then apply the above rules.  
           `슈퍼히어로` → `シュポヒオロ` → `シユホヒオロ`

  - ARTIST Artist Name Specification

    - Name of the artist

  - GENRE Genre Name Specification

    - Genre of the song
    - Note: If unspecified, the parent folder name of the song folder is used.

  - DESIGN Chart Designer Specification

    - Name of the chart designer

  - DIFF Difficulty

    - Difficulty: BASIC: 0, ADVANCED: 1, EXPERT: 2, MASTER: 3, WORLD'S END: 4, ULTIMA: 5

  - LEVEL Play Level Specification

    - Level value
      - For WORLD'S END difficulty, specify the number of stars.

  - WEATTR WORLD'S END Attribute Specification

    - Attribute (single kanji or full-width symbol)

  - CONST Chart Constant Specification

    - Constant value for the chart

  - SONGID Song ID Specification

    - SongId
      - Use the same ID for different difficulties of the same song, except WORLD'S END charts must have unique IDs.

  - RLDATE Song Release Date

    - Date added, in YYYYMMDD format

  - BGM Audio File Specification

    - FileName
      - Supported formats: WAV, MP3, OGG, M4A
      - M4A is highly recommended; keeping around 3 MB improves load speed.

  - BGMOFS Audio Offset Specification

    - OffsetTime in seconds
      - Positive: delay playback; Negative: advance playback
      - It’s recommended to set this to 0 in the chart and adjust offset at the audio level.

  - BGMPRV Audio Preview Range Specification

    - StartTime (seconds)
    - EndTime (seconds)

  - JACKET Jacket Image Specification

    - FileName
      - Supported formats: PNG, BMP, JPEG, GIF
      - GPU-compressed formats supported: DDS (BC1; no mipmaps recommended)
      - 400×400 resolution is highly recommended.

  - BGIMG Background Image Specification

    - FileName
      - Supported formats: PNG, BMP, JPEG, GIF
      - Video formats supported: MP4, AVI

  - BGSCENE Background 3D Scene Specification

    - SceneId

  - BGMODE Background Mode Settings

    - AttrName (setting name)
      - PASSIVE: ignore audio playback position and loop if shorter than chart length.
    - Value: TRUE / FALSE

  - FLDCOL Field Divider Line Color Specification

    - ColorIndex
      - -1 Auto
      - 0 White
      - 1 Red
      - 2 Orange
      - 3 Yellow
      - 4 Lime
      - 5 Green
      - 6 Teal
      - 7 Blue
      - 8 Purple

  - FLDSCENE Field Background 3D Scene Specification

    - SceneId

  - TICKS Time Resolution

    - Resolution (always 480)

  - MAINBPM Main BPM Specification

    - Bpm

  - MAINTIL Main Timeline Specification

    - TimelineId (0 recommended)

  - CLKCNT Click Sound Count

    - Count
    - Note: If omitted, plays the number of click sounds equal to the numerator of the first measure’s time signature.

  - FLAG Flag Settings

    - AttrName (flag name)
      - `DIFFTTL`: tutorial chart (always specify FALSE)
      - `SOFFSET`: insert one measure of blank space at the start
      - `CLICK`: enable click sounds
      - `EXLONG`: use ExLong
      - `BGMWCMP`: wait until audio playback finishes
      - `HIPRECISION`: use high-resolution values for AIR notes
    - Value: TRUE / FALSE

  - BPM BPM Definition

    - BarTick
    - Bpm

  - BEAT Time Signature Definition

    - Bar (measure index)
    - Numer (numerator)
    - Denom (denominator)

  - TIL Timeline Definition

    - TimelineId
    - BarTick
    - Speed

  - SPDMOD Note Speed Definition

    - BarTick
    - Speed

  - USETIL Timeline ID Selector
    - TimelineId
    - Note: Special command to assign a timeline ID for subsequent note lines.

- Note Lines

  - Overview

    - Parent Note
      - `#BarTick:txw` format
        - t: note type
        - x: horizontal position (base-36)
        - w: width (base-36)
    - Child Note
      - `#OffsetTick>txw` format
        - t: note type
        - x: horizontal position (base-36)
        - w: width (base-36)

  - CLICK

    - `#BarTick:t`
      - t = `c`

  - TAP

    - `#BarTick:txw`
      - t = `t`

  - EXTAP

    - `#BarTick:txwd`
      - t = `x`
      - d: effect
        - `U` Up
        - `D` Down
        - `C` Center
        - `A` Clockwise
        - `W` Counterclockwise
        - `L` Right
        - `R` Left
        - `I` In/Out

  - FLICK

    - `#BarTick:txwd`
      - t = `f`
      - d: auto‑play effect direction
        - `A` Auto
        - `L` Right
        - `R` Left

  - DAMAGE

    - `#BarTick:txw`
      - t = `d`

  - HOLD

    - `#BarTick:txw`
      - t = `h`
    - Child Note
      - End point
        - `#OffsetTick:t`
        - t = `s`

  - SLIDE

    - `#BarTick:txw`
      - t = `s`
    - Child Notes
      - Intermediate / End point
        - `#OffsetTick:txw`
        - t = `s`
      - Control point
        - `#OffsetTick:txw`
        - t = `c`

  - AIR

    - `#BarTick:txwddc`
      - t = `a`
      - dd: direction
        - `UC` Up
        - `UL` Up‑Right
        - `UR` Up‑Left
        - `DC` Down
        - `DL` Down‑Right
        - `DR` Down‑Left
      - c: color
        - `N` Normal
        - `I` Inverted

  - AIR‑HOLD

    - `#BarTick:txwc`
      - t = `H`
      - c: color
        - `N` Normal
        - `I` Inverted
    - Child Notes
      - Intermediate / End point
        - `#OffsetTick:t`
        - t = `s`
      - AIR‑ACTION no‑action end point
        - `#OffsetTick:t`
        - t = `c`

  - AIR‑SLIDE

    - `#BarTick:txwhhc`
      - t = `S`
      - hh: height (two‑digit base‑36 of height×10)
      - c: color
        - `N` Normal
        - `I` Inverted
    - Child Notes
      - Intermediate / End point
        - `#OffsetTick:txwhh`
        - t = `s`
        - hh: height (two‑digit base‑36 of height×10)
      - Control point / AIR‑ACTION no‑action end point
        - `#OffsetTick:txwhh`
        - t = `c`
        - hh: height (two‑digit base‑36 of height×10)

  - AIR‑CRUSH
    - `#BarTick:txwhhc,{interval}`
      - t = `C`
      - hh: height (two‑digit base‑36 of height×10)
      - c: color
        - `0` Normal
        - `1` Red
        - `2` Orange
        - `3` Yellow
        - `4` Yellow‑Green
        - `5` Green
        - `6` Cyan
        - `7` Sky
        - `8` Light
        - `9` Blue
        - `A` Blue‑Purple
        - `Y` Magenta
        - `B` Pink
        - `C` White
        - `D` Black
        - `Z` Transparent
      - {interval}: placement interval in decimal
        - `0` for AIR‑TRACE; `$` for only the start note generating combo.
    - Child Note
      - End point
        - `#OffsetTick:txwhh`
        - t = `c`
        - hh: height (two‑digit base‑36 of height×10)
