import type { Type } from "@/models/model/type";
import type { ModelDto } from "@/models/__generated/model/create";

export interface CreateModelRequest {
  name: string;
  type: Type;
}

export type CreateModelDto = ModelDto;
