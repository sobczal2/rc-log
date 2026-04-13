import axios from "axios";

export function getApiErrorMessage(err: unknown): string | null {
  if (axios.isAxiosError(err)) {
    const msg = err.response?.data?.error;
    if (typeof msg === "string" && msg.length > 0) {
      return msg;
    }
  }
  return null;
}
