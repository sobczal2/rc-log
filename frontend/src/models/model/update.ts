import type { Type } from "@/models/model/type";

export interface UpdateModelRequest {
  name: string;
  type: Type;
}

export interface UpdateModelDto {
  id: string;
  name: string;
  type: Type;
  photoAssetId: string | null;
}
