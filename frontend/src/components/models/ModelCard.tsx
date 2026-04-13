import { Card, CardContent } from "@/components/ui/card";
import { Link } from "react-router-dom";
import type { ListModelDto } from "@/models/model";
import { getVehicleIcon } from "@/models/shared";
import { usePhotoPath } from "@/hooks/usePhotoPath";
import { getPhotoUrl } from "@/models/asset/photo";

export function ModelCard({ model }: { model: ListModelDto }) {
  const { data: photoPathData } = usePhotoPath(model.photoAssetName);
  const photoSrc = photoPathData ? getPhotoUrl(photoPathData.smallPath) : null;

  return (
    <Link to={`/profile/models/${model.id}`}>
      <Card className="group relative overflow-hidden flex flex-col aspect-square transition-colors bg-card shadow-sm cursor-pointer p-0 gap-0">
        <div className="relative w-full h-[55%] flex-shrink-0 bg-muted/30 overflow-hidden border-b border-border/50">
          {photoSrc ? (
            <img
              src={photoSrc}
              alt={model.name}
              className="w-full h-full object-cover transition-transform duration-700 ease-out group-hover:scale-105"
            />
          ) : (
            <div className="w-full h-full flex items-center justify-center text-muted-foreground/20">
              {getVehicleIcon(model.vehicleType, 48)}
            </div>
          )}
        </div>
        <CardContent className="flex flex-col flex-1 p-3.5 gap-2 relative">
          <div className="flex items-start justify-between gap-2">
            <h3 className="font-bold text-[15px] leading-tight line-clamp-2">{model.name}</h3>
            <div className="flex-shrink-0 text-muted-foreground/70 mt-0.5">
              {getVehicleIcon(model.vehicleType, 16)}
            </div>
          </div>
        </CardContent>
      </Card>
    </Link>
  );
}
