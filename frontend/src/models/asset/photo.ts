export type { PhotoPathsDto } from "@/models/__generated/asset/photo";

export function getPhotoUrl(path: string): string {
  return `/api/assets/${path}`;
}
