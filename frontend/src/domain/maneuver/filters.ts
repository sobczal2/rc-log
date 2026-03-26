import type { VehicleType } from "./vehicle";
import type { DifficultyLevel } from "./difficulty";

export interface ManeuverFilter {
  tags?: string[];
  vehicleType?: VehicleType | null;
  difficulty?: DifficultyLevel | null;
  searchQuery?: string | null;
}

export interface ManeuverSort {
  field?: "name" | "difficulty";
  direction?: "asc" | "desc";
}

export interface PaginationOptions {
  page?: number;
  pageSize?: number;
}

export interface PaginatedResult<T> {
  items: T[];
  total: number;
  page: number;
  pageSize: number;
  totalPages: number;
}