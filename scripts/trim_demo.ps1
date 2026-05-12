param(
    [string]$InputFile = "raw_demo.mp4",
    [string]$Output = "voidrift_demo.mp4",
    [int]$StartSec = 0,
    [int]$DurationSec = 90
)

ffmpeg -i $InputFile `
    -ss $StartSec `
    -t $DurationSec `
    -vf "scale=720:1280:force_original_aspect_ratio=decrease" `
    -c:v libx264 -crf 18 -preset slow `
    -c:a aac -b:a 128k `
    $Output

Write-Host "Output: $Output"
