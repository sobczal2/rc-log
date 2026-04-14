import { apiClient } from "../apiClient";
import type { ListModelDto } from "@/models/model/list";
import type { CreateModelRequest, CreateModelDto } from "@/models/model/create";
import type { GetByIdModelDto } from "@/models/model/get-by-id";
import type { UpdateModelRequest, UpdateModelDto } from "@/models/model/update";
import type { UpdateModelPhotoDto } from "@/models/model/update-photo";
import type { PaginatedResult } from "@/models/shared/pagination";

export interface ListModelsRequest {
  page?: number;
  pageSize?: number;
}

export const modelsApi = {
  list: async (req: ListModelsRequest = {}): Promise<PaginatedResult<ListModelDto>> => {
    const params = new URLSearchParams();
    params.set("page", String(req.page ?? 1));
    params.set("pageSize", String(req.pageSize ?? 20));
    const { data } = await apiClient.get<PaginatedResult<ListModelDto>>("/models", { params });
    return data;
  },

  getById: async (id: string): Promise<GetByIdModelDto> => {
    const { data } = await apiClient.get<GetByIdModelDto>(`/models/${encodeURIComponent(id)}`);
    return data;
  },

  create: async (req: CreateModelRequest): Promise<CreateModelDto> => {
    const { data } = await apiClient.post<CreateModelDto>("/models", req);
    return data;
  },

  update: async (id: string, req: UpdateModelRequest): Promise<UpdateModelDto> => {
    const { data } = await apiClient.put<UpdateModelDto>(`/models/${encodeURIComponent(id)}`, req);
    return data;
  },

  delete: async (id: string): Promise<void> => {
    await apiClient.delete(`/models/${encodeURIComponent(id)}`);
  },

  updatePhoto: async (id: string, file: File): Promise<UpdateModelPhotoDto> => {
    const fd = new FormData();
    fd.append("photo", file);
    const { data } = await apiClient.put<UpdateModelPhotoDto>(
      `/models/${encodeURIComponent(id)}/photo`,
      fd,
    );
    return data;
  },

  removePhoto: async (id: string): Promise<void> => {
    await apiClient.delete(`/models/${encodeURIComponent(id)}/photo`);
  },
};
