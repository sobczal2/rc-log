import type { RatingLevel } from "./list";

export type AddPerformedVariationQualityDto = RatingLevel;
export type AddPerformedVariationComfortDto = RatingLevel;
export type AddPerformedVariationRepeatabilityDto = RatingLevel;

export interface AddPerformedVariationRequest {
  variationId: string;
  quality: AddPerformedVariationQualityDto;
  comfort: AddPerformedVariationComfortDto;
  repeatability: AddPerformedVariationRepeatabilityDto;
  note?: string | null;
}

export interface AddPerformedVariationDto {
  performedVariationId: string;
  variationId: string;
  quality: AddPerformedVariationQualityDto;
  comfort: AddPerformedVariationComfortDto;
  repeatability: AddPerformedVariationRepeatabilityDto;
  note: string | null;
}
