import type {
  ComfortDto,
  PerformedVariationDto,
  QualityDto,
  RepeatabilityDto,
} from "@/models/__generated/session/add-performed-variation";

export type AddPerformedVariationQualityDto = QualityDto;
export type AddPerformedVariationComfortDto = ComfortDto;
export type AddPerformedVariationRepeatabilityDto = RepeatabilityDto;

export interface AddPerformedVariationRequest {
  variationId: string;
  quality: AddPerformedVariationQualityDto;
  comfort: AddPerformedVariationComfortDto;
  repeatability: AddPerformedVariationRepeatabilityDto;
  note?: string | null;
}

export type AddPerformedVariationDto = PerformedVariationDto;
