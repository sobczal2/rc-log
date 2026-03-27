import { createContext } from "react";
import type { User } from "@/models/user";
import type { SignInRequest, SignUpRequest } from "@/lib/api/auth";

export interface AuthContextValue {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
  signIn: (req: SignInRequest) => Promise<void>;
  signUp: (req: SignUpRequest) => Promise<void>;
  signOut: () => void;
}

export const AuthContext = createContext<AuthContextValue | null>(null);
