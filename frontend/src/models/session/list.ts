import type { Type } from "@/models/model/type";
import type {
  PerformedVariationDto as GeneratedPerformedVariationDto,
  RatingDto as GeneratedRatingDto,
  SessionDto as GeneratedSessionDto,
} from "@/models/__generated/session/list";

export type SessionSortField = "date";
export type SortDirection = "asc" | "desc";

export type RatingLevel = GeneratedRatingDto;
export type RatingDto = GeneratedRatingDto;

export const ALL_RATING_LEVELS: readonly RatingLevel[] = ["one", "two", "three", "four", "five"];

export function getRatingLabel(level: RatingLevel): string {
  switch (level) {
    case "one":
      return "1";
    case "two":
      return "2";
    case "three":
      return "3";
    case "four":
      return "4";
    case "five":
      return "5";
  }
}

export interface SessionFilter {
  modelIds?: string[];
  maneuverIds?: string[];
  searchQuery?: string | null;
}

export interface SessionSort {
  field?: SessionSortField;
  direction?: SortDirection;
}

export type PerformedVariationDto = GeneratedPerformedVariationDto;
export type SessionDto = Omit<GeneratedSessionDto, "modelType"> & { modelType: Type | null };

export function ratingToNumber(level: RatingDto): number {
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
