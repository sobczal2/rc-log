export type { VideoPathsDto } from "@/models/__generated/asset/video";

export function getVideoUrl(smallPath: string): string {
  return `/api/assets/${smallPath}`;
}
