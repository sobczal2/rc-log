import type { PaginatedResult, PaginationDto } from "./shared";

export interface TagDto {
  id: string;
  name: string;
}

export interface ManeuverDto {
  id: string;
  vehicle_type: string;
  name: string;
  tags: TagDto[];
  description: string;
  difficulty: number;
  video_path: string | null;
}

export interface GetManeuverByIdRequest {
  id: string;
}

export type GetManeuverByIdResponse = ManeuverDto;

export type ListManeuversRequest = PaginationDto;

export type ListManeuversResponse = PaginatedResult<ManeuverDto>;
