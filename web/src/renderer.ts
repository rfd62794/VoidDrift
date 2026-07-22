import type { GameState, Drone } from './types.ts';
import { C } from './constants.ts';

interface StarDot { x: number; y: number; r: number; a: number }

let stars: StarDot[] | null = null;

function initStars(): StarDot[] {
  const s: StarDot[] = [];
  // Seeded deterministic using a simple LCG
  let seed = 0xDEADBEEF;
  const rng = () => { seed = (seed * 1664525 + 1013904223) & 0xFFFFFFFF; return (seed >>> 0) / 0xFFFFFFFF; };
  for (let i = 0; i < 320; i++) {
    const angle = rng() * Math.PI * 2;
    const d = 80 + rng() * 1200;
    s.push({ x: Math.cos(angle) * d, y: Math.sin(angle) * d, r: rng() < 0.3 ? 1.2 : 0.7, a: 0.3 + rng() * 0.7 });
  }
  return s;
}

function worldToCanvas(wx: number, wy: number, cx: number, cy: number, scale: number): [number, number] {
  return [cx + wx * scale, cy + wy * scale];
}

function drawStation(ctx: CanvasRenderingContext2D, cx: number, cy: number, scale: number, rotation: number): void {
  const armLen = C.STATION_ARM_LENGTH * scale;
  const armThick = Math.max(2, C.STATION_ARM_THICKNESS * scale);
  const hubR = C.STATION_HUB_RADIUS * scale;
  const berthR = Math.max(4, C.STATION_BERTH_RADIUS * scale);
  const numArms = 6;
  const berthArms = [0, 1, 2];

  ctx.save();
  ctx.translate(cx, cy);
  ctx.rotate(rotation);

  // Draw arms
  ctx.strokeStyle = C.COL_ARM;
  ctx.lineWidth = armThick;
  for (let i = 0; i < numArms; i++) {
    const angle = (i / numArms) * Math.PI * 2;
    ctx.beginPath();
    ctx.moveTo(0, 0);
    ctx.lineTo(Math.cos(angle) * armLen, Math.sin(angle) * armLen);
    ctx.stroke();
  }

  // Draw berths at arm ends
  for (const bi of berthArms) {
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
  const grad = ctx.createRadialGradient(0, 0, hubR * 0.2, 0, 0, hubR);
  grad.addColorStop(0, '#FFEE44');
  grad.addColorStop(0.6, '#FFD700');
  grad.addColorStop(1, '#AA8800');
  ctx.fillStyle = grad;
  ctx.fill();
  ctx.strokeStyle = '#886600';
  ctx.lineWidth = Math.max(1, scale * 1.5);
  ctx.stroke();

  ctx.restore();
}

function drawAsteroid(ctx: CanvasRenderingContext2D, cx: number, cy: number, scale: number, x: number, y: number, oreRemaining: number, maxOre: number, verts: Array<{ x: number; y: number }>): void {
  const [sx, sy] = worldToCanvas(x, y, cx, cy, scale);
  const depletion = oreRemaining / maxOre;
  const col = depletion > 0.15
    ? `rgb(${Math.round(192 * depletion + 46 * (1 - depletion))}, ${Math.round(96 * depletion + 46 * (1 - depletion))}, ${Math.round(37 * depletion + 46 * (1 - depletion))})`
    : C.COL_IRON_AST_DEPLETED;

  ctx.save();
  ctx.translate(sx, sy);

  // Glow
  if (depletion > 0.1) {
    const glow = ctx.createRadialGradient(0, 0, 0, 0, 0, C.ASTEROID_RADIUS_IRON * scale * 1.6);
    glow.addColorStop(0, 'rgba(180,90,30,0.18)');
    glow.addColorStop(1, 'rgba(180,90,30,0)');
    ctx.fillStyle = glow;
    ctx.beginPath();
    ctx.arc(0, 0, C.ASTEROID_RADIUS_IRON * scale * 1.6, 0, Math.PI * 2);
    ctx.fill();
  }

  // Polygon
  ctx.beginPath();
  ctx.moveTo(verts[0].x * scale, verts[0].y * scale);
  for (let i = 1; i < verts.length; i++) {
    ctx.lineTo(verts[i].x * scale, verts[i].y * scale);
  }
  ctx.closePath();
  ctx.fillStyle = col;
  ctx.fill();
  ctx.strokeStyle = depletion > 0.15 ? '#E07830' : '#555';
  ctx.lineWidth = Math.max(1, scale * 0.8);
  ctx.stroke();

  // Ore bar (mini progress bar above asteroid)
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

function drawDrone(ctx: CanvasRenderingContext2D, cx: number, cy: number, scale: number, drone: Drone): void {
  const [sx, sy] = worldToCanvas(drone.x, drone.y, cx, cy, scale);
  const w = Math.max(5, 14 * scale);
  const h = Math.max(8, 28 * scale);
  const cargoRatio = drone.cargo / C.CARGO_CAPACITY;

  ctx.save();
  ctx.translate(sx, sy);
  ctx.rotate(drone.headingAngle + Math.PI / 2);

  // Body (triangle pointing "up" in local space — rotated by heading)
  ctx.beginPath();
  ctx.moveTo(0, -h * 0.5);          // nose
  ctx.lineTo(-w * 0.4, h * 0.2);   // left base
  ctx.lineTo(w * 0.4, h * 0.2);    // right base
  ctx.closePath();
  ctx.fillStyle = C.COL_DRONE;
  ctx.fill();

  // Fins
  ctx.beginPath();
  ctx.moveTo(-w * 0.4, h * 0.1);
  ctx.lineTo(-w * 0.7, h * 0.4);
  ctx.lineTo(-w * 0.25, h * 0.25);
  ctx.closePath();
  ctx.fillStyle = '#009922';
  ctx.fill();

  ctx.beginPath();
  ctx.moveTo(w * 0.4, h * 0.1);
  ctx.lineTo(w * 0.7, h * 0.4);
  ctx.lineTo(w * 0.25, h * 0.25);
  ctx.closePath();
  ctx.fillStyle = '#009922';
  ctx.fill();

  // Thruster glow (when not Holding/Unloading)
  if (drone.state !== 'Holding' && drone.state !== 'Unloading' && drone.state !== 'Mining') {
    ctx.beginPath();
    ctx.arc(0, h * 0.25, w * 0.22, 0, Math.PI * 2);
    ctx.fillStyle = 'rgba(255,100,20,0.85)';
    ctx.fill();
  }

  ctx.restore();

  // Cargo bar (always visible, in screen space)
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

function drawMiningBeam(ctx: CanvasRenderingContext2D, cx: number, cy: number, scale: number, drone: Drone, asteroidX: number, asteroidY: number): void {
  const [sx, sy] = worldToCanvas(drone.x, drone.y, cx, cy, scale);
  const [ax, ay] = worldToCanvas(asteroidX, asteroidY, cx, cy, scale);

  ctx.save();
  ctx.strokeStyle = C.COL_BEAM;
  ctx.lineWidth = Math.max(1, 2 * scale);
  ctx.shadowColor = '#00FF99';
  ctx.shadowBlur = 8 * scale;
  ctx.beginPath();
  ctx.moveTo(sx, sy);
  ctx.lineTo(ax, ay);
  ctx.stroke();
  ctx.restore();
}

export function render(
  state: GameState,
  canvas: HTMLCanvasElement,
  ctx: CanvasRenderingContext2D,
): void {
  const dpr = window.devicePixelRatio || 1;
  const W = canvas.width / dpr;
  const H = canvas.height / dpr;
  const cx = W / 2;
  const cy = H / 2;
  const scale = Math.min(Math.min(W, H) / 900, 1.2);

  // Clear
  ctx.fillStyle = C.COL_BG;
  ctx.fillRect(0, 0, canvas.width / dpr, canvas.height / dpr);

  // Stars
  if (!stars) stars = initStars();
  for (const star of stars) {
    const [sx, sy] = worldToCanvas(star.x, star.y, cx, cy, scale * 0.6);
    ctx.beginPath();
    ctx.arc(sx, sy, star.r, 0, Math.PI * 2);
    ctx.fillStyle = `rgba(200,210,255,${star.a})`;
    ctx.fill();
  }

  // Asteroids
  for (const ast of state.asteroids) {
    drawAsteroid(ctx, cx, cy, scale, ast.x, ast.y, ast.oreRemaining, ast.maxOre, ast.polyVerts);
  }

  // Station
  drawStation(ctx, cx, cy, scale, state.stationRotation);

  // Mining beams (draw before drones so beam appears "under" drone)
  for (const drone of state.drones) {
    if (drone.state === 'Mining' && drone.targetAsteroidId !== null) {
      const ast = state.asteroids.find(a => a.id === drone.targetAsteroidId);
      if (ast) {
        drawMiningBeam(ctx, cx, cy, scale, drone, ast.x, ast.y);
      }
    }
  }

  // Drones
  for (const drone of state.drones) {
    drawDrone(ctx, cx, cy, scale, drone);
  }
}
