import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Link } from "react-router-dom";
import { getModelTypeIcon } from "@/models/model/type";
import { DifficultyRangeBadgeShort } from "@/components/ui/difficulty-badge";
import type { ListManeuverDto } from "@/models/maneuver";
import { useVideoPath } from "@/hooks/useVideoPath";
import { getVideoUrl } from "@/models/asset/video";

export function ManeuverCard({ maneuver }: { maneuver: ListManeuverDto }) {
  const { data: videoPathData } = useVideoPath(maneuver.defaultVariationVideoAssetName);
  const videoSrc = videoPathData ? getVideoUrl(videoPathData.smallPath) : null;
  return (
    <Link to={`/maneuvers/${maneuver.id}`}>
      <Card className="group relative overflow-hidden flex flex-col aspect-square transition-colors bg-card shadow-sm cursor-pointer p-0 gap-0">
        <div className="relative w-full h-[55%] flex-shrink-0 bg-muted/30 overflow-hidden border-b border-border/50">
          {videoSrc ? (
            <video
              src={videoSrc}
              className="w-full h-full object-cover transition-transform duration-700 ease-out"
              autoPlay
              loop
              muted
              playsInline
            />
          ) : (
            <div className="w-full h-full flex items-center justify-center text-muted-foreground/20 transition-transform duration-700 ease-out">
              {getModelTypeIcon(maneuver.type, 48)}
            </div>
          )}
        </div>
        <CardContent className="flex flex-col flex-1 p-3.5 gap-2 relative">
          <div className="flex items-start justify-between gap-2">
            <h3 className="font-bold text-[15px] leading-tight line-clamp-2">{maneuver.name}</h3>
            <div className="flex-shrink-0 text-muted-foreground/70 mt-0.5">
              {getModelTypeIcon(maneuver.type, 16)}
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
            <DifficultyRangeBadgeShort
              minDifficulty={maneuver.minDifficulty}
              maxDifficulty={maneuver.maxDifficulty}
              className="px-1.5 py-0.5"
            />
          </div>
        </CardContent>
      </Card>
    </Link>
  );
}
