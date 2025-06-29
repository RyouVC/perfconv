# Umiguri Chart v8 仕様

- 基本事項
  - 拡張子は `.ugc` です。
  - 文字コードは UTF-8、改行文字は LF もしくは CRLF を使用します。
  - 1 つの行で 1 つのコマンド / ノーツを表します。
  - `@` か `#` で始まらない行は無視されます。
  - パラメーターが不足している行は無視されます。
  - コメント行は `'` で始めます。
  - 時間の指定には `Bar'Tick` の形式が使用されます。ここでは BarTick 形式と呼びます。
    例: 0 小節 240 Tick 目 → `0'240`

- ヘッダー行
  - パラメーター間は水平タブで区切ります。
  - 以下コマンド一覧と引数
  - VER バージョン指定
    - Version (常に 8)

  - EXVER 拡張バージョン指定
    - ExVersion 0 または 1
      - `1` にすると、`FLAG:EXLONG` が強制的に `TRUE` になった場合と同じ挙動をします。
      - UMIGURI NEXT では常に `1` です。

  - TITLE 曲名指定
    - Title 曲名

  - SORT 並び替えキー指定
    - Key 並び替えキー
      - 曲名で並び替えるときに順序を決める並び替え用のキーです。曲名を以下の規則に従って変換したものを指定します。
        1. 英字は大文字にする。<br>`magnet` → `MAGNET`
        2. 空白含む記号類は除去する。<br>`Miracle∞Hinacle` → `MIRACLEHINACLE`
        3. ひらがなはカタカナにし、濁点半濁点は外し、小文字は大文字にし、長音は `ウ` に置き換える。<br>`かーてんこーる!!!!!` → `カウテンコウル`
        4. 漢字は読みに直して上記規則を適用する。<br>`幻想症候群` → `ケンソウシントロウム`
        5. leet 表記は記号も含めて本来の表記に書き換えて上記規則を適用する。<br>`^/7(L?[_(L#<>+&l^(o)` → `NYARLATHOTEP`
        6. 日本語、英語以外の言語の場合は日本語に音訳した上で上記規則を適用する。<br>`슈퍼히어로` → `シュポヒオロ` → `シユホヒオロ`

  - ARTIST アーティスト名指定
    - Artist アーティスト名

  - GENRE ジャンル名指定
    - Genre ジャンル名
    - 備考: 指定がなければ楽曲フォルダーの親フォルダーの名前が使用されます。

  - DESIGN 譜面制作者指定
    - Designer 譜面制作者

  - DIFF 難易度
    - Difficulty BASIC: 0, ADVANCED: 1, EXPERT: 2, MASTER: 3, WORLD'S END: 4, ULTIMA: 5

  - LEVEL プレイ レベル指定
    - Level プレイ レベル
      - 難易度 WORLD'S END ならば星の数になります。

  - WEATTR WORLD'S END 属性指定
    - Attribute 属性
      - 漢字もしくは全角記号一文字で指定します。

  - CONST 譜面定数指定
    - Constant 譜面定数

  - SONGID 楽曲 ID 指定
    - SongId 楽曲 ID
      - 同一楽曲で異なる難易度は同じ楽曲 ID で揃えます。ただし、WORLD'S END のみは異なる ID を指定する必要があります。

  - RLDATE 楽曲追加日
    - Date 追加日 YYYYMMDD 形式で指定

  - BGM 音源指定
    - FileName
      - 次の形式に対応しています: WAV、MP3、OGG、M4A
      - M4A がとてもおすすめです。容量が 3 MB 程度に収まると読み込み速度の面でかなりよいです。

  - BGMOFS 音源オフセット指定
    - OffsetTime 秒単位のオフセット
      - 正: 再生を遅らせる、負: 再生を早める
      - 譜面ではオフセットに 0 を指定し、音源側でオフセットを波形レベルで調整することを推奨します。

  - BGMPRV 音源試聴範囲指定
    - StartTime 開始位置 (秒)
    - EndTime 開始位置 (秒)

  - JACKET ジャケット画像指定
    - FileName
      - 次の形式に対応しています: PNG、BMP、JPEG、GIF
      - GPU 圧縮形式にも対応しています: DDS (BC1、Mipmap 無し推奨)
      - 解像度は 400x400 がとてもおすすめです

  - BGIMG 背景画像指定
    - FileName
      - 次の形式に対応しています: PNG、BMP、JPEG、GIF
      - 動画も指定することができます: MP4、AVI

  - BGSCENE 背景 3D シーン指定
    - SceneId

  - BGMODE 背景モード設定
    - AttrName 属性名
      - PASSIVE 音源の再生位置を無視して再生するかどうか。尺が足りない場合は繰り返し再生されます。
    - Value 設定値 TRUE / FALSE

  - FLDCOL フィールド分割線色指定
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

  - FLDSCENE フィールド背景 3D シーン指定
    - SceneId

  - TICKS 時間解像度
    - Resolution 常に 480

  - MAINBPM 基準 BPM 指定
    - Bpm

  - MAINTIL 基準タイムライン指定
    - TimelineId タイムライン ID (0 推奨)
  
  - CLKCNT クリック音の回数
    - Count 回数
    - 備考: 省略した場合は最初の小節の拍子記号の分子の数分鳴ります。

  - FLAG フラグ設定
    - AttrName 属性名
      - `DIFFTTL` チュートリアル譜面かどうか (常に FALSE を指定してください)
      - `SOFFSET` 頭に 1 小節分の空白を挿入するかどうか
      - `CLICK` クリック音を鳴らすかどうか
      - `EXLONG` ExLong を使用するかどうか
      - `BGMWCMP` 音源が再生終了するまで待機するかどうか
      - `HIPRECISION` AIR ノーツに対して高解像度の値を使用するかどうか
    - Value 設定値 TRUE / FALSE
  
  - BPM BPM 定義
    - BarTick
    - Bpm

  - BEAT 拍定義
    - Bar 小節位置
    - Numer 分子
    - Denom 分母

  - TIL タイムライン定義
    - TimeleineId
    - BarTick
    - Speed

  - SPDMOD ノーツ速度定義
    - BarTick
    - Speed

  - USETIL タイムライン ID 指定
    - TimelineId
    - 備考: 後続のノーツ行が属するタイムライン ID を指定できる特殊コマンドです。

