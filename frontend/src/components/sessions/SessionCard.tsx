import { Link } from "react-router-dom";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import type { ListSessionDto } from "@/models/session";
import { getModelTypeIcon, getModelTypeLabel } from "@/models/model/type";
import { ratingToNumber } from "@/models/session";
import { usePhotoPath } from "@/hooks/usePhotoPath";
import { getPhotoUrl } from "@/models/asset/photo";

function formatDate(date: string): string {
  const d = new Date(`${date}T00:00:00`);
  return d.toLocaleDateString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
    weekday: "short",
  });
}

export function SessionCard({ session }: { session: ListSessionDto }) {
  const { data: photoPathData } = usePhotoPath(session.modelPhotoAssetId);
  const photoSrc = photoPathData ? getPhotoUrl(photoPathData.smallPath) : null;

  const maneuvers = session.performedVariations
    .map((v) => v.maneuverName)
    .filter((name): name is string => !!name);
  const uniqueManeuvers = Array.from(new Set(maneuvers));

  const variations = session.performedVariations
    .map((v) => v.variationName)
    .filter((name): name is string => !!name);

  const avgRating =
    session.performedVariations.length === 0
      ? null
      : session.performedVariations.reduce((acc, item) => {
          const score =
            ratingToNumber(item.quality) +
            ratingToNumber(item.comfort) +
            ratingToNumber(item.repeatability);
          return acc + score / 3;
        }, 0) / session.performedVariations.length;

  return (
    <Link to={`/sessions/${session.id}`} state={{ session }}>
      <Card className="group overflow-hidden border-border/60 transition-colors hover:border-border">
        <CardHeader className="pb-3">
          <div className="flex items-start justify-between gap-3">
            <div>
              <CardTitle className="text-base">{formatDate(session.date)}</CardTitle>
              <p className="text-xs text-muted-foreground mt-1">
                {session.performedVariations.length} variation
                {session.performedVariations.length === 1 ? "" : "s"} logged
              </p>
            </div>
            {avgRating !== null && <Badge variant="secondary">Avg {avgRating.toFixed(1)}/5</Badge>}
          </div>
        </CardHeader>

        <CardContent className="pt-0 flex flex-col gap-3">
          <div className="flex items-center gap-3 min-w-0">
            <div className="w-14 h-14 shrink-0 border border-border/60 bg-muted/20 overflow-hidden flex items-center justify-center">
              {photoSrc ? (
                <img
                  src={photoSrc}
                  alt={session.modelName ?? "Model photo"}
                  className="w-full h-full object-cover"
                />
              ) : session.modelType ? (
                <div className="text-muted-foreground/70">
                  {getModelTypeIcon(session.modelType, 20)}
                </div>
              ) : (
                <div className="text-muted-foreground/40 text-xs">No model</div>
              )}
            </div>
            <div className="min-w-0">
              <p className="text-sm font-semibold truncate">
                {session.modelName ?? "Simulator / freestyle session"}
              </p>
              <p className="text-xs text-muted-foreground flex items-center gap-1">
                {session.modelType ? (
                  <>
                    {getModelTypeIcon(session.modelType, 12)}
                    <span>{getModelTypeLabel(session.modelType)}</span>
                  </>
                ) : (
                  <span>Model type inferred from maneuvers</span>
                )}
              </p>
            </div>
          </div>

          {uniqueManeuvers.length > 0 && (
            <div className="flex flex-wrap gap-1.5">
              {uniqueManeuvers.slice(0, 3).map((name) => (
                <Badge key={name} variant="outline">
                  {name}
                </Badge>
              ))}
              {uniqueManeuvers.length > 3 && (
                <Badge variant="ghost">+{uniqueManeuvers.length - 3} more</Badge>
              )}
            </div>
          )}

          {variations.length > 0 && (
            <p className="text-xs text-muted-foreground truncate">
              Variations: {Array.from(new Set(variations)).slice(0, 2).join(", ")}
              {Array.from(new Set(variations)).length > 2 ? "…" : ""}
            </p>
          )}
        </CardContent>
      </Card>
    </Link>
  );
}
