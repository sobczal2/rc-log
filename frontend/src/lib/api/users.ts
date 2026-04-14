import { apiClient } from "../apiClient";
import type { UpdateUserRequest, UpdateUserDto } from "@/models/user/update";
import type { UpdateUserPhotoDto } from "@/models/user/update-photo";

export const usersApi = {
  update: async (req: UpdateUserRequest): Promise<UpdateUserDto> => {
    const { data } = await apiClient.put<UpdateUserDto>("/users/me", req);
    return data;
  },

  updatePhoto: async (file: File): Promise<UpdateUserPhotoDto> => {
    const fd = new FormData();
    fd.append("photo", file);
    const { data } = await apiClient.put<UpdateUserPhotoDto>("/users/me/photo", fd);
    return data;
  },

  removePhoto: async (): Promise<void> => {
    await apiClient.delete("/users/me/photo");
  },
};
