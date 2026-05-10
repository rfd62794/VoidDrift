# VoidDrift — Dev Branch Infrastructure Directive
**Date:** May 2026
**Branch:** `dev`
**Scope:** Three independent tasks. Complete in order. No game logic changes.

---

## Task 1: GitHub Issue — TD-001 HUD God Class

Using `gh_tools.ps1`, create Issue #16:

```powershell
Add-Issue `
  -Title "TD-001: hud/mod.rs God Class Refactor" `
  -Body "hud/mod.rs is ~800 lines handling panel registration, cargo bay, forge, quests, logs, and tutorial overlay in a single file. This is the primary source of layout cascade failures and makes targeted fixes risky.

**Refactor plan:**
Split into separate modules under src/systems/ui/hud/:
- mod.rs — panel registration order only (~150 lines)
- cargo.rs — cargo bay chain layout
- forge.rs — forge tab
- quests.rs — quests tab  
- logs.rs — logs tab
- tutorial_overlay.rs — tutorial popup rendering

**Why:** Every layout change today required understanding the entire file. Split modules allow targeted edits with no cascade risk.

**Approach:** Dev branch only. Slow incremental split between sprints. No behavioral changes — pure structural refactor.

**Acceptance:** All tabs render identically after refactor. verify.ps1 passes."
```

---

## Task 2: Local Itch.io Preview File

Create `scripts/local_itch_preview.html`:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>VoidDrift — Local Itch Preview</title>
    <style>
        body {
            background: #eeeeee;
            display: flex;
            flex-direction: column;
            align-items: center;
            padding: 20px;
            font-family: sans-serif;
        }
        h2 {
            color: #333;
            margin-bottom: 8px;
        }
        .label {
            font-size: 12px;
            color: #666;
            margin-bottom: 8px;
        }
        .frame-wrapper {
            background: white;
            padding: 10px;
            border-radius: 4px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.15);
        }
        iframe {
            border: none;
            display: block;
        }
        .controls {
            margin-top: 12px;
            display: flex;
            gap: 12px;
        }
        button {
            padding: 8px 16px;
            cursor: pointer;
            border-radius: 4px;
            border: 1px solid #ccc;
            background: white;
        }
        button.active {
            background: #fa5c5c;
            color: white;
            border-color: #fa5c5c;
        }
    </style>
</head>
<body>
    <h2>VoidDrift — Local Itch.io Preview</h2>
    <div class="label" id="size-label">Embedded: 1280 × 640 (landscape)</div>
    <div class="frame-wrapper">
        <iframe
            id="game-frame"
            src="http://localhost:8080/index.html"
            width="1280"
            height="640">
        </iframe>
    </div>
    <div class="controls">
        <button class="active" onclick="setSize(1280, 640, 'Embedded: 1280 × 640 (landscape)', this)">
            Landscape 1280×640
        </button>
        <button onclick="setSize(720, 640, 'Embedded: 720 × 640 (portrait-ish)', this)">
            Portrait 720×640
        </button>
        <button onclick="setSize(720, 1280, 'Embedded: 720 × 1280 (full portrait)', this)">
            Full Portrait 720×1280
        </button>
        <button onclick="setSize(window.innerWidth - 40, window.innerHeight - 140, 'Fullscreen simulation', this)">
            Fullscreen Sim
        </button>
    </div>
    <script>
        function setSize(w, h, label, btn) {
            document.getElementById('game-frame').width = w;
            document.getElementById('game-frame').height = h;
            document.getElementById('size-label').textContent = label;
            document.querySelectorAll('button').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
        }
    </script>
</body>
</html>
```

**Usage:** Run `.\run.ps1` to start the game server, then open `scripts/local_itch_preview.html` in a browser. Switch between preset sizes to test different itch.io embed configurations without publishing.

---

## Task 3: Dev Branch Setup

```powershell
# Create dev branch from current main
git checkout -b dev
git push origin dev

# Verify
git branch -a
```

Confirm `dev` branch exists on origin before proceeding.

---

## Task 4: First Dev Branch Touch — Begin hud/mod.rs Split

On the `dev` branch only, extract the cargo bay rendering into its own file. This is the first step of TD-001.

**Create** `src/systems/ui/hud/cargo.rs`:
- Move the `render_cargo_tab` function and all cargo-specific helpers from `hud/mod.rs`
- Add `pub mod cargo;` to `hud/mod.rs`
- Call `cargo::render_cargo_tab(...)` from `hud/mod.rs` where it previously lived inline

**Acceptance:**
- [ ] `.\verify.ps1` passes on dev branch
- [ ] Cargo tab renders identically — no visual change
- [ ] `hud/mod.rs` line count reduced by cargo function size
- [ ] No changes to main branch

---

## Completion

```powershell
git add -A
git commit -m "infra: dev branch setup, local itch preview, TD-001 cargo tab extraction"
git push origin dev
```

Comment on Issue #16: "TD-001 started on dev branch. cargo.rs extracted. hud/mod.rs reduced."

Do NOT merge to main. Dev branch only.

---

*VoidDrift Dev Infrastructure Directive*
*May 2026 — RFD IT Services Ltd.*
