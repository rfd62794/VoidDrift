import type { GameState, Drone } from './types.ts';
import { C } from './constants.ts';

// ─── Star layers ────────────────────────────────────────────────────────────

interface StarDot { x: number; y: number; r: number; a: number }
let starsNear: StarDot[] | null = null;
let starsFar:  StarDot[] | null = null;

function makeLCG(seed: number) {
  let s = seed >>> 0;
  return () => { s = (Math.imul(s, 1664525) + 1013904223) >>> 0; return s / 0xFFFFFFFF; };
}

function initStars(): void {
  const rFar  = makeLCG(0xDEADBEEF);
  const rNear = makeLCG(0xCAFEBABE);
  const makeLayer = (rng: () => number, count: number, spread: number, minR: number, maxR: number): StarDot[] => {
    const s: StarDot[] = [];
    for (let i = 0; i < count; i++) {
      const angle = rng() * Math.PI * 2;
      const d = 40 + rng() * spread;
      s.push({ x: Math.cos(angle) * d, y: Math.sin(angle) * d, r: minR + rng() * (maxR - minR), a: 0.25 + rng() * 0.65 });
    }
    return s;
  };
  starsFar  = makeLayer(rFar,  260, 1400, 0.5, 1.0);
  starsNear = makeLayer(rNear, 120, 900,  0.9, 1.6);
}

function worldToCanvas(wx: number, wy: number, cx: number, cy: number, scale: number): [number, number] {
  return [cx + wx * scale, cy + wy * scale];
}

// ─── Particle system (module-level, cosmetic only) ───────────────────────────

interface Particle { x: number; y: number; vx: number; vy: number; life: number; maxLife: number }
const particles: Particle[] = [];
const prevAsteroidPos = new Map<number, { x: number; y: number }>();

function emitDepletionBurst(x: number, y: number): void {
  const rng = makeLCG(Math.floor(x * 13 + y * 7 + Date.now()) >>> 0);
  for (let i = 0; i < 14; i++) {
    const angle = rng() * Math.PI * 2;
    const speed = 30 + rng() * 90;
    particles.push({
      x, y,
      vx: Math.cos(angle) * speed,
      vy: Math.sin(angle) * speed,
      life: 0.55 + rng() * 0.45,
      maxLife: 0.55 + rng() * 0.45,
    });
  }
}

function tickParticles(dt: number): void {
  for (let i = particles.length - 1; i >= 0; i--) {
    const p = particles[i];
    p.x  += p.vx * dt;
    p.y  += p.vy * dt;
    p.vx *= 0.88;
    p.vy *= 0.88;
    p.life -= dt;
    if (p.life <= 0) particles.splice(i, 1);
  }
}

// ─── Station ─────────────────────────────────────────────────────────────────

function drawStation(ctx: CanvasRenderingContext2D, cx: number, cy: number, scale: number, rotation: number, t: number): void {
  const armLen  = C.STATION_ARM_LENGTH * scale;
  const armThick = Math.max(2, C.STATION_ARM_THICKNESS * scale);
  const hubR    = C.STATION_HUB_RADIUS * scale;
  const berthR  = Math.max(4, C.STATION_BERTH_RADIUS * scale);
  const numArms = 6;

  ctx.save();
  ctx.translate(cx, cy);
  ctx.rotate(rotation);

  // Breathing glow behind hub
  const breathe = 0.5 + 0.5 * Math.sin(t * 1.1);
  const glowR   = hubR * (1.8 + breathe * 0.6);
  const glowAlpha = 0.06 + breathe * 0.10;
  const glow = ctx.createRadialGradient(0, 0, hubR * 0.5, 0, 0, glowR);
  glow.addColorStop(0, `rgba(255,220,40,${glowAlpha})`);
  glow.addColorStop(1, 'rgba(255,180,0,0)');
  ctx.beginPath();
  ctx.arc(0, 0, glowR, 0, Math.PI * 2);
  ctx.fillStyle = glow;
  ctx.fill();

  // Arms
  ctx.strokeStyle = C.COL_ARM;
  ctx.lineWidth = armThick;
  for (let i = 0; i < numArms; i++) {
    const angle = (i / numArms) * Math.PI * 2;
    ctx.beginPath();
    ctx.moveTo(0, 0);
    ctx.lineTo(Math.cos(angle) * armLen, Math.sin(angle) * armLen);
    ctx.stroke();
  }

  // Berths
  for (const bi of [0, 1, 2]) {
    const angle = (bi / numArms) * Math.PI * 2;
    const bx = Math.cos(angle) * armLen;
    const by = Math.sin(angle) * armLen;
    ctx.beginPath();
    ctx.arc(bx, by, berthR, 0, Math.PI * 2);
    ctx.fillStyle = '#444';
    ctx.fill();
    ctx.strokeStyle = '#888';
    ctx.lineWidth = Math.max(1, scale);
    ctx.stroke();
  }

  // Hub
  ctx.beginPath();
  ctx.arc(0, 0, hubR, 0, Math.PI * 2);
  const hubGrad = ctx.createRadialGradient(0, 0, hubR * 0.2, 0, 0, hubR);
  hubGrad.addColorStop(0, '#FFEE44');
  hubGrad.addColorStop(0.6, '#FFD700');
  hubGrad.addColorStop(1, '#AA8800');
  ctx.fillStyle = hubGrad;
  ctx.fill();
  ctx.strokeStyle = '#886600';
  ctx.lineWidth = Math.max(1, scale * 1.5);
  ctx.stroke();

  ctx.restore();
}

