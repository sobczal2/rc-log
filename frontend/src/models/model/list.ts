import type { Type } from "@/models/model/type";

export interface ListModelDto {
  id: string;
  name: string;
  type: Type;
  photoAssetName: string | null;
}
