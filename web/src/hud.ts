import type { GameState } from './types.ts';
import { C } from './constants.ts';

// ── DOM element references ──────────────────────────────────────────────────

let elIronOre: HTMLElement;
let elIronIngots: HTMLElement;
let elHullPlates: HTMLElement;
let elDroneCount: HTMLElement;
let elElapsed: HTMLElement;

let elRefineryBar: HTMLElement;
let elRefineryRate: HTMLElement;
let elForgeBar: HTMLElement;
let elForgeRate: HTMLElement;
let elBuildBar: HTMLElement;
let elBuildStatus: HTMLElement;

let elPanel: HTMLElement;
let panelOpen = false;

// ── Build HUD DOM ───────────────────────────────────────────────────────────

export function buildHUD(): void {
  const hud = document.createElement('div');
  hud.id = 'hud';
  hud.innerHTML = `
    <div id="resource-strip">
      <div class="res-item">
        <span class="res-icon">⛏</span>
        <span class="res-label">ORE</span>
        <span class="res-val" id="iron-ore">0</span>
      </div>
      <div class="res-item">
        <span class="res-icon">🔩</span>
        <span class="res-label">INGOTS</span>
        <span class="res-val" id="iron-ingots">0.0</span>
      </div>
      <div class="res-item">
        <span class="res-icon">🛡</span>
        <span class="res-label">HULLS</span>
        <span class="res-val" id="hull-plates">0.0</span>
      </div>
      <div class="res-item">
        <span class="res-icon">🚀</span>
        <span class="res-label">DRONES</span>
        <span class="res-val" id="drone-count">1</span>
      </div>
    </div>

    <div id="handle-bar" role="button" aria-label="Toggle production panel">
      <span id="handle-label">PRODUCTION ▲</span>
      <span id="elapsed-time">00:00</span>
    </div>

    <div id="production-panel">
      <div class="prod-section">
        <div class="prod-header">
          <span class="prod-title">REFINERY</span>
          <span class="prod-subtitle">Iron Ore → Iron Ingots (10:1)</span>
          <span class="prod-rate" id="refinery-rate"></span>
        </div>
        <div class="bar-track">
          <div class="bar-fill bar-refinery" id="refinery-bar"></div>
        </div>
      </div>

      <div class="prod-section">
        <div class="prod-header">
          <span class="prod-title">FORGE</span>
          <span class="prod-subtitle">Iron Ingots (×2) → Hull Plate</span>
          <span class="prod-rate" id="forge-rate"></span>
        </div>
        <div class="bar-track">
          <div class="bar-fill bar-forge" id="forge-bar"></div>
        </div>
      </div>

      <div class="prod-section">
        <div class="prod-header">
          <span class="prod-title">DRONE BAY</span>
          <span class="prod-subtitle">Hull Plates (×3) → Drone (18s)</span>
          <span class="prod-rate" id="build-status"></span>
        </div>
        <div class="bar-track">
          <div class="bar-fill bar-build" id="build-bar"></div>
        </div>
      </div>

      <div id="hud-footer">
        <span class="dim-text">All production runs automatically</span>
      </div>
    </div>
  `;

  document.body.appendChild(hud);

  // Cache refs
  elIronOre      = document.getElementById('iron-ore')!;
  elIronIngots   = document.getElementById('iron-ingots')!;
  elHullPlates   = document.getElementById('hull-plates')!;
  elDroneCount   = document.getElementById('drone-count')!;
  elElapsed      = document.getElementById('elapsed-time')!;
  elRefineryBar  = document.getElementById('refinery-bar')!;
  elRefineryRate = document.getElementById('refinery-rate')!;
  elForgeBar     = document.getElementById('forge-bar')!;
  elForgeRate    = document.getElementById('forge-rate')!;
  elBuildBar     = document.getElementById('build-bar')!;
  elBuildStatus  = document.getElementById('build-status')!;
  elPanel        = document.getElementById('production-panel')!;

  // Handle bar toggle
  const handle = document.getElementById('handle-bar')!;
  const handleLabel = document.getElementById('handle-label')!;
  handle.addEventListener('click', () => {
    panelOpen = !panelOpen;
    elPanel.classList.toggle('open', panelOpen);
    handleLabel.textContent = panelOpen ? 'PRODUCTION ▼' : 'PRODUCTION ▲';
  });
}

// ── Update HUD every frame ──────────────────────────────────────────────────

function fmt(n: number, dec = 0): string {
  return n.toFixed(dec);
}

function fmtTime(secs: number): string {
  const m = Math.floor(secs / 60);
  const s = Math.floor(secs % 60);
  return `${String(m).padStart(2, '0')}:${String(s).padStart(2, '0')}`;
}

function pct(val: number): string {
  return `${Math.min(100, Math.round(val * 100))}%`;
}

export function updateHUD(state: GameState): void {
  elIronOre.textContent    = fmt(state.ironOre, 0);
  elIronIngots.textContent = fmt(state.ironIngots, 1);
  elHullPlates.textContent = fmt(state.hullPlates, 1);
  elDroneCount.textContent = `${state.drones.length} / ${C.DRONE_MAX_ACTIVE}`;
  elElapsed.textContent    = fmtTime(state.elapsedSeconds);

  // Refinery bar — approximates cycle progress via ingot fraction of production
  const refineryActive = state.ironOre > 0;
  const refineryFrac = refineryActive
    ? (state.ironIngots % 1 + state.refineryProgress) / 2
    : 0;
  elRefineryBar.style.width  = refineryActive ? '100%' : '0%';
  elRefineryBar.style.opacity = refineryActive ? '1' : '0.3';
  elRefineryRate.textContent  = refineryActive
    ? `+${(1 / C.IRON_TIME).toFixed(2)} ingots/s`
    : 'idle — no ore';

  // Forge bar
  const forgeActive = state.ironIngots >= 0.01;
  elForgeBar.style.width   = forgeActive ? '100%' : '0%';
  elForgeBar.style.opacity = forgeActive ? '1' : '0.3';
  elForgeRate.textContent  = forgeActive
    ? `+${(1 / C.HULL_TIME).toFixed(2)} hulls/s`
    : 'idle — no ingots';

  // Drone build bar
  const buildActive = state.hullPlates >= C.DRONE_COST_HULLS && state.drones.length < C.DRONE_MAX_ACTIVE;
  const buildProgress = buildActive ? state.droneBuildProgress : 0;
  elBuildBar.style.width  = pct(buildProgress);

  if (state.drones.length >= C.DRONE_MAX_ACTIVE) {
    elBuildStatus.textContent = 'fleet at max capacity';
    elBuildBar.style.opacity  = '0.3';
  } else if (buildActive) {
    const remaining = C.DRONE_BUILD_TIME * (1 - state.droneBuildProgress);
    elBuildStatus.textContent = `building… ${remaining.toFixed(0)}s`;
    elBuildBar.style.opacity  = '1';
  } else {
    const needed = Math.max(0, C.DRONE_COST_HULLS - state.hullPlates);
    elBuildStatus.textContent = `needs ${needed.toFixed(1)} more hulls`;
    elBuildBar.style.opacity  = '0.3';
  }
}
