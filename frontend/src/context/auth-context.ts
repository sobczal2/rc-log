import { createContext } from "react";
import type { SignInRequest, SignUpRequest } from "@/lib/api/auth";

export interface User {
  id: string;
  username: string;
  email: string;
  photoAssetId?: string | null;
}

export interface AuthContextValue {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
  signIn: (req: SignInRequest) => Promise<void>;
  signUp: (req: SignUpRequest) => Promise<void>;
  signOut: () => void;
  updateUser: (user: User) => void;
}

export const AuthContext = createContext<AuthContextValue | null>(null);
