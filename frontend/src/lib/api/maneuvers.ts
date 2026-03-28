import type {
  ListManeuverDto,
  ListManeuverFilter,
  ListManeuverSort,
  GetByIdManeuverDto,
} from "@/models/maneuver";
import type { PaginationOptions, PaginatedResult } from "@/models/shared";
import { apiClient } from "../apiClient";

export type { ListManeuverFilter, ListManeuverSort, PaginationOptions };

export interface GetManeuverByIdRequest {
  id: string;
}

export type GetManeuverByIdResponse = GetByIdManeuverDto;

export interface ListManeuversRequest extends PaginationOptions {
  filter?: ListManeuverFilter;
  sort?: ListManeuverSort;
}

export type ListManeuversResponse = PaginatedResult<ListManeuverDto>;

export const maneuversApi = {
  getById: async (req: GetManeuverByIdRequest): Promise<GetManeuverByIdResponse> => {
    const { data } = await apiClient.get<GetManeuverByIdResponse>(`/maneuvers/${req.id}`);
    return data;
  },

  list: async (req: ListManeuversRequest): Promise<ListManeuversResponse> => {
    const params = new URLSearchParams();

    if (req.page !== undefined) params.append("page", req.page.toString());
    if (req.pageSize !== undefined) params.append("pageSize", req.pageSize.toString());

    if (req.filter) {
      if (req.filter.tags && req.filter.tags.length > 0) {
        params.append("tags", req.filter.tags.join(","));
      }
      if (req.filter.vehicleType) {
        params.append("vehicleType", req.filter.vehicleType);
      }
      if (req.filter.difficulty) {
        params.append("difficulty", req.filter.difficulty);
      }
      if (req.filter.searchQuery && req.filter.searchQuery.trim()) {
        params.append("searchQuery", req.filter.searchQuery.trim());
      }
    }

    if (req.sort) {
      if (req.sort.field) {
        params.append("sortField", req.sort.field);
      }
      if (req.sort.direction) {
        params.append("sortDirection", req.sort.direction);
      }
    }

    const { data } = await apiClient.get<ListManeuversResponse>("/maneuvers", {
      params,
    });
    return data;
  },
};
