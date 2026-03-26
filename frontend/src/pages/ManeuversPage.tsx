import { useQuery } from "@tanstack/react-query";
import { ManeuverCard } from "@/components/maneuvers/ManeuverCard";
import { maneuversApi } from "@/lib/api/maneuvers";
import { mapManeuverDto } from "@/domain/maneuver/mapper";
import { Loader2 } from "lucide-react";

export function ManeuversPage() {
  const { data, isLoading, isError, error } = useQuery({
    queryKey: ['maneuvers', { page: 1, pageSize: 20 }],
    queryFn: () => maneuversApi.list({ page: 1, pageSize: 20 }),
  });

  const maneuvers = data?.items.map(mapManeuverDto) || [];

  return (
    <div className="p-4 md:p-8 flex flex-col gap-6 h-full w-full">
      <div className="flex items-end justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight mb-2">Maneuvers Catalog</h1>
          <p className="text-muted-foreground max-w-2xl text-sm">
            Browse and learn different maneuvers for your RC vehicles.
          </p>
        </div>
      </div>
      
      {isLoading ? (
        <div className="flex-1 flex items-center justify-center p-12">
            <Loader2 className="animate-spin text-muted-foreground w-8 h-8" />
        </div>
      ) : isError ? (
        <div className="p-6 bg-destructive/10 text-destructive rounded-xl border border-destructive/20 text-center">
            {error instanceof Error ? error.message : "Failed to load maneuvers"}
        </div>
      ) : maneuvers.length === 0 ? (
        <div className="flex-1 flex items-center justify-center p-12 text-center text-muted-foreground">
            No maneuvers found in the database.
        </div>
      ) : (
        <div className="grid grid-cols-1 xs:grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
          {maneuvers.map(m => (
            <ManeuverCard key={m.id} maneuver={m} />
          ))}
        </div>
      )}
    </div>
  );
}
