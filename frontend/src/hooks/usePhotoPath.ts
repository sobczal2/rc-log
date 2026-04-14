import { useQuery } from "@tanstack/react-query";
import { assetsApi } from "@/lib/api/assets";
import type { PhotoPathsDto } from "@/models/asset/photo";

export type { PhotoPathsDto };

export function usePhotoPath(assetId: string | null) {
  return useQuery({
    queryKey: ["photo-path", assetId],
    queryFn: () => assetsApi.getPhotoPath(assetId!),
    staleTime: 5 * 60 * 1000,
    enabled: !!assetId,
  });
}
