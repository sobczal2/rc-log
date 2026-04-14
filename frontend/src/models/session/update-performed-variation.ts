import type { RatingDto } from "@/models/__generated/session/update-performed-variation";

export type UpdatePerformedVariationRatingDto = RatingDto;

export interface UpdatePerformedVariationRequest {
  quality: UpdatePerformedVariationRatingDto;
  comfort: UpdatePerformedVariationRatingDto;
  repeatability: UpdatePerformedVariationRatingDto;
  note?: string | null;
}
