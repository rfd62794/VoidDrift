# Voidrift Development Pipeline

**Date:** May 2026
**Purpose:** Formal end-to-end pipeline from local edit to live on itch.io

---

## Overview

The development pipeline has three distinct loops:
1. **Local Development Loop** — Fast iteration on desktop, WASM testing at itch.io dimensions
2. **Commit and Tag Loop** — Feature branches, merging to main, version tagging, publishing
3. **CI/CD Loop** — Automated builds and deployment via GitHub Actions

---

## Local Development Loop

### Desktop Development (Fast Iteration)

**Purpose:** Day-to-day development with instant feedback. TOML/YAML changes don't require rebuild.

```powershell
# Start desktop build
.\run.ps1
```

**What this does:**
- Compiles desktop build (Bevy native)
- Runs game in windowed mode
- Hot-reloads config files (TOML/YAML) without rebuild
- Instant feedback for gameplay, UI, balance changes

**When to use:**
- All feature development
- Bug fixes
- Config tuning
- UI layout work
- Gameplay testing

**Verification before commit:**
```powershell
# Verify compiles and tests pass
.\verify.ps1
```

### WASM Testing at itch.io Dimensions

**Purpose:** Test WASM build in exact itch.io iframe environment before publishing.

**Step 1: Build WASM**
```powershell
.\build_wasm.ps1
```

**Step 2: Serve WASM locally**
```powershell
cd pkg
python -m http.server 8080
```

**Step 3: Open itch.io preview**
```
Open scripts/local_itch_preview.html in browser
```

**What this does:**
- Builds WASM via wasm-pack
- Serves `pkg/` directory on localhost:8080
- Opens preview page with iframe at exact itch.io dimensions (720×1280 portrait)
- Loads your WASM build inside the iframe
- Four preset sizes to test: Embed, Tablet, Mobile Portrait, Mobile Landscape

**When to use:**
- Before any publish to itch.io
- Testing WASM-specific issues (canvas sizing, fullscreen, input)
- Verifying drawer layout at constrained height (~162px available)
- Testing touch interactions on desktop

**Note:** The preview page loads from `http://localhost:8080/index.html` by default. Ensure the WASM server is running before opening the preview.

### Local Itch.io Emulation

Full itch.io browser page emulation without publishing:

**Step 1: Build WASM**
```powershell
.\build_wasm.ps1
```

**Step 2: Serve pkg/ directory**
```powershell
cd pkg
python -m http.server 8080
```

**Step 3: Open itch.io preview**
```
Open scripts/local_itch_preview.html in browser
```

**Step 4: Select preset size matching your test target**
- **Full Portrait 720×1280** — current itch.io embed setting
- **Landscape 1280×640** — legacy landscape embed
- **Portrait 720×640** — short portrait
- **Fullscreen Sim** — approximates browser fullscreen

This replicates the exact iframe constraints itch.io applies. Canvas CSS rules, EGUI_SCALE, and available_height values will match production exactly. Use this before every publish.

**Known values at Full Portrait 720×1280:**
- `ui.available_height()` in cargo tab = 162.1875px
- Canvas constrained by `max-width: 720px, max-height: 100vh`

See `docs/WASM_ITCH_SIZING.md` for complete sizing reference.

---

## Commit and Tag Loop

### Feature Development

**Branch strategy:**
- `main` — Production branch, always deployable
- `dev` — Active development branch
- Feature branches — Created from `dev` for specific work

**Workflow:**
```powershell
# Create feature branch from dev
git checkout dev
git checkout -b feature/your-feature-name

# Work on feature (use .\run.ps1 for desktop testing)
# Verify with .\verify.ps1 before commit

# Commit feature work
git add -A
git commit -m "feature: description"

# Push feature branch
git push origin feature/your-feature-name
```

### Merge to Main

**When to merge:**
- Feature is complete and tested
- Desktop build works
- WASM build works (tested via local itch preview if relevant)
- No failing tests
- Documentation updated if needed

**Merge process:**
```powershell
# Checkout main
git checkout main
git pull origin main

# Merge feature branch
git merge feature/your-feature-name

# Resolve conflicts if any
# Test merged build: .\run.ps1
# Verify: .\verify.ps1

# Push merge
git push origin main
```

### Version Tagging

**Tag format:** `v{major}.{minor}.{patch}-{descriptor}`

**Descriptors:**
- `sprint{n}-{description}` — Sprint releases (e.g., `v3.1.0-sprint5-visual-overhaul`)
- `tutorial-complete` — Milestone completions
- `wasm-polish` — WASM-specific polish
- `launch-blocker` — Critical fixes for launch

**Tagging process:**
```powershell
# Ensure you're on main and up to date
git checkout main
git pull origin main

# Create annotated tag
git tag -a v3.1.0-sprint6-space-visuals -m "Sprint 6: Space entity visuals"

# Push tag
git push origin v3.1.0-sprint6-space-visuals
```

**Semantic versioning:**
- **Major (X.0.0):** Breaking changes, major architecture shifts
- **Minor (0.X.0):** New features, significant additions
- **Patch (0.0.X):** Bug fixes, small improvements

### Publishing to itch.io

**Manual publish (immediate):**
```powershell
# Build and push to itch.io
.\publish.ps1 -Build
```

**What this does:**
- Builds desktop release
- Builds WASM via wasm-pack
- Triggers Butler push to itch.io
- Deploys to configured itch.io project