- ノーツ行
  - 基本
    - 親ノーツ
      - `#BarTick:txw` 形式
        - t ノーツの種類
        - x ノーツの横位置 (36 進数)
        - w ノーツの幅 (36 進数)
    - 子ノーツ
      - `#OffsetTick:txw` 形式
        - t ノーツの種類
        - x ノーツの横位置 (36 進数)
        - w ノーツの幅 (36 進数)

  - CLICK
    - `#BarTick:t`
      - t = `c`

  - TAP
    - `#BarTick:txw`
      - t = `t`
  
  - EXTAP
    - `#BarTick:txwd`
      - t = `x`
      - d エフェクト
        - `U` 上
        - `D` 下
        - `C` 中央
        - `A` 時計回り
        - `W` 反時計回り
        - `L` 右
        - `R` 左
        - `I` 内外

  - FLICK
    - `#BarTick:txwd`
      - t = `f`
      - d 自動プレイ時のエフェクト向き
        - `A` 自動判別
        - `L` 右
        - `R` 左

  - DAMAGE
    - `#BarTick:txw`
      - t = `d`

  - HOLD
    - `#BarTick:txw`
      - t = `h`
    - 子ノーツ
      - 終点
        - `#OffsetTick:t`
        - t = `s`

  - SLIDE
    - `#BarTick:txw`
      - t = `s`
    - 子ノーツ
      - 中継点 / 終点
        - `#OffsetTick:txw`
        - t = `s`
      - 制御点
        - `#OffsetTick:txw`
        - t = `c`

  - AIR
    - `#BarTick:txwddc`
      - t = `a`
      - dd 向き
        - `UC` 上
        - `UL` 右上
        - `UR` 左上
        - `DC` 下
        - `DL` 右下
        - `DR` 左下
      - c 色
        - `N` 通常
        - `I` 反転

  - AIR-HOLD
    - `#BarTick:txwc`
      - t = `H`
      - c 色
        - `N` 通常
        - `I` 反転
    - 子ノーツ
      - 中継点 / 終点
        - `#OffsetTick:t`
        - t = `s`
      - AIR-ACTION 無し終点
        - `#OffsetTick:t`
        - t = `c`

  - AIR-SLIDE
    - `#BarTick:txwhhc`
      - t = `S`
      - hh 高さ、36 進数 2 桁で元の値を 10 倍したもの
      - c 色
        - `N` 通常
        - `I` 反転
    - 子ノーツ
      - 中継点 / 終点
        - `#OffsetTick:txwhh`
        - t = `s`
        - hh 高さ、36 進数 2 桁で元の値を 10 倍したもの
      - 制御点 / AIR-ACTION 無し終点
        - `#OffsetTick:txwhh`
        - t = `c`
        - hh 高さ、36 進数 2 桁で元の値を 10 倍したもの

  - AIR-CRUSH
    - `#BarTick:txwhhc,{interval}`
      - t = `C`
      - hh 高さ、36 進数 2 桁で元の値を 10 倍したもの
      - c 色
        - `0`: 通常
        - `1`: 赤
        - `2`: 橙
        - `3`: 黄
        - `4`: 黄緑
        - `5`: 緑
        - `6`: 水
        - `7`: 空
        - `8`: 天
        - `9`: 青
        - `A`: 青紫
        - `Y`: 赤紫
        - `B`: 桃
        - `C`: 白
        - `D`: 黒
        - `Z`: 透明
      - {interval} 10 進数のノーツ配置間隔
        - 0 だと AIR-TRACE、$ だと始点だけにコンボが発生するノーツが配置されます。
    - 子ノーツ
      - 終点
        - `#OffsetTick:txwhh`
        - t = `c`
        - hh 高さ、36 進数 2 桁で元の値を 10 倍したもの
