import type { GameState, SaveData } from './types.ts';
import { C } from './constants.ts';
import { createFreshState, spawnDrone } from './state.ts';

const SAVE_VERSION = 1;

export function saveGame(state: GameState): void {
  const data: SaveData = {
    version: SAVE_VERSION,
    ironOre: state.ironOre,
    ironIngots: state.ironIngots,
    hullPlates: state.hullPlates,
    refineryProgress: state.refineryProgress,
    forgeProgress: state.forgeProgress,
    droneBuildProgress: state.droneBuildProgress,
    totalDronesBuilt: state.totalDronesBuilt,
    totalOreHarvested: state.totalOreHarvested,
    elapsedSeconds: state.elapsedSeconds,
    droneCount: state.drones.length,
  };
  try {
    localStorage.setItem(C.SAVE_KEY, JSON.stringify(data));
  } catch {
    // localStorage may be unavailable in some environments
  }
}

export function loadGame(): GameState {
  try {
    const raw = localStorage.getItem(C.SAVE_KEY);
    if (!raw) return createFreshState();
    const data: SaveData = JSON.parse(raw);
    if (data.version !== SAVE_VERSION) {
      localStorage.removeItem(C.SAVE_KEY);
      return createFreshState();
    }

    const state = createFreshState();
    state.ironOre = data.ironOre ?? 0;
    state.ironIngots = data.ironIngots ?? 0;
    state.hullPlates = data.hullPlates ?? 0;
    state.refineryProgress = data.refineryProgress ?? 0;
    state.forgeProgress = data.forgeProgress ?? 0;
    state.droneBuildProgress = data.droneBuildProgress ?? 0;
    state.totalDronesBuilt = data.totalDronesBuilt ?? 1;
    state.totalOreHarvested = data.totalOreHarvested ?? 0;
    state.elapsedSeconds = data.elapsedSeconds ?? 0;

    // Rebuild drone fleet — spawn saved number of drones at station
    const droneCount = Math.max(1, Math.min(data.droneCount ?? 1, C.DRONE_MAX_ACTIVE));
    state.drones = [];
    for (let i = 0; i < droneCount; i++) {
      state.drones.push(spawnDrone());
    }

    return state;
  } catch {
    return createFreshState();
  }
}

export function clearSave(): void {
  try {
    localStorage.removeItem(C.SAVE_KEY);
  } catch {
    // ignore
  }
}
