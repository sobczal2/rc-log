import type { SessionDto } from "@/models/__generated/session/create";

export interface CreateSessionRequest {
  date: string;
  modelId?: string | null;
  note?: string | null;
}

export type CreateSessionDto = SessionDto;
