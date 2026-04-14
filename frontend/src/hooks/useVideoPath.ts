import { useQuery } from "@tanstack/react-query";
import { assetsApi } from "@/lib/api/assets";
import type { VideoPathsDto } from "@/models/asset/video";

export type { VideoPathsDto };

export function useVideoPath(assetId: string | null) {
  return useQuery({
    queryKey: ["video-path", assetId],
    queryFn: () => assetsApi.getVideoPath(assetId!),
    staleTime: 5 * 60 * 1000,
    enabled: !!assetId,
  });
}