**Automated publish (via CI/CD):**
- Push tag matching `publish-*` pattern
- GitHub Actions builds WASM
- Butler pushes to itch.io automatically
- See CI/CD section below

---

## CI/CD Loop

### GitHub Actions: itch.io Deployment

**Trigger:** Push tag matching `publish-*` (e.g., `publish-sprint6`)

**Workflow:**
1. Checkout code
2. Install Rust toolchain
3. Install wasm-pack
4. Build WASM via wasm-pack
5. Install Butler CLI
6. Push to itch.io via Butler

**Configuration:** `.github/workflows/itch-deploy.yml`

**Usage:**
```powershell
# Tag for automatic deployment
git tag -a publish-sprint6 -m "Publish Sprint 6 to itch.io"
git push origin publish-sprint6
```

### GitHub Actions: Telemetry Deployment

**Trigger:** Push changes to `rfd-telemetry/` repository

**Workflow:**
1. Checkout code
2. Build FastAPI Docker image
3. Deploy to rfditservices.com
4. Restart service

**Configuration:** `.github/workflows/telemetry-deploy.yml` (in telemetry repo)

**Usage:**
```powershell
# In telemetry repository
git add -A
git commit -m "deploy: telemetry update"
git push origin main
```

---

## Verification Checklist

### Before Commit
- [ ] Code compiles: `.\verify.ps1` passes
- [ ] Tests pass: `cargo test` passes
- [ ] Desktop build works: `.\run.ps1` runs without errors
- [ ] WASM build works (if relevant): `.\build_wasm.ps1` succeeds
- [ ] WASM tested at itch.io dimensions (if relevant): local itch preview passes

### Before Merge to Main
- [ ] All "Before Commit" items pass
- [ ] Feature is complete and tested
- [ ] No failing tests
- [ ] Documentation updated if needed
- [ ] Code reviewed (if working with team)

### Before Publish
- [ ] All "Before Merge to Main" items pass
- [ ] Version tag created
- [ ] Tag message describes changes
- [ ] WASM tested via local itch preview
- [ ] Desktop build verified on target hardware (if possible)

---

## Common Workflows

### Quick Bug Fix
```powershell
# Start desktop dev
.\run.ps1

# Fix bug, test locally

# Verify
.\verify.ps1

# Commit
git add -A
git commit -m "fix: description"
git push origin main
```

### Feature Development
```powershell
# Create feature branch
git checkout dev
git checkout -b feature/your-feature

# Develop with .\run.ps1
# Test WASM with local itch preview if needed

# Verify
.\verify.ps1

# Commit and push
git add -A
git commit -m "feature: description"
git push origin feature/your-feature

# Create PR or merge to dev
```

### Sprint Release
```powershell
# Complete sprint work on dev
# Merge dev to main
git checkout main
git pull origin main
git merge dev
.\verify.ps1

# Test WASM via local itch preview
.\build_wasm.ps1
cd pkg
python -m http.server 8080
# Open scripts/local_itch_preview.html

# Tag sprint
git tag -a v3.1.0-sprint7 -m "Sprint 7: description"
git push origin v3.1.0-sprint7

# Publish (manual or via CI/CD)
.\publish.ps1 -Build
# OR
git tag -a publish-sprint7 -m "Publish Sprint 7"
git push origin publish-sprint7
```

### WASM Debugging
```powershell
# Build WASM
.\build_wasm.ps1

# Serve locally
cd pkg
python -m http.server 8080

# Open itch preview
# Open scripts/local_itch_preview.html

# Iterate: make changes, rebuild, refresh browser
# No need to stop/start server between rebuilds
```

---

## Troubleshooting

### WASM Build Fails
- Check Rust version: `rustc --version` (should match toolchain)
- Check wasm-pack installed: `wasm-pack --version`
- Clear pkg directory: `Remove-Item -Recurse -Force pkg`
- Rebuild: `.\build_wasm.ps1`

### Local itch Preview Not Loading
- Ensure WASM server is running: `cd pkg; python -m http.server 8080`
- Check browser console for errors
- Verify pkg/index.html exists
- Try hard refresh: Ctrl+Shift+R

### Butler Push Fails
- Check Butler installed: `butler --version`
- Check itch.io API key configured
- Verify project slug in publish.ps1
- Check network connection

### Desktop Build Fails
- Check Bevy version in Cargo.toml (should be 0.15.3)
- Check native dependencies (Android NDK if building Android)
- Clear target directory: `cargo clean`
- Rebuild: `.\run.ps1`

---

## Related Documentation

- `docs/DEVELOPER.md` — Developer onboarding guide
- `docs/WASM_BUILD.md` — WASM build details and local server setup
- `docs/ARCHITECTURE.md` — Codebase architecture
- `docs/ADR/` — Architecture Decision Records
- `docs/state/current.md` — Current project state

---

## Quick Reference

| Command | Purpose |
| :--- | :--- |
| `.\run.ps1` | Start desktop build (fast iteration) |
| `.\verify.ps1` | Verify compiles and tests pass |
| `.\build_wasm.ps1` | Build WASM for web deployment |
| `.\publish.ps1 -Build` | Manual publish to itch.io |
| `cd pkg; python -m http.server 8080` | Serve WASM locally |
| `scripts/local_itch_preview.html` | Itch.io iframe preview |
| `git tag -a v{version} -m "message"` | Create version tag |
| `git push origin {tag}` | Push tag to remote |
