import type { VehicleType } from "@/models/shared/vehicle-type";
import type { DifficultyLevel } from "@/models/shared/difficulty";

export interface TagDto {
  id: string;
  name: string;
}

export interface ManeuverDto {
  id: string;
  name: string;
  vehicleType: VehicleType;
  minDifficulty: DifficultyLevel;
  maxDifficulty: DifficultyLevel;
  tags: TagDto[];
  description: string;
  defaultVariationVideoAssetName: string;
}

export interface ManeuverFilter {
  tags?: string[];
  vehicleType?: VehicleType | null;
  difficulty?: DifficultyLevel | null;
  searchQuery?: string | null;
}

export interface ManeuverSort {
  field?: "name" | "difficulty";
  direction?: "asc" | "desc";
}
