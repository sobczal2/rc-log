import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Link } from "react-router-dom";
import { Plane, Helicopter, Drone } from "lucide-react";
import { type ManeuverDto, getDifficultyInfo } from "@/domain/maneuver";

export function ManeuverCard({ maneuver }: { maneuver: ManeuverDto }) {
  // Map difficulty to a color or indicator
  const difficultyColors: Record<string, string> = {
    Level1: "bg-green-500/10 text-green-500 border-green-500/20",
    Level2: "bg-emerald-500/10 text-emerald-500 border-emerald-500/20",
    Level3: "bg-yellow-500/10 text-yellow-500 border-yellow-500/20",
    Level4: "bg-amber-500/10 text-amber-500 border-amber-500/20",
    Level5: "bg-orange-500/10 text-orange-500 border-orange-500/20",
    Level6: "bg-red-500/10 text-red-500 border-red-500/20",
    Level7: "bg-rose-600/10 text-rose-600 border-rose-600/20",
  };

  const getVehicleIcon = (size = 14) => {
    switch (maneuver.vehicleType) {
      case "Plane": return <Plane size={size} />;
      case "Helicopter": return <Helicopter size={size} />;
      case "Drone": return <Drone size={size} />;
    }
  }

  return (
    <Link to={`/maneuvers/${maneuver.id}`}>
      <Card className="group relative overflow-hidden flex flex-col aspect-square hover:border-sidebar-ring transition-colors bg-card shadow-sm cursor-pointer p-0 gap-0">
        <div className="relative w-full h-[55%] flex-shrink-0 bg-muted/30 overflow-hidden border-b border-border/50">
          {maneuver.videoUrl ? (
            <video
              src={maneuver.videoUrl}
              className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-700 ease-out"
              autoPlay
              loop
              muted
              playsInline
            />
          ) : (
            <div className="w-full h-full flex items-center justify-center text-muted-foreground/20 group-hover:scale-105 transition-transform duration-700 ease-out">
              {getVehicleIcon(48)}
            </div>
          )}
        </div>
        <CardContent className="flex flex-col flex-1 p-3.5 gap-2 relative">
          <div className="flex items-start justify-between gap-2">
            <h3 className="font-bold text-[15px] leading-tight line-clamp-2">{maneuver.name}</h3>
            <div className="flex-shrink-0 text-muted-foreground/70 mt-0.5">
              {getVehicleIcon(16)}
            </div>
          </div>
          <div className="flex flex-wrap gap-1.5 mt-1 pr-10">
            {maneuver.tags.map(tag => (
              <Badge key={tag.id} variant="secondary" className="text-[10px] px-1.5 py-0 h-4 rounded-sm bg-secondary/50 font-normal">
                {tag.name}
              </Badge>
            ))}
          </div>
          <div className="absolute bottom-3.5 right-3.5 flex mt-auto ml-auto">
            <Badge variant="outline" className={`px-1.5 py-0.5 rounded-sm font-mono font-bold ${difficultyColors[maneuver.difficulty]}`}>
              L{getDifficultyInfo(maneuver.vehicleType, maneuver.difficulty).levelNumber}: {getDifficultyInfo(maneuver.vehicleType, maneuver.difficulty).name}
            </Badge>
          </div>
        </CardContent>
      </Card>
    </Link>
  );
}
