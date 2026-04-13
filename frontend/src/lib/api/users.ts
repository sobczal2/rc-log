import { apiClient } from "../apiClient";
import type { User } from "@/models/user";

export interface UpdateUserRequest {
  newUsername: string;
}

export const usersApi = {
  update: async (req: UpdateUserRequest): Promise<User> => {
    const { data } = await apiClient.put<User>("/users/me", req);
    return data;
  },

  updatePhoto: async (file: File): Promise<User> => {
    const fd = new FormData();
    fd.append("photo", file);
    const { data } = await apiClient.put<User>("/users/me/photo", fd, {
      headers: { "Content-Type": undefined },
    });
    return data;
  },

  removePhoto: async (): Promise<void> => {
    await apiClient.delete("/users/me/photo");
  },
};
