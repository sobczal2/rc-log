import type { SessionDto } from "@/models/__generated/session/update";

export interface UpdateSessionRequest {
  date: string;
  modelId?: string | null;
  note?: string | null;
}

export type UpdateSessionDto = SessionDto;
