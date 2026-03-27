import { useParams, Link } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import ReactMarkdown from "react-markdown";
import { maneuversApi } from "@/lib/api/maneuvers";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { ArrowLeft, Loader2, AlertCircle } from "lucide-react";
import { getVehicleIcon, getDifficultyLevelName } from "@/models/shared";
import { useVideoPath } from "@/hooks/useVideoPath";
import { getVideoUrl } from "@/models/asset/video";

export function ManeuverDetailsPage() {
  const { id } = useParams<{ id: string }>();

  const {
    data: maneuver,
    isLoading,
    isError,
    error,
  } = useQuery({
    queryKey: ["maneuver", id],
    queryFn: () => maneuversApi.getById({ id: id! }),
    enabled: !!id,
  });

  const assetName = maneuver?.defaultVariation?.videoAssetName ?? "";
  const { data: videoPathData } = useVideoPath(assetName);
  const videoSrc = videoPathData ? getVideoUrl(videoPathData.smallPath) : null;

  if (isLoading) {
    return (
      <div className="flex-1 flex items-center justify-center p-12 h-full">
        <Loader2 className="animate-spin text-muted-foreground w-8 h-8" />
      </div>
    );
  }

  if (isError || !maneuver) {
    return (
      <div className="p-8 text-center flex flex-col items-center justify-center h-full gap-4">
        <AlertCircle className="w-12 h-12 text-destructive/50" />
        <h2 className="text-2xl font-bold">
          {error instanceof Error ? error.message : "Maneuver not found"}
        </h2>
        <Button variant="outline" render={<Link to="/maneuvers" />}>
          <ArrowLeft className="mr-2" size={16} /> Back to Maneuvers
        </Button>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full w-full overflow-hidden">
      <div className="flex-1 overflow-auto custom-scrollbar">
        <div className="max-w-5xl mx-auto p-4 md:p-8">
          <div className="mb-6">
            <Button
              nativeButton={false}
              variant="ghost"
              size="sm"
              className="-ml-3 text-muted-foreground hover:text-foreground"
              render={<Link to="/maneuvers" />}
            >
              <ArrowLeft className="mr-2" size={16} /> Back to maneuvers
            </Button>
          </div>
          <div className="flex flex-col gap-8 lg:gap-12">
            <div className="bg-muted/30 rounded-xl overflow-hidden border border-border/50 shadow-sm relative">
              {videoSrc ? (
                <video
                  src={videoSrc}
                  className="w-full aspect-video object-cover"
                  autoPlay
                  loop
                  muted
                  playsInline
                />
              ) : (
                <div className="w-full aspect-video flex items-center justify-center text-muted-foreground/20">
                  {getVehicleIcon(maneuver.vehicleType, 64)}
                </div>
              )}
            </div>

            <div className="flex flex-col md:flex-row gap-8 lg:gap-12">
              <div className="w-full md:w-1/3 self-start flex flex-col gap-3 p-5 bg-card/50 shadow-sm border border-border/50 rounded-xl">
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2 text-sm font-medium text-foreground">
                    <span className="text-muted-foreground">
                      {getVehicleIcon(maneuver.vehicleType)}
                    </span>
                    <span>{maneuver.vehicleType}</span>
                  </div>
                </div>
                <div className="font-bold flex items-center gap-2">
                  <Badge variant="outline" className="font-mono bg-background">
                    {maneuver.difficulty.replace("Level", "L")}
                  </Badge>
                  <span>
                    {getDifficultyLevelName(
                      maneuver.vehicleType,
                      maneuver.difficulty,
                    )}
                  </span>
                </div>
                <div className="flex flex-wrap gap-1.5 mt-2">
                  {maneuver.tags.map((tag) => (
                    <Badge
                      key={tag.id}
                      variant="secondary"
                      className="font-normal border-border/50"
                    >
                      {tag.name}
                    </Badge>
                  ))}
                </div>
              </div>

              <div className="w-full md:w-2/3 flex flex-col pb-16">
                <h1 className="text-3xl md:text-5xl font-bold tracking-tight mb-8 leading-tight">
                  {maneuver.name}
                </h1>

                <article className="prose prose-zinc dark:prose-invert prose-headings:font-bold prose-h2:text-2xl mt-2 w-full max-w-none prose-a:text-primary">
                  <ReactMarkdown>{maneuver.description}</ReactMarkdown>
                </article>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
