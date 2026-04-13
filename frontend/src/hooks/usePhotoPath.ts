import { useQuery } from "@tanstack/react-query";
import { assetsApi } from "@/lib/api/assets";
import type { PhotoPathsDto } from "@/models/asset/photo";

export type { PhotoPathsDto };

export function usePhotoPath(assetName: string | null) {
  return useQuery({
    queryKey: ["photo-path", assetName],
    queryFn: () => assetsApi.getPhotoPath(assetName!),
    staleTime: 5 * 60 * 1000,
    enabled: !!assetName,
  });
}
