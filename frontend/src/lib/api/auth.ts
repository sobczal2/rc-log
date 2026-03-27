import type { User } from "@/domain/user";
import { apiClient } from "../apiClient";

export interface SignInRequest {
  username: string;
  password: string;
}

export interface SignUpRequest {
  username: string;
  email: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

export const authApi = {
  signIn: async (req: SignInRequest): Promise<AuthResponse> => {
    const { data } = await apiClient.post<AuthResponse>("/auth/sign-in", req);
    return data;
  },

  signUp: async (req: SignUpRequest): Promise<AuthResponse> => {
    const { data } = await apiClient.post<AuthResponse>("/auth/sign-up", req);
    return data;
  },
};
