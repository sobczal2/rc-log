export interface CreateSessionRequest {
  date: string;
  modelId?: string | null;
  note?: string | null;
}

export interface CreateSessionDto {
  id: string;
  userId: string;
  date: string;
  modelId: string | null;
  note: string | null;
}
