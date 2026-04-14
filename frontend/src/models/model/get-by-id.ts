import type { Type } from "@/models/model/type";

export interface GetByIdModelDto {
  id: string;
  name: string;
  type: Type;
  photoAssetName: string | null;
}
