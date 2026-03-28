import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { ChevronLeft, ChevronRight, Loader2 } from "lucide-react";
import { ManeuverCard } from "@/components/maneuvers/ManeuverCard";
import { ManeuverFilters } from "@/components/maneuvers/ManeuverFilters";
import { maneuversApi } from "@/lib/api/maneuvers";
import { useManeuverFilters } from "@/hooks/useManeuverFilters";
import { Button } from "@/components/ui/button";

export function ManeuversPage() {
  const [filters, actions] = useManeuverFilters();
  const [isFilterOpen, setIsFilterOpen] = useState(false);

  const { data, isLoading, isError, error, isFetching } = useQuery({
    queryKey: [
      "maneuvers",
      {
        page: filters.page,
        searchQuery: filters.searchQuery,
        vehicleType: filters.vehicleType,
        difficulty: filters.difficulty,
        sortField: filters.sortField,
        sortDirection: filters.sortDirection,
      },
    ],
    queryFn: () =>
      maneuversApi.list({
        page: filters.page,
        pageSize: 20,
        filter: {
          searchQuery: filters.searchQuery || undefined,
          vehicleType: filters.vehicleType || undefined,
          difficulty: filters.difficulty || undefined,
        },
        sort: {
          field: filters.sortField,
          direction: filters.sortDirection,
        },
      }),
  });

  const maneuvers = data?.items || [];
  const totalPages = data?.totalPages || 1;
  const totalItems = data?.total || 0;

  const hasActiveFilters = filters.searchQuery || filters.vehicleType || filters.difficulty;

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

      <ManeuverFilters
        filters={filters}
        actions={actions}
        isOpen={isFilterOpen}
        onToggle={() => setIsFilterOpen(!isFilterOpen)}
      />

      {isLoading ? (
        <div className="flex-1 flex items-center justify-center p-12">
          <Loader2 className="animate-spin text-muted-foreground w-8 h-8" />
        </div>
      ) : isError ? (
        <div className="p-6 bg-destructive/10 text-destructive rounded-xl border border-destructive/20 text-center">
          {error instanceof Error ? error.message : "Failed to load maneuvers"}
        </div>
      ) : maneuvers.length === 0 ? (
        <div className="flex-1 flex flex-col items-center justify-center p-12 text-center">
          <p className="text-muted-foreground mb-4">
            {hasActiveFilters
              ? "No maneuvers match your filters."
              : "No maneuvers found in the database."}
          </p>
          {hasActiveFilters && (
            <Button variant="outline" onClick={() => actions.clearAll()}>
              Clear all filters
            </Button>
          )}
        </div>
      ) : (
        <>
          <div className="flex items-center justify-between text-sm text-muted-foreground">
            <span>
              Showing {(filters.page - 1) * 20 + 1} - {Math.min(filters.page * 20, totalItems)} of{" "}
              {totalItems} maneuvers
            </span>
            {isFetching && !isLoading && (
              <span className="flex items-center gap-1">
                <Loader2 className="h-3 w-3 animate-spin" />
                Updating...
              </span>
            )}
          </div>

          <div className="grid grid-cols-1 xs:grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
            {maneuvers.map((m) => (
              <ManeuverCard key={m.id} maneuver={m} />
            ))}
          </div>

          {totalPages > 1 && (
            <div className="flex items-center justify-center gap-2 mt-4">
              <Button
                variant="outline"
                size="icon-sm"
                onClick={() => actions.setPage(filters.page - 1)}
                disabled={filters.page <= 1}
              >
                <ChevronLeft className="h-4 w-4" />
                <span className="sr-only">Previous page</span>
              </Button>

              <span className="text-sm text-muted-foreground px-4">
                Page {filters.page} of {totalPages}
              </span>

              <Button
                variant="outline"
                size="icon-sm"
                onClick={() => actions.setPage(filters.page + 1)}
                disabled={filters.page >= totalPages}
              >
                <ChevronRight className="h-4 w-4" />
                <span className="sr-only">Next page</span>
              </Button>
            </div>
          )}
        </>
      )}
    </div>
  );
}
