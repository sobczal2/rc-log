import type { VehicleType } from "./vehicle";
import type { DifficultyLevel } from "./difficulty";
import type { TagDto } from "./tag";

export interface Maneuver {
  id: string;
  name: string;
  vehicleType: VehicleType;
  difficulty: DifficultyLevel;
  tags: TagDto[];
  description: string;
  videoPath?: string;
}

export function getManeuverVideoUrl(maneuver: Maneuver): string | null {
  if (!maneuver.videoPath) {
    return null;
  }

  return `/assets/${maneuver.videoPath}`;
}
