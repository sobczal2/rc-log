import { useQuery } from "@tanstack/react-query";
import { assetsApi } from "@/lib/api/assets";
import type { VideoPathsDto } from "@/models/asset/video";

export type { VideoPathsDto };

export function useVideoPath(assetName: string | null) {
  return useQuery({
    queryKey: ["video-path", assetName],
    queryFn: () => assetsApi.getVideoPath(assetName!),
    staleTime: 5 * 60 * 1000,
    enabled: !!assetName,
  });
}
