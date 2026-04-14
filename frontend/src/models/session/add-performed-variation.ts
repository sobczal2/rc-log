import type {
  PerformedVariationDto,
  RatingDto,
} from "@/models/__generated/session/add-performed-variation";

export type AddPerformedVariationRatingDto = RatingDto;

export interface AddPerformedVariationRequest {
  variationId: string;
  quality: AddPerformedVariationRatingDto;
  comfort: AddPerformedVariationRatingDto;
  repeatability: AddPerformedVariationRatingDto;
  note?: string | null;
}

export type AddPerformedVariationDto = PerformedVariationDto;
