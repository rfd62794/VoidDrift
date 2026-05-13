# OBS Demo Recording Setup

## Scene settings
- Canvas: 720x1280 (portrait)
- Output: 720x1280
- FPS: 30

## Source: Window Capture
- Source type: Window Capture
- Window: Chrome (http://localhost:8080)
- Capture method: Windows 10 (1.1+)
- Client area only: YES

## Output settings
- Format: MP4
- Encoder: NVENC H.264 (hardware)
- Rate control: CQP, value 18
- Save to: C:/Github/VoidDrift/raw_demo.mp4

## Shot list (90 seconds)
- 0:00 Loading + starfield (5s)
- 0:05 Tap asteroid, drone dispatches (10s)
- 0:15 Fleet in motion, 3-4 drones (10s)
- 0:25 Open drawer, CARGO tab (10s)
- 0:35 Signal bottle appears, collect (10s)
- 0:45 Echo narrative scrolling (10s)
- 0:55 PIPELINE tree open (10s)
- 1:05 Title hold (5s)

## After recording
Run: .\scripts\trim_demo.ps1 -Input raw_demo.mp4 -DurationSec 90

## FFmpeg

FFmpeg location: `C:\Github\GameReviewAgent\content-engine\ffmpeg.exe`

Not on system PATH - either add to PATH or update `trim_demo.ps1` to hardcode this path.

