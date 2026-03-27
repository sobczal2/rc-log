import { apiClient } from "../apiClient";
import type { VideoPathsDto } from "@/models/asset/video";
import type { PhotoPathsDto } from "@/models/asset/photo";

export type { VideoPathsDto, PhotoPathsDto };

export const assetsApi = {
  getVideoPath: async (name: string): Promise<VideoPathsDto> => {
    const { data } = await apiClient.get<VideoPathsDto>(
      `/asset-paths/video/${encodeURIComponent(name)}`,
    );
    return data;
  },

  getPhotoPath: async (name: string): Promise<PhotoPathsDto> => {
    const { data } = await apiClient.get<PhotoPathsDto>(
      `/asset-paths/photo/${encodeURIComponent(name)}`,
    );
    return data;
  },
};