// ─── Asteroid ────────────────────────────────────────────────────────────────

function drawAsteroid(
  ctx: CanvasRenderingContext2D,
  cx: number, cy: number, scale: number,
  x: number, y: number, oreRemaining: number, maxOre: number,
  verts: Array<{ x: number; y: number }>,
): void {
  const [sx, sy] = worldToCanvas(x, y, cx, cy, scale);
  const depletion = oreRemaining / maxOre;
  const col = depletion > 0.15
    ? `rgb(${Math.round(192 * depletion + 46 * (1 - depletion))},${Math.round(96 * depletion + 46 * (1 - depletion))},${Math.round(37 * depletion + 46 * (1 - depletion))})`
    : C.COL_IRON_AST_DEPLETED;

  ctx.save();
  ctx.translate(sx, sy);

  // Ambient glow
  if (depletion > 0.1) {
    const gr = ctx.createRadialGradient(0, 0, 0, 0, 0, C.ASTEROID_RADIUS_IRON * scale * 1.7);
    gr.addColorStop(0, 'rgba(180,90,30,0.16)');
    gr.addColorStop(1, 'rgba(180,90,30,0)');
    ctx.fillStyle = gr;
    ctx.beginPath();
    ctx.arc(0, 0, C.ASTEROID_RADIUS_IRON * scale * 1.7, 0, Math.PI * 2);
    ctx.fill();
  }

  // Polygon body
  ctx.beginPath();
  ctx.moveTo(verts[0].x * scale, verts[0].y * scale);
  for (let i = 1; i < verts.length; i++) ctx.lineTo(verts[i].x * scale, verts[i].y * scale);
  ctx.closePath();
  ctx.fillStyle = col;
  ctx.fill();
  ctx.strokeStyle = depletion > 0.15 ? '#E07830' : '#555';
  ctx.lineWidth = Math.max(1, scale * 0.8);
  ctx.stroke();

  // Vein banding — 2 lighter accent lines across the polygon
  if (depletion > 0.15 && verts.length >= 4) {
    const veinCol = `rgba(230,140,70,${0.25 + depletion * 0.20})`;
    ctx.strokeStyle = veinCol;
    ctx.lineWidth = Math.max(0.5, scale * 0.6);
    // Use pairs of opposite-ish vertices
    for (let vi = 0; vi < 2; vi++) {
      const a = verts[vi + 1];
      const b = verts[(vi + 1 + Math.floor(verts.length / 2)) % verts.length];
      ctx.beginPath();
      ctx.moveTo(a.x * scale * 0.6, a.y * scale * 0.6);
      ctx.lineTo(b.x * scale * 0.6, b.y * scale * 0.6);
      ctx.stroke();
    }
  }

  // Ore bar
  if (depletion > 0) {
    const barW = C.ASTEROID_RADIUS_IRON * scale * 1.6;
    const barH = Math.max(2, scale * 2);
    const barY = -(C.ASTEROID_RADIUS_IRON * scale + barH + 3 * scale);
    ctx.fillStyle = '#333';
    ctx.fillRect(-barW / 2, barY, barW, barH);
    ctx.fillStyle = '#E07030';
    ctx.fillRect(-barW / 2, barY, barW * depletion, barH);
  }

  ctx.restore();
}

