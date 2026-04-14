import type { Type } from "@/models/model/type";
import type { DifficultyLevel } from "@/models/shared/difficulty";

export interface TagDto {
  id: string;
  name: string;
}

export interface ManeuverDto {
  id: string;
  name: string;
  model_type: Type;
  minDifficulty: DifficultyLevel;
  maxDifficulty: DifficultyLevel;
  tags: TagDto[];
  description: string;
  defaultVariationVideoAssetId: string;
}

export interface ManeuverFilter {
  tags?: string[];
  model_type?: Type | null;
  difficulty?: DifficultyLevel | null;
  searchQuery?: string | null;
}

export type ManeuverSortField = "name" | "difficulty";
export type SortDirection = "asc" | "desc";

export const ALL_MANEUVER_SORT_FIELDS: readonly ManeuverSortField[] = ["name", "difficulty"];
export const ALL_SORT_DIRECTIONS: readonly SortDirection[] = ["asc", "desc"];

export function getManeuverSortFieldLabel(field: ManeuverSortField): string {
  switch (field) {
    case "name":
      return "Name";
    case "difficulty":
      return "Difficulty";
  }
}

export function getSortDirectionLabel(direction: SortDirection): string {
  switch (direction) {
    case "asc":
      return "Ascending";
    case "desc":
      return "Descending";
  }
}

export interface ManeuverSort {
  field?: ManeuverSortField;
  direction?: SortDirection;
}
