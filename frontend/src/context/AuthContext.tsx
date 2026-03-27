import { createContext, useCallback, useContext, useState } from "react";
import type { ReactNode } from "react";
import type { User } from "@/domain/user";
import type { SignInRequest, SignUpRequest } from "@/lib/api/auth";
import { authApi } from "@/lib/api/auth";

interface AuthContextValue {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
  signIn: (req: SignInRequest) => Promise<void>;
  signUp: (req: SignUpRequest) => Promise<void>;
  signOut: () => void;
}

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(() => {
    const stored = localStorage.getItem("user");
    return stored ? (JSON.parse(stored) as User) : null;
  });

  const [token, setToken] = useState<string | null>(
    () => localStorage.getItem("token"),
  );

  const handleAuthResponse = useCallback((newToken: string, newUser: User) => {
    localStorage.setItem("token", newToken);
    localStorage.setItem("user", JSON.stringify(newUser));
    setToken(newToken);
    setUser(newUser);
  }, []);

  const signIn = useCallback(
    async (req: SignInRequest) => {
      const { token: newToken, user: newUser } = await authApi.signIn(req);
      handleAuthResponse(newToken, newUser);
    },
    [handleAuthResponse],
  );

  const signUp = useCallback(
    async (req: SignUpRequest) => {
      const { token: newToken, user: newUser } = await authApi.signUp(req);
      handleAuthResponse(newToken, newUser);
    },
    [handleAuthResponse],
  );

  const signOut = useCallback(() => {
    localStorage.removeItem("token");
    localStorage.removeItem("user");
    setToken(null);
    setUser(null);
  }, []);

  return (
    <AuthContext.Provider
      value={{
        user,
        token,
        isAuthenticated: !!token,
        signIn,
        signUp,
        signOut,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth(): AuthContextValue {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
}
