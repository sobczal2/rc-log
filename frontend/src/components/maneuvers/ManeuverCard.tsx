import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Link } from "react-router-dom";
import {
  getDifficultyLevelColor,
  getDifficultyLevelLabel,
  getVehicleIcon,
} from "@/models/shared";
import type { ListManeuverDto } from "@/models/maneuver";
import { useVideoPath } from "@/hooks/useVideoPath";
import { getVideoUrl } from "@/models/asset/video";

export function ManeuverCard({ maneuver }: { maneuver: ListManeuverDto }) {
  const { data: videoPathData } = useVideoPath(
    maneuver.defaultVariationVideoAssetName,
  );
  const videoSrc = videoPathData ? getVideoUrl(videoPathData.smallPath) : null;
  return (
    <Link to={`/maneuvers/${maneuver.id}`}>
      <Card className="group relative overflow-hidden flex flex-col aspect-square hover:border-sidebar-ring transition-colors bg-card shadow-sm cursor-pointer p-0 gap-0">
        <div className="relative w-full h-[55%] flex-shrink-0 bg-muted/30 overflow-hidden border-b border-border/50">
          {videoSrc ? (
            <video
              src={videoSrc}
              className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-700 ease-out"
              autoPlay
              loop
              muted
              playsInline
            />
          ) : (
            <div className="w-full h-full flex items-center justify-center text-muted-foreground/20 group-hover:scale-105 transition-transform duration-700 ease-out">
              {getVehicleIcon(maneuver.vehicleType, 48)}
            </div>
          )}
        </div>
        <CardContent className="flex flex-col flex-1 p-3.5 gap-2 relative">
          <div className="flex items-start justify-between gap-2">
            <h3 className="font-bold text-[15px] leading-tight line-clamp-2">
              {maneuver.name}
            </h3>
            <div className="flex-shrink-0 text-muted-foreground/70 mt-0.5">
              {getVehicleIcon(maneuver.vehicleType, 16)}
            </div>
          </div>
          <div className="flex flex-wrap gap-1.5 mt-1 pr-10">
            {maneuver.tags.map((tag) => (
              <Badge
                key={tag.id}
                variant="secondary"
                className="text-[10px] px-1.5 py-0 h-4 rounded-sm bg-secondary/50 font-normal"
              >
                {tag.name}
              </Badge>
            ))}
          </div>
          <div className="absolute bottom-3.5 right-3.5 flex mt-auto ml-auto">
            <Badge
              variant="outline"
              className={`px-1.5 py-0.5 rounded-sm font-mono font-bold ${getDifficultyLevelColor(maneuver.difficulty)}`}
            >
              {getDifficultyLevelLabel(
                maneuver.vehicleType,
                maneuver.difficulty,
              )}
            </Badge>
          </div>
        </CardContent>
      </Card>
    </Link>
  );
}
