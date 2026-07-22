import { C } from './constants.ts';
import { loadGame } from './save.ts';
import { saveGame } from './save.ts';
import { spawnAsteroid } from './state.ts';
import { updateAsteroids } from './systems/asteroids.ts';
import { updateDrones } from './systems/drones.ts';
import { updateRefinery, updateForge, updateDroneBuild } from './systems/production.ts';
import { render } from './renderer.ts';
import { buildHUD, updateHUD } from './hud.ts';
import type { GameState } from './types.ts';

// ── Canvas setup ─────────────────────────────────────────────────────────────

const canvas = document.getElementById('game-canvas') as HTMLCanvasElement;
const ctx = canvas.getContext('2d')!;

function resizeCanvas(): void {
  const dpr = window.devicePixelRatio || 1;
  const W = window.innerWidth;
  const H = window.innerHeight;
  canvas.style.width  = `${W}px`;
  canvas.style.height = `${H}px`;
  canvas.width  = Math.round(W * dpr);
  canvas.height = Math.round(H * dpr);
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
}

// ── Game loop ────────────────────────────────────────────────────────────────

const MAX_DT = 0.1; // cap to prevent spiral-of-death on tab blur
let lastTime = 0;
let state: GameState;

function gameLoop(timestamp: number): void {
  const dt = Math.min((timestamp - lastTime) / 1000, MAX_DT);
  lastTime = timestamp;

  // Systems
  state.elapsedSeconds += dt;
  state.stationRotation += C.STATION_ROTATION_SPEED * dt;
  updateAsteroids(state, dt);
  updateDrones(state, dt);
  updateRefinery(state, dt);
  updateForge(state, dt);
  updateDroneBuild(state, dt);

  // Render + HUD
  render(state, canvas, ctx, timestamp);
  updateHUD(state);

  // Autosave every ~5s at 60fps
  state.tick++;
  if (state.tick % C.AUTOSAVE_INTERVAL === 0) {
    saveGame(state);
  }

  requestAnimationFrame(gameLoop);
}

// ── Boot ─────────────────────────────────────────────────────────────────────

function boot(): void {
  buildHUD();
  state = loadGame();

  // Seed asteroids immediately on start
  while (state.asteroids.length < C.MAX_ASTEROIDS) {
    state.asteroids.push(spawnAsteroid());
  }

  resizeCanvas();
  window.addEventListener('resize', resizeCanvas);

  requestAnimationFrame((t) => {
    lastTime = t;
    requestAnimationFrame(gameLoop);
  });
}

boot();
