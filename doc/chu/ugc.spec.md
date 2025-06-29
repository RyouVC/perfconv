# Umiguri Chart v8 Specification

- Basics
  - File extension is `.ugc`.
  - Character encoding is UTF-8; line endings can be LF or CRLF.
  - One command/note per line.
  - Lines not starting with `@` or `#` are ignored.
  - Lines missing parameters are ignored.
  - Comment lines begin with `'`.
  - Time is specified in `Bar'Tick` format, referred to as BarTick format.
    Example: Bar 0, Tick 240 → `0'240`

- Header Lines
  - Parameters are separated by horizontal tabs.
  - Command list and arguments:

  - `VER` Version
    - Version (always 8)

  - `EXVER` Extended version
    - ExVersion 0 or 1
      - `1` forces `FLAG:EXLONG` to `TRUE`
      - Always `1` in UMIGURI NEXT

  - `TITLE` Song title
    - Title SongTitle

  - `SORT` Sort key
    - Key SortKey
      - Used for ordering when sorting by title:
        1. Convert letters to uppercase: `magnet` → `MAGNET`
        2. Remove symbols/spaces: `Miracle∞Hinacle` → `MIRACLEHINACLE`
        3. Convert hiragana to katakana, remove diacritics, small chars → large, long vowel → `ウ`: `かーてんこーる!!!!!` → `カウテンコウル`
        4. Convert kanji to readings and apply above rules: `幻想症候群` → `ケンソウシントロウム`
        5. Convert leet to correct form and apply above: `^/7(L?[_(L#<>+&l^(o)` → `NYARLATHOTEP`
        6. For non-Japanese/English: transliterate to Japanese then apply above: `슈퍼히어로` → `シユホヒオロ`

  - `ARTIST` Artist name
    - Artist ArtistName

  - `GENRE` Genre name
    - Genre GenreName
    - Note: If unspecified, parent folder name is used.

  - `DESIGN` Chart designer
    - Designer DesignerName

  - `DIFF` Difficulty
    - Difficulty BASIC: 0, ADVANCED: 1, EXPERT: 2, MASTER: 3, WORLD'S END: 4, ULTIMA: 5

  - `LEVEL` Play level
    - Level LevelValue
    - For WORLD'S END, this is the star count.

  - `WEATTR` WORLD'S END attribute
    - Attribute (kanji or one full-width symbol)

  - `CONST` Chart constant
    - Constant Value

  - `SONGID` Song ID
    - SongId IDValue
    - Use the same ID for all difficulties of the same song. WORLD'S END requires a different ID.

  - `RLDATE` Release date
    - Date in `YYYYMMDD` format

  - `BGM` Audio file
    - FileName
      - Supported: WAV, MP3, OGG, M4A (recommended, ~3MB for fast loading)

  - `BGMOFS` Audio offset
    - OffsetTime in seconds
      - Positive: delay playback, Negative: start early
      - Recommended to set offset to 0 and adjust waveform externally

  - `BGMPRV` Audio preview range
    - StartTime (seconds)
    - EndTime (seconds)

  - `JACKET` Jacket image
    - FileName
      - Supported: PNG, BMP, JPEG, GIF
      - Also GPU formats: DDS (BC1, no mipmaps recommended)
      - Recommended resolution: 400x400

  - `BGIMG` Background image
    - FileName
      - Supported: PNG, BMP, JPEG, GIF
      - Also supports video: MP4, AVI

  - `BGSCENE` Background 3D scene
    - SceneId

  - `BGMODE` Background mode
    - AttrName
      - `PASSIVE` - ignores audio playback position, loops if short
    - Value: TRUE / FALSE

  - `FLDCOL` Field divider color
    - ColorIndex:
      - -1 Auto, 0 White, 1 Red, 2 Orange, 3 Yellow, 4 Lime, 5 Green,
        6 Teal, 7 Blue, 8 Purple

  - `FLDSCENE` Field background 3D scene
    - SceneId

  - `TICKS` Time resolution
    - Resolution (always 480)

  - `MAINBPM` Main BPM
    - Bpm Value

  - `MAINTIL` Main timeline
    - TimelineId (0 recommended)

  - `CLKCNT` Click sound count
    - Count
    - Note: Defaults to numerator of first time signature

  - `FLAG` Flag settings
    - AttrName:
      - `DIFFTTL`: Tutorial chart (always FALSE)
      - `SOFFSET`: Insert 1-bar blank at start
      - `CLICK`: Play click sound
      - `EXLONG`: Enable ExLong
      - `BGMWCMP`: Wait for audio to finish
      - `HIPRECISION`: High precision AIR note values
    - Value: TRUE / FALSE

  - `BPM` BPM definition
    - BarTick
    - Bpm

  - `BEAT` Time signature definition
    - Bar
    - Numerator
    - Denominator

  - `TIL` Timeline definition
    - TimelineId
    - BarTick
    - Speed

  - `SPDMOD` Note speed definition
    - BarTick
    - Speed

  - `USETIL` Timeline ID assignment
    - TimelineId
    - Notes following this line use the given timeline ID

- Note Lines
  - Basics
    - Parent notes:
      - `#BarTick:txw`
        - t: note type
        - x: horizontal position (base36)
        - w: width (base36)
    - Child notes:
      - `#OffsetTick:txw`
        - Offset from parent note
        - Same format as above

  - CLICK: `#BarTick:c`
  - TAP: `#BarTick:txw` (t = `t`)
  - EXTAP: `#BarTick:txwd` (t = `x`, d = U/D/C/A/W/L/R/I)
  - FLICK: `#BarTick:txwd` (t = `f`, d = A/L/R)
  - DAMAGE: `#BarTick:txw` (t = `d`)
  - HOLD:
    - Parent: `#BarTick:txw` (t = `h`)
    - End: `#OffsetTick:s`
  - SLIDE:
    - Parent: `#BarTick:txw` (t = `s`)
    - Intermediate/End: `#OffsetTick:sxw`
    - Control point: `#OffsetTick:cxw`
  - AIR:
    - `#BarTick:txwddc`
      - dd = direction (UC/UL/UR/DC/DL/DR)
      - c = color (N/I)
  - AIR-HOLD:
    - Parent: `#BarTick:txwc` (t = `H`)
      - c = color (N/I)
    - Intermediate/End: `#OffsetTick:s`
    - AIR-ACTION free end: `#OffsetTick:c`
  - AIR-SLIDE:
    - Parent: `#BarTick:txwhhc`
      - hh = height ×10 in base36
      - c = color (N/I)
    - Intermediate/End: `#OffsetTick:sxwhh`
    - Control/Free end: `#OffsetTick:cxwhh`
  - AIR-CRUSH:
    - Parent: `#BarTick:txwhhc,{interval}`
      - hh = height ×10 in base36
      - c = color: `0`-`9`, `A`, `Y`, `B`, `C`, `D`, `Z`
      - interval: decimal value, `0` for AIR-TRACE, `$` for combo-on-start only
    - End: `#OffsetTick:cxwhh`
