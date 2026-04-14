import type {
  AddPerformedVariationRequest,
  AddPerformedVariationDto,
  CreateSessionRequest,
  CreateSessionDto,
  ListSessionDto,
  ListSessionFilter,
  ListSessionSort,
  UpdateSessionRequest,
  UpdateSessionDto,
  UpdatePerformedVariationRequest,
} from "@/models/session";
import type { PaginationOptions, PaginatedResult } from "@/models/shared";
import { apiClient } from "../apiClient";

export type { ListSessionFilter, ListSessionSort, PaginationOptions };

export interface ListSessionsRequest extends PaginationOptions {
  filter?: ListSessionFilter;
  sort?: ListSessionSort;
}

export interface AddPerformedVariationParams {
  sessionId: string;
  payload: AddPerformedVariationRequest;
}

export interface UpdatePerformedVariationParams {
  sessionId: string;
  performedVariationId: string;
  payload: UpdatePerformedVariationRequest;
}

export type ListSessionsResponse = PaginatedResult<ListSessionDto>;

export const sessionsApi = {
  list: async (req: ListSessionsRequest): Promise<ListSessionsResponse> => {
    const params = new URLSearchParams();

    if (req.page !== undefined) params.append("page", req.page.toString());
    if (req.pageSize !== undefined) params.append("pageSize", req.pageSize.toString());

    if (req.filter) {
      if (req.filter.modelIds && req.filter.modelIds.length > 0) {
        params.append("modelIds", req.filter.modelIds.join(","));
      }
      if (req.filter.maneuverIds && req.filter.maneuverIds.length > 0) {
        params.append("maneuverIds", req.filter.maneuverIds.join(","));
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

    const { data } = await apiClient.get<ListSessionsResponse>("/sessions", { params });
    return data;
  },

  create: async (payload: CreateSessionRequest): Promise<CreateSessionDto> => {
    const { data } = await apiClient.post<CreateSessionDto>("/sessions", payload);
    return data;
  },

  update: async (sessionId: string, payload: UpdateSessionRequest): Promise<UpdateSessionDto> => {
    const { data } = await apiClient.put<UpdateSessionDto>(
      `/sessions/${encodeURIComponent(sessionId)}`,
      payload,
    );
    return data;
  },

  delete: async (sessionId: string): Promise<void> => {
    await apiClient.delete(`/sessions/${encodeURIComponent(sessionId)}`);
  },

  addPerformedVariation: async (
    params: AddPerformedVariationParams,
  ): Promise<AddPerformedVariationDto> => {
    const { data } = await apiClient.post<AddPerformedVariationDto>(
      `/sessions/${encodeURIComponent(params.sessionId)}/performed-variations`,
      params.payload,
    );
    return data;
  },

  updatePerformedVariation: async (params: UpdatePerformedVariationParams): Promise<void> => {
    await apiClient.put(
      `/sessions/${encodeURIComponent(params.sessionId)}/performed-variations/${encodeURIComponent(params.performedVariationId)}`,
      params.payload,
    );
  },

  removePerformedVariation: async (
    sessionId: string,
    performedVariationId: string,
  ): Promise<void> => {
    await apiClient.delete(
      `/sessions/${encodeURIComponent(sessionId)}/performed-variations/${encodeURIComponent(performedVariationId)}`,
    );
  },
};
