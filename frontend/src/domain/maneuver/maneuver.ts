import type { VehicleType } from "./vehicle";
import type { DifficultyLevel } from "./difficulty";
import type { TagDto } from "./tag";

export interface ManeuverDto {
  id: string;
  name: string;
  vehicleType: VehicleType;
  difficulty: DifficultyLevel;
  tags: TagDto[];
  description: string;
  videoUrl?: string;
}
