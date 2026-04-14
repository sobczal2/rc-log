export interface SessionMutationDto {
  id: string;
  userId: string;
  date: string;
  modelId: string | null;
  note: string | null;
}

export type AddPerformedVariationQualityDto = "one" | "two" | "three" | "four" | "five";
export type AddPerformedVariationComfortDto = "one" | "two" | "three" | "four" | "five";
export type AddPerformedVariationRepeatabilityDto =
  | "one"
  | "two"
  | "three"
  | "four"
  | "five";

export type UpdatePerformedVariationQualityDto = "one" | "two" | "three" | "four" | "five";
export type UpdatePerformedVariationComfortDto = "one" | "two" | "three" | "four" | "five";
export type UpdatePerformedVariationRepeatabilityDto =
  | "one"
  | "two"
  | "three"
  | "four"
  | "five";

export interface SessionPerformedVariationDto {
  performedVariationId: string;
  variationId: string;
  quality: AddPerformedVariationQualityDto;
  comfort: AddPerformedVariationComfortDto;
  repeatability: AddPerformedVariationRepeatabilityDto;
  note: string | null;
}

export interface CreateSessionRequest {
  date: string;
  modelId?: string | null;
  note?: string | null;
}

export interface UpdateSessionRequest {
  date: string;
  modelId?: string | null;
  note?: string | null;
}

export interface AddPerformedVariationRequest {
  variationId: string;
  quality: AddPerformedVariationQualityDto;
  comfort: AddPerformedVariationComfortDto;
  repeatability: AddPerformedVariationRepeatabilityDto;
  note?: string | null;
}

export interface UpdatePerformedVariationRequest {
  quality: UpdatePerformedVariationQualityDto;
  comfort: UpdatePerformedVariationComfortDto;
  repeatability: UpdatePerformedVariationRepeatabilityDto;
  note?: string | null;
}