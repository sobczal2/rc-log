import type { Type } from "@/models/model/type";
import type { DifficultyLevel } from "@/models/shared/difficulty";

export interface TagDto {
  id: string;
  name: string;
}

export interface VariationDto {
  id: string;
  name: string;
  description: string;
  videoAssetName: string;
  difficulty: DifficultyLevel;
}

export interface ManeuverDto {
  id: string;
  name: string;
  type: Type;
  minDifficulty: DifficultyLevel;
  maxDifficulty: DifficultyLevel;
  tags: TagDto[];
  description: string;
  defaultVariation: VariationDto;
  variations: VariationDto[];
}
