import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import ReactMarkdown from "react-markdown";
import type { GetByIdVariationDto } from "@/models/maneuver";
import { useVideoPath } from "@/hooks/useVideoPath";
import { getVideoUrl } from "@/models/asset/video";

interface VariationCardProps {
  variation: GetByIdVariationDto;
  isDefault?: boolean;
}

export function VariationCard({ variation, isDefault }: VariationCardProps) {
  const { data: videoPathData } = useVideoPath(variation.videoAssetName);
  const smallSrc = videoPathData ? getVideoUrl(videoPathData.smallPath) : null;
  const largeSrc = videoPathData
    ? getVideoUrl(
        videoPathData.largePath ??
          videoPathData.mediumPath ??
          videoPathData.smallPath,
      )
    : null;

  return (
    <Dialog>
      <DialogTrigger className="text-left w-full">
        <Card className="group relative overflow-hidden flex flex-col hover:border-sidebar-ring transition-colors bg-card shadow-sm p-0 gap-0 cursor-pointer">
          <div className="relative w-full aspect-video flex-shrink-0 bg-muted/30 overflow-hidden border-b border-border/50">
            {smallSrc ? (
              <video
                src={smallSrc}
                className="w-full h-full object-cover transition-transform duration-700 ease-out"
                autoPlay
                loop
                muted
                playsInline
              />
            ) : (
              <div className="w-full h-full transition-transform duration-700 ease-out" />
            )}
          </div>
          <CardContent className="flex items-center justify-between p-2.5 gap-2">
            <span className="font-semibold text-[13px] leading-tight line-clamp-1">
              {variation.name}
            </span>
            {isDefault && (
              <Badge
                variant="secondary"
                className="text-[10px] px-1.5 py-0 h-4 rounded-sm flex-shrink-0 font-normal"
              >
                Default
              </Badge>
            )}
          </CardContent>
        </Card>
      </DialogTrigger>

      <DialogContent className="sm:max-w-2xl p-0 gap-0 overflow-hidden">
        <div className="w-full aspect-video bg-muted/30">
          {largeSrc ? (
            <video
              src={largeSrc}
              className="w-full h-full object-cover"
              autoPlay
              loop
              muted
              playsInline
            />
          ) : (
            <div className="w-full h-full" />
          )}
        </div>
        <div className="p-5 flex flex-col gap-3">
          <DialogHeader>
            <DialogTitle className="flex items-center gap-2 text-base">
              {variation.name}
              {isDefault && (
                <Badge
                  variant="secondary"
                  className="text-[10px] px-1.5 py-0 h-4 rounded-sm font-normal"
                >
                  Default
                </Badge>
              )}
            </DialogTitle>
          </DialogHeader>
          <article className="prose prose-zinc dark:prose-invert prose-sm mt-1 w-full max-w-none">
            <ReactMarkdown>{variation.description}</ReactMarkdown>
          </article>
        </div>
      </DialogContent>
    </Dialog>
  );
}
