import type {
  ComfortDto,
  QualityDto,
  RepeatabilityDto,
} from "@/models/__generated/session/update-performed-variation";

export type UpdatePerformedVariationQualityDto = QualityDto;
export type UpdatePerformedVariationComfortDto = ComfortDto;
export type UpdatePerformedVariationRepeatabilityDto = RepeatabilityDto;

export interface UpdatePerformedVariationRequest {
  quality: UpdatePerformedVariationQualityDto;
  comfort: UpdatePerformedVariationComfortDto;
  repeatability: UpdatePerformedVariationRepeatabilityDto;
  note?: string | null;
}