// ─── Drone ───────────────────────────────────────────────────────────────────

function drawDrone(ctx: CanvasRenderingContext2D, cx: number, cy: number, scale: number, drone: Drone, t: number): void {
  const [sx, sy] = worldToCanvas(drone.x, drone.y, cx, cy, scale);
  const w = Math.max(5, 14 * scale);
  const h = Math.max(8, 28 * scale);
  const cargoRatio = drone.cargo / C.CARGO_CAPACITY;
  const flying = drone.state === 'Outbound' || drone.state === 'Returning';

  ctx.save();
  ctx.translate(sx, sy);
  ctx.rotate(drone.headingAngle + Math.PI / 2);

  // Thruster flame — animated plume behind drone tail
  if (flying) {
    const pulse = 0.6 + 0.4 * Math.sin(t * 18 + drone.id * 2.3);
    const flameH = h * (0.35 + pulse * 0.3);
    const flameW = w * 0.35;
    const flameGrad = ctx.createLinearGradient(0, h * 0.2, 0, h * 0.2 + flameH);
    flameGrad.addColorStop(0, `rgba(255,180,40,${0.9 * pulse})`);
    flameGrad.addColorStop(0.4, `rgba(255,80,10,${0.7 * pulse})`);
    flameGrad.addColorStop(1, 'rgba(255,40,0,0)');
    ctx.beginPath();
    ctx.ellipse(0, h * 0.2 + flameH * 0.5, flameW, flameH * 0.55, 0, 0, Math.PI * 2);
    ctx.fillStyle = flameGrad;
    ctx.fill();

    // Hard bright core dot
    ctx.beginPath();
    ctx.arc(0, h * 0.22, w * 0.18, 0, Math.PI * 2);
    ctx.fillStyle = `rgba(255,220,100,${0.9 * pulse})`;
    ctx.fill();
  }

  // Body
  ctx.beginPath();
  ctx.moveTo(0, -h * 0.5);
  ctx.lineTo(-w * 0.4, h * 0.2);
  ctx.lineTo(w * 0.4, h * 0.2);
  ctx.closePath();
  ctx.fillStyle = C.COL_DRONE;
  ctx.fill();

  // Fins
  for (const side of [-1, 1]) {
    ctx.beginPath();
    ctx.moveTo(side * w * 0.4, h * 0.1);
    ctx.lineTo(side * w * 0.7, h * 0.4);
    ctx.lineTo(side * w * 0.25, h * 0.25);
    ctx.closePath();
    ctx.fillStyle = '#009922';
    ctx.fill();
  }

  ctx.restore();

  // Cargo bar (screen space)
  if (drone.state !== 'Holding') {
    const barW = Math.max(10, 22 * scale);
    const barH = Math.max(2, 3 * scale);
    const barX = sx - barW / 2;
    const barY = sy - h * 0.65 - barH - 3 * scale;
    ctx.fillStyle = C.COL_CARGO_BAR_BG;
    ctx.fillRect(barX, barY, barW, barH);
    ctx.fillStyle = C.COL_CARGO_BAR_FG;
    ctx.fillRect(barX, barY, barW * cargoRatio, barH);
  }
}

// ─── Mining beam ─────────────────────────────────────────────────────────────

function drawMiningBeam(ctx: CanvasRenderingContext2D, cx: number, cy: number, scale: number, drone: Drone, asteroidX: number, asteroidY: number, t: number): void {
  const [sx, sy] = worldToCanvas(drone.x, drone.y, cx, cy, scale);
  const [ax, ay] = worldToCanvas(asteroidX, asteroidY, cx, cy, scale);
  const flicker = 0.55 + 0.45 * Math.abs(Math.sin(t * 23 + drone.id));

  ctx.save();

  // Outer glow pass
  ctx.strokeStyle = `rgba(0,255,140,${0.18 * flicker})`;
  ctx.lineWidth   = Math.max(3, 7 * scale);
  ctx.shadowColor = '#00FF99';
  ctx.shadowBlur  = 12 * scale;
  ctx.beginPath();
  ctx.moveTo(sx, sy);
  ctx.lineTo(ax, ay);
  ctx.stroke();

  // Core bright pass
  ctx.shadowBlur  = 4 * scale;
  ctx.strokeStyle = `rgba(160,255,200,${0.75 * flicker})`;
  ctx.lineWidth   = Math.max(1, 1.5 * scale);
  ctx.beginPath();
  ctx.moveTo(sx, sy);
  ctx.lineTo(ax, ay);
  ctx.stroke();

  ctx.restore();
}

