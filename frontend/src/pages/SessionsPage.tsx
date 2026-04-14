import { useMemo, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Loader2, ChevronLeft, ChevronRight, Search } from "lucide-react";
import { sessionsApi } from "@/lib/api/sessions";
import { modelsApi } from "@/lib/api/models";
import { maneuversApi } from "@/lib/api/maneuvers";
import { useDebounce } from "@/hooks/useDebounce";
import { SessionCard } from "@/components/sessions/SessionCard";
import type { SortDirection, SessionSortField } from "@/models/session";
import { useNavigate } from "react-router-dom";
import { getApiErrorMessage } from "@/lib/api/errors";

const PAGE_SIZE = 20;

export function SessionsPage() {
  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const [page, setPage] = useState(1);
  const [searchQuery, setSearchQuery] = useState("");
  const [sortField, setSortField] = useState<SessionSortField>("date");
  const [sortDirection, setSortDirection] = useState<SortDirection>("desc");
  const [selectedModelIds, setSelectedModelIds] = useState<string[]>([]);
  const [selectedManeuverIds, setSelectedManeuverIds] = useState<string[]>([]);
  const [maneuverOptionQuery, setManeuverOptionQuery] = useState("");

  const createSessionMutation = useMutation({
    mutationFn: () => {
      const now = new Date();
      const localDate = new Date(now.getTime() - now.getTimezoneOffset() * 60000)
        .toISOString()
        .slice(0, 10);
      return sessionsApi.create({ date: localDate, modelId: null, note: null });
    },
    onSuccess: (created) => {
      queryClient.invalidateQueries({ queryKey: ["sessions"] });
      navigate(`/sessions/${created.id}`, {
        state: {
          session: {
            id: created.id,
            userId: created.userId,
            date: created.date,
            modelId: created.modelId,
            modelName: null,
            modelType: null,
            modelPhotoAssetId: null,
            performedVariations: [],
          },
          note: created.note,
        },
      });
    },
  });

  const debouncedSearchQuery = useDebounce(searchQuery, 300);

  const modelsQuery = useQuery({
    queryKey: ["session-list-model-options"],
    queryFn: () => modelsApi.list({ page: 1, pageSize: 100 }),
  });

  const maneuversQuery = useQuery({
    queryKey: ["session-list-maneuver-options"],
    queryFn: () =>
      maneuversApi.list({
        page: 1,
        pageSize: 100,
        sort: { field: "name", direction: "asc" },
      }),
  });

  const sessionsQuery = useQuery({
    queryKey: [
      "sessions",
      {
        page,
        searchQuery: debouncedSearchQuery,
        sortField,
        sortDirection,
        modelIds: selectedModelIds,
        maneuverIds: selectedManeuverIds,
      },
    ],
    queryFn: () =>
      sessionsApi.list({
        page,
        pageSize: PAGE_SIZE,
        filter: {
          searchQuery: debouncedSearchQuery || undefined,
          modelIds: selectedModelIds.length > 0 ? selectedModelIds : undefined,
          maneuverIds: selectedManeuverIds.length > 0 ? selectedManeuverIds : undefined,
        },
        sort: {
          field: sortField,
          direction: sortDirection,
        },
      }),
  });

  const filteredManeuverOptions = useMemo(() => {
    const items = maneuversQuery.data?.items ?? [];
    const q = maneuverOptionQuery.trim().toLowerCase();
    if (!q) return items;
    return items.filter((m) => m.name.toLowerCase().includes(q));
  }, [maneuversQuery.data?.items, maneuverOptionQuery]);

  const sessions = sessionsQuery.data?.items ?? [];
  const totalPages = sessionsQuery.data?.totalPages ?? 1;
  const totalItems = sessionsQuery.data?.total ?? 0;
  const hasAnyFilter =
    !!debouncedSearchQuery || selectedModelIds.length > 0 || selectedManeuverIds.length > 0;

  const toggleModel = (id: string) => {
    setPage(1);
    setSelectedModelIds((prev) =>
      prev.includes(id) ? prev.filter((v) => v !== id) : [...prev, id],
    );
  };

  const toggleManeuver = (id: string) => {
    setPage(1);
    setSelectedManeuverIds((prev) =>
      prev.includes(id) ? prev.filter((v) => v !== id) : [...prev, id],
    );
  };

  const clearAll = () => {
    setPage(1);
    setSearchQuery("");
    setSortField("date");
    setSortDirection("desc");
    setSelectedModelIds([]);
    setSelectedManeuverIds([]);
    setManeuverOptionQuery("");
  };

  return (
    <div className="p-4 md:p-8 flex flex-col gap-6 h-full w-full">
      <div>
        <div className="flex items-center justify-between gap-3">
          <h1 className="text-3xl font-bold tracking-tight mb-2">Flight Sessions</h1>
          <Button
            onClick={() => createSessionMutation.mutate()}
            disabled={createSessionMutation.isPending}
          >
            {createSessionMutation.isPending ? (
              <>
                <Loader2 className="size-4 animate-spin" />
                Creating...
              </>
            ) : (
              "New Session"
            )}
          </Button>
        </div>
        <p className="text-muted-foreground text-sm max-w-3xl">
          Review your flying history, track model usage, and quickly spot what maneuvers and
          variations you trained most recently.
        </p>
        {createSessionMutation.isError && (
          <p className="text-xs text-destructive mt-2">
            {getApiErrorMessage(createSessionMutation.error) ?? "Failed to create session"}
          </p>
        )}
      </div>

      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-sm">Session Filters & Sorting</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-4">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
            <div className="relative md:col-span-2">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-muted-foreground" />
              <Input
                value={searchQuery}
                onChange={(e) => {
                  setPage(1);
                  setSearchQuery(e.target.value);
                }}
                placeholder="Search by model name, maneuver name, or variation name"
                className="pl-9"
              />
            </div>

            <div className="grid grid-cols-2 gap-2">
              <Select.Root
                value={sortField}
                onValueChange={(v) => {
                  setPage(1);
                  setSortField(v as SessionSortField);
                }}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="date">Sort: Date</SelectItem>
                </SelectContent>
              </Select.Root>

              <Select.Root
                value={sortDirection}
                onValueChange={(v) => {
                  setPage(1);
                  setSortDirection(v as SortDirection);
                }}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="desc">Newest first</SelectItem>
                  <SelectItem value="asc">Oldest first</SelectItem>
                </SelectContent>
              </Select.Root>
            </div>
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
            <div className="flex flex-col gap-2">
              <p className="text-xs text-muted-foreground uppercase tracking-wide">
                Filter by model
              </p>
              {modelsQuery.isLoading ? (
                <div className="h-14 flex items-center text-xs text-muted-foreground">
                  <Loader2 className="size-3.5 mr-2 animate-spin" />
                  Loading models...
                </div>
              ) : (
                <div className="flex flex-wrap gap-2">
                  {(modelsQuery.data?.items ?? []).map((model) => (
                    <Button
                      key={model.id}
                      type="button"
                      size="sm"
                      variant={selectedModelIds.includes(model.id) ? "default" : "outline"}
                      onClick={() => toggleModel(model.id)}
                      className="h-7 text-xs"
                    >
                      {model.name}
                    </Button>
                  ))}
                  {(modelsQuery.data?.items ?? []).length === 0 && (
                    <p className="text-xs text-muted-foreground">No models available.</p>
                  )}
                </div>
              )}
            </div>

            <div className="flex flex-col gap-2">
              <p className="text-xs text-muted-foreground uppercase tracking-wide">
                Filter by maneuver
              </p>
              <Input
                value={maneuverOptionQuery}
                onChange={(e) => setManeuverOptionQuery(e.target.value)}
                placeholder="Find maneuver options..."
              />
              <div className="flex flex-wrap gap-2 max-h-28 overflow-auto pr-1">
                {filteredManeuverOptions.map((maneuver) => (
                  <Button
                    key={maneuver.id}
                    type="button"
                    size="sm"
                    variant={selectedManeuverIds.includes(maneuver.id) ? "default" : "outline"}
                    onClick={() => toggleManeuver(maneuver.id)}
                    className="h-7 text-xs"
                  >
                    {maneuver.name}
                  </Button>
                ))}
                {filteredManeuverOptions.length === 0 && (
                  <p className="text-xs text-muted-foreground">No matching maneuvers.</p>
                )}
              </div>
            </div>
          </div>

          <div className="flex items-center justify-between">
            <div className="flex flex-wrap gap-1.5">
              {selectedModelIds.length > 0 && (
                <Badge variant="secondary">{selectedModelIds.length} model filters</Badge>
              )}
              {selectedManeuverIds.length > 0 && (
                <Badge variant="secondary">{selectedManeuverIds.length} maneuver filters</Badge>
              )}
              {debouncedSearchQuery && (
                <Badge variant="secondary">search: {debouncedSearchQuery}</Badge>
              )}
            </div>
            <Button variant="ghost" size="sm" onClick={clearAll} disabled={!hasAnyFilter}>
              Clear filters
            </Button>
          </div>
        </CardContent>
      </Card>

      {sessionsQuery.isLoading ? (
        <div className="flex-1 flex items-center justify-center p-12">
          <Loader2 className="animate-spin text-muted-foreground w-8 h-8" />
        </div>
      ) : sessionsQuery.isError ? (
        <div className="p-6 bg-destructive/10 text-destructive rounded-none border border-destructive/20 text-center">
          {sessionsQuery.error instanceof Error
            ? sessionsQuery.error.message
            : "Failed to load sessions"}
        </div>
      ) : sessions.length === 0 ? (
        <div className="flex-1 flex flex-col items-center justify-center p-12 text-center">
          <p className="text-muted-foreground mb-2">
            {hasAnyFilter ? "No sessions match your current filters." : "No sessions logged yet."}
          </p>
          <p className="text-xs text-muted-foreground/70">
            Start logging your flights and this page will become your pilot logbook timeline.
          </p>
        </div>
      ) : (
        <>
          <div className="flex items-center justify-between text-sm text-muted-foreground">
            <span>
              Showing {(page - 1) * PAGE_SIZE + 1} - {Math.min(page * PAGE_SIZE, totalItems)} of{" "}
              {totalItems} sessions
            </span>
            {sessionsQuery.isFetching && !sessionsQuery.isLoading && (
              <span className="flex items-center gap-1">
                <Loader2 className="h-3 w-3 animate-spin" />
                Updating...
              </span>
            )}
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
            {sessions.map((session) => (
              <SessionCard key={session.id} session={session} />
            ))}
          </div>

          {totalPages > 1 && (
            <div className="flex items-center justify-center gap-2 mt-2">
              <Button
                variant="outline"
                size="icon-sm"
                onClick={() => setPage((p) => p - 1)}
                disabled={page <= 1}
              >
                <ChevronLeft className="h-4 w-4" />
                <span className="sr-only">Previous page</span>
              </Button>

              <span className="text-sm text-muted-foreground px-4">
                Page {page} of {totalPages}
              </span>

              <Button
                variant="outline"
                size="icon-sm"
                onClick={() => setPage((p) => p + 1)}
                disabled={page >= totalPages}
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
