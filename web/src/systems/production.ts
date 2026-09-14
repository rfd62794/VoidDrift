import type { GameState } from '../types.ts';
import { C } from '../constants.ts';
import { spawnDrone } from '../state.ts';

/**
 * Auto-refinery: continuously converts iron ore → iron ingots.
 * Matches the fractional rate from auto_process.rs:
 *   ingots/s = 1 / iron_time = 0.25 ingots/s
 *   ore consumed = ingots_produced * ratio
 */
export function updateRefinery(state: GameState, dt: number): void {
  if (state.ironOre <= 0) return;

  const ingotRate = 1 / C.IRON_TIME; // ingots/s
  const oreNeeded = ingotRate * C.REFINERY_RATIO * dt;
  const actualOre = Math.min(oreNeeded, state.ironOre);
  state.ironOre -= actualOre;
  state.ironIngots += actualOre / C.REFINERY_RATIO;
}

/**
 * Auto-forge: continuously converts iron ingots → hull plates.
 * Rate: 1 / hull_time = 0.2 hull plates/s, consuming 2 ingots per plate.
 */
export function updateForge(state: GameState, dt: number): void {
  if (state.ironIngots <= 0) return;

  const hullRate = 1 / C.HULL_TIME; // hulls/s
  const ingotNeeded = hullRate * C.HULL_COST_IRON_INGOTS * dt;
  const maxFromIngots = state.ironIngots / C.HULL_COST_IRON_INGOTS;
  const actualHullBatches = Math.min(hullRate * dt, maxFromIngots);
  state.ironIngots -= actualHullBatches * C.HULL_COST_IRON_INGOTS;
  state.hullPlates += actualHullBatches;
}

/**
 * Auto-build drones: consumes hull plates, builds drones over time.
 * Uses a fractional progress accumulator like the original Bevy system.
 */
export function updateDroneBuild(state: GameState, dt: number): void {
  const activeDrones = state.drones.length;
  if (activeDrones >= C.DRONE_MAX_ACTIVE) return;
  if (state.hullPlates < C.DRONE_COST_HULLS) return;

  state.droneBuildProgress += dt / C.DRONE_BUILD_TIME;

  if (state.droneBuildProgress >= 1.0) {
    const built = Math.floor(state.droneBuildProgress);
    const affordable = Math.floor(state.hullPlates / C.DRONE_COST_HULLS);
    const spaceLeft = C.DRONE_MAX_ACTIVE - activeDrones;
    const actual = Math.min(built, affordable, spaceLeft);

    if (actual > 0) {
      state.hullPlates -= actual * C.DRONE_COST_HULLS;
      for (let i = 0; i < actual; i++) {
        state.drones.push(spawnDrone());
        state.totalDronesBuilt++;
      }
    }
    state.droneBuildProgress -= Math.floor(state.droneBuildProgress);
  }
}
