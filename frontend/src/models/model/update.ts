import type { Type } from "@/models/model/type";
import type { ModelDto } from "@/models/__generated/model/update";

export interface UpdateModelRequest {
  name: string;
  type: Type;
}

export type UpdateModelDto = ModelDto;
