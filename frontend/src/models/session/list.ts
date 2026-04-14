import type { VehicleType } from "@/models/shared/vehicle-type";

export type SessionSortField = "date";
export type SortDirection = "asc" | "desc";
export type QualityDto = "one" | "two" | "three" | "four" | "five";
export type ComfortDto = "one" | "two" | "three" | "four" | "five";
export type RepeatabilityDto = "one" | "two" | "three" | "four" | "five";

export interface SessionFilter {
  modelIds?: string[];
  maneuverIds?: string[];
  searchQuery?: string | null;
}

export interface SessionSort {
  field?: SessionSortField;
  direction?: SortDirection;
}

export interface PerformedVariationDto {
  performedVariationId: string;
  variationId: string;
  maneuverName: string | null;
  variationName: string | null;
  quality: QualityDto;
  comfort: ComfortDto;
  repeatability: RepeatabilityDto;
}

export interface SessionDto {
  id: string;
  userId: string;
  date: string;
  modelId: string | null;
  modelName: string | null;
  modelType: VehicleType | null;
  modelPhotoAssetName: string | null;
  performedVariations: PerformedVariationDto[];
}

export function ratingToNumber(level: QualityDto | ComfortDto | RepeatabilityDto): number {
  switch (level) {
    case "one":
      return 1;
    case "two":
      return 2;
    case "three":
      return 3;
    case "four":
      return 4;
    case "five":
      return 5;
  }
}
