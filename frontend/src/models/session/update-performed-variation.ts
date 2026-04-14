import type { RatingLevel } from "./list";

export type UpdatePerformedVariationQualityDto = RatingLevel;
export type UpdatePerformedVariationComfortDto = RatingLevel;
export type UpdatePerformedVariationRepeatabilityDto = RatingLevel;

export interface UpdatePerformedVariationRequest {
  quality: UpdatePerformedVariationQualityDto;
  comfort: UpdatePerformedVariationComfortDto;
  repeatability: UpdatePerformedVariationRepeatabilityDto;
  note?: string | null;
}