// ─── Particles ───────────────────────────────────────────────────────────────

function drawParticles(ctx: CanvasRenderingContext2D, cx: number, cy: number, scale: number): void {
  for (const p of particles) {
    const [sx, sy] = worldToCanvas(p.x, p.y, cx, cy, scale);
    const frac = p.life / p.maxLife;
    const r    = Math.max(1, (1.5 + frac * 2.5) * scale);
    ctx.beginPath();
    ctx.arc(sx, sy, r, 0, Math.PI * 2);
    ctx.fillStyle = `rgba(220,120,40,${frac * 0.9})`;
    ctx.fill();
  }
}

// ─── Main render export ───────────────────────────────────────────────────────

let lastRenderTime = 0;

export function render(
  state: GameState,
  canvas: HTMLCanvasElement,
  ctx: CanvasRenderingContext2D,
  now: number,   // performance.now() from rAF
): void {
  const dt = Math.min((now - lastRenderTime) / 1000, 0.1);
  lastRenderTime = now;

  const dpr = window.devicePixelRatio || 1;
  const W   = canvas.width  / dpr;
  const H   = canvas.height / dpr;
  const cx  = W / 2;
  const cy  = H / 2;
  const scale = Math.min(Math.min(W, H) / 900, 1.2);
  const t   = state.elapsedSeconds;

  // Detect depleted asteroids and emit particle bursts
  const curIds = new Set(state.asteroids.map(a => a.id));
  for (const [id, pos] of prevAsteroidPos) {
    if (!curIds.has(id)) emitDepletionBurst(pos.x, pos.y);
  }
  for (const ast of state.asteroids) prevAsteroidPos.set(ast.id, { x: ast.x, y: ast.y });
  for (const id of prevAsteroidPos.keys()) {
    if (!curIds.has(id)) prevAsteroidPos.delete(id);
  }

  tickParticles(dt);

  // Clear
  ctx.fillStyle = C.COL_BG;
  ctx.fillRect(0, 0, W, H);

  if (!starsNear || !starsFar) initStars();

  // Parallax offset — centroid of all drones, very subtle
  let dox = 0, doy = 0;
  if (state.drones.length > 0) {
    for (const d of state.drones) { dox += d.x; doy += d.y; }
    dox /= state.drones.length;
    doy /= state.drones.length;
  }

  // Far star layer (very slow parallax)
  for (const star of starsFar!) {
    const ox = dox * 0.012, oy = doy * 0.012;
    const [sx, sy] = worldToCanvas(star.x - ox, star.y - oy, cx, cy, scale * 0.55);
    ctx.beginPath();
    ctx.arc(sx, sy, star.r * 0.8, 0, Math.PI * 2);
    ctx.fillStyle = `rgba(180,195,255,${star.a * 0.7})`;
    ctx.fill();
  }

  // Near star layer (moderate parallax)
  for (const star of starsNear!) {
    const ox = dox * 0.04, oy = doy * 0.04;
    const [sx, sy] = worldToCanvas(star.x - ox, star.y - oy, cx, cy, scale * 0.65);
    ctx.beginPath();
    ctx.arc(sx, sy, star.r, 0, Math.PI * 2);
    ctx.fillStyle = `rgba(210,220,255,${star.a})`;
    ctx.fill();
  }

  // Asteroids
  for (const ast of state.asteroids) {
    drawAsteroid(ctx, cx, cy, scale, ast.x, ast.y, ast.oreRemaining, ast.maxOre, ast.polyVerts);
  }

  // Station
  drawStation(ctx, cx, cy, scale, state.stationRotation, t);

  // Mining beams
  for (const drone of state.drones) {
    if (drone.state === 'Mining' && drone.targetAsteroidId !== null) {
      const ast = state.asteroids.find(a => a.id === drone.targetAsteroidId);
      if (ast) drawMiningBeam(ctx, cx, cy, scale, drone, ast.x, ast.y, t);
    }
  }

  // Drones
  for (const drone of state.drones) {
    drawDrone(ctx, cx, cy, scale, drone, t);
  }

  // Depletion particles
  drawParticles(ctx, cx, cy, scale);
}
