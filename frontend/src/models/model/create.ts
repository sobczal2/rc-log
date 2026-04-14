import type { Type } from "@/models/model/type";

export interface CreateModelRequest {
  name: string;
  type: Type;
}

export interface CreateModelDto {
  id: string;
  name: string;
  type: Type;
  photoAssetId: string | null;
}
