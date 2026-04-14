import type { Type } from "@/models/model/type";

export interface UpdateModelPhotoDto {
  id: string;
  name: string;
  type: Type;
  photoAssetName: string | null;
}
