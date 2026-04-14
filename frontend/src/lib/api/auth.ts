import type { SignInUserDto } from "@/models/user/sign-in";
import type { SignUpUserDto } from "@/models/user/sign-up";
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

export interface SignInResponse {
  token: string;
  user: SignInUserDto;
}

export interface SignUpResponse {
  token: string;
  user: SignUpUserDto;
}

export const authApi = {
  signIn: async (req: SignInRequest): Promise<SignInResponse> => {
    const { data } = await apiClient.post<SignInResponse>("/auth/sign-in", req);
    return data;
  },

  signUp: async (req: SignUpRequest): Promise<SignUpResponse> => {
    const { data } = await apiClient.post<SignUpResponse>("/auth/sign-up", req);
    return data;
  },
};
