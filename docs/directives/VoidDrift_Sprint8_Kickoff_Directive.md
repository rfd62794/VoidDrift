# VoidDrift — Sprint 8 Kickoff Directive
**Date:** May 2026  
**Scope:** Three sequential tasks. Complete each before starting the next. Do not parallelize.

---

## Task 1: verify.ps1 — Self-Correcting Build Harness

Create `verify.ps1` in the VoidDrift repo root.

```powershell
# verify.ps1 — Run cargo check and cargo test, capture output
# Usage: .\verify.ps1

$ErrorActionPreference = "Continue"
$outputFile = "context/last_error.txt"

New-Item -ItemType Directory -Force -Path "context" | Out-Null

Write-Host "Running cargo check..." -ForegroundColor Cyan
$checkOutput = cargo check 2>&1
$checkExit = $LASTEXITCODE

Write-Host "Running cargo test..." -ForegroundColor Cyan
$testOutput = cargo test 2>&1
$testExit = $LASTEXITCODE

$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

if ($checkExit -eq 0 -and $testExit -eq 0) {
    $result = "Sprint 8 verification pass. No errors found. [$timestamp]"
    Write-Host $result -ForegroundColor Green
} else {
    $result = @"
VERIFICATION FAILED [$timestamp]

--- cargo check ---
$checkOutput

--- cargo test ---
$testOutput
"@
    Write-Host "Errors found. See context/last_error.txt" -ForegroundColor Red
}

$result | Out-File -FilePath $outputFile -Encoding utf8
Write-Host "Output written to $outputFile"
```

**Acceptance:** Running `.\verify.ps1` on clean codebase produces `context/last_error.txt` containing "Sprint 8 verification pass."

---

## Task 2: RFD-Telemetry — FastAPI Hello World

Create a new directory `rfd-telemetry/` in the VoidDrift repo root (or as a sibling repo if preferred).

### 2a: Project structure

```
rfd-telemetry/
  main.py
  requirements.txt
  .env.example
```

### 2b: requirements.txt

```
fastapi==0.111.0
uvicorn==0.29.0
python-dotenv==1.0.1
```

### 2c: main.py

```python
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from datetime import datetime
import uuid

app = FastAPI(title="RFD-Telemetry", version="0.1.0")

app.add_middleware(
    CORSMiddleware,
    allow_origins=[
        "https://rdug627.itch.io",
        "http://localhost:8080",
        "https://voidrift.rfditservices.com",
    ],
    allow_methods=["POST", "OPTIONS"],
    allow_headers=["Content-Type", "X-RFD-Client-Version"],
    max_age=86400,
)

class Event(BaseModel):
    event_type: str
    timestamp: str
    session_id: str
    client_version: str = "unknown"
    platform: str = "unknown"
    meta: dict = {}

events_log = []  # In-memory for now — SQLite in Sprint 8 proper

@app.get("/health")
def health():
    return {"status": "nominal", "timestamp": datetime.utcnow().isoformat()}

@app.post("/v1/event")
def receive_event(event: Event):
    event_id = str(uuid.uuid4())
    events_log.append({"id": event_id, **event.dict()})
    print(f"[EVENT] {event.event_type} | session={event.session_id} | platform={event.platform}")
    return {"status": "accepted", "event_id": event_id}
```

### 2d: Verify

```powershell
cd rfd-telemetry
pip install -r requirements.txt --break-system-packages
uvicorn main:app --reload --port 8000
```

Then in a second terminal:
```powershell
curl http://localhost:8000/health
```

**Acceptance:** Health endpoint returns `{"status": "nominal", ...}`. POST to `/v1/event` returns `{"status": "accepted", ...}`.

---

## Task 3: GitHub CLI — Issue and Project Scripts

Create `scripts/gh_tools.ps1` in the VoidDrift repo root.

```powershell
# gh_tools.ps1 — GitHub CLI helpers for VoidDrift project management
# Usage: . .\scripts\gh_tools.ps1

$REPO = "rfd62794/VoidDrift"
$PROJECT_NUMBER = 1  # Update if project number differs

function List-Issues {
    gh issue list --repo $REPO --state open
}

function View-Issue {
    param([int]$Number)
    gh issue view $Number --repo $REPO
}

function Close-Issue {
    param([int]$Number, [string]$Comment = "Complete.")
    gh issue close $Number --repo $REPO --comment $Comment
}

function Add-Issue {
    param(
        [string]$Title,
        [string]$Body,
        [string]$Label = ""
    )
    if ($Label) {
        gh issue create --repo $REPO --title $Title --body $Body --label $Label
    } else {
        gh issue create --repo $REPO --title $Title --body $Body
    }
}

function List-ProjectItems {
    gh project item-list $PROJECT_NUMBER --owner rfd62794
}

Write-Host "gh_tools loaded. Commands: List-Issues, View-Issue, Close-Issue, Add-Issue, List-ProjectItems" -ForegroundColor Cyan
```

**Acceptance:** `. .\scripts\gh_tools.ps1` loads without error. `List-Issues` returns the 14 open issues.

---

## Completion Order

1. `verify.ps1` passes on clean build
2. FastAPI health endpoint responds
3. `gh_tools.ps1` loads and `List-Issues` returns results

Do not begin Task 2 until Task 1 passes. Do not begin Task 3 until Task 2 passes.

---

*VoidDrift Sprint 8 Kickoff*  
*May 2026 — RFD IT Services Ltd.*
