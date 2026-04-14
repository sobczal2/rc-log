export interface UpdateSessionRequest {
  date: string;
  modelId?: string | null;
  note?: string | null;
}

export interface UpdateSessionDto {
  id: string;
  userId: string;
  date: string;
  modelId: string | null;
  note: string | null;
}
