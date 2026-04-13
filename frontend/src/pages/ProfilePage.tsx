import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { ChevronLeft, ChevronRight, Loader2, User } from "lucide-react";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { ModelCard } from "@/components/models/ModelCard";
import { CreateModelDialog } from "@/components/models/CreateModelDialog";
import { modelsApi } from "@/lib/api/models";
import { useAuth } from "@/hooks/useAuth";

const PAGE_SIZE = 20;

export function ProfilePage() {
  const { user } = useAuth();
  const [page, setPage] = useState(1);

  const { data, isLoading, isError, error, isFetching } = useQuery({
    queryKey: ["models", { page }],
    queryFn: () => modelsApi.list({ page, pageSize: PAGE_SIZE }),
  });

  const models = data?.items ?? [];
  const totalPages = data?.totalPages ?? 1;
  const totalItems = data?.total ?? 0;

  return (
    <div className="p-4 md:p-8 flex flex-col gap-6 h-full w-full max-w-5xl mx-auto">
      {/* Profile header */}
      <div className="flex items-center gap-3">
        <div className="size-14 rounded-full bg-muted flex items-center justify-center text-muted-foreground/60 flex-shrink-0">
          <User size={28} />
        </div>
        <div>
          <h1 className="text-2xl font-bold tracking-tight">{user?.username}</h1>
          <p className="text-xs text-muted-foreground">{user?.email}</p>
        </div>
      </div>

      {/* Tabs */}
      <Tabs defaultValue="models" className="flex-1">
        <TabsList>
          <TabsTrigger value="models">Models</TabsTrigger>
          <TabsTrigger value="statistics">Statistics</TabsTrigger>
        </TabsList>

        {/* ── Models tab ── */}
        <TabsContent value="models" className="flex flex-col gap-4 pt-4">
          <div className="flex items-center justify-between">
            <p className="text-sm text-muted-foreground">
              {totalItems > 0 && `${totalItems} model${totalItems !== 1 ? "s" : ""}`}
            </p>
            <CreateModelDialog />
          </div>

          {isLoading ? (
            <div className="flex items-center justify-center py-16">
              <Loader2 className="animate-spin text-muted-foreground size-8" />
            </div>
          ) : isError ? (
            <div className="p-6 bg-destructive/10 text-destructive rounded-none border border-destructive/20 text-center text-xs">
              {error instanceof Error ? error.message : "Failed to load models"}
            </div>
          ) : models.length === 0 ? (
            <div className="flex flex-col items-center justify-center py-16 text-center gap-3">
              <p className="text-muted-foreground text-sm">No models yet.</p>
              <p className="text-xs text-muted-foreground/60">
                Add your first RC model to get started.
              </p>
            </div>
          ) : (
            <>
              {isFetching && !isLoading && (
                <div className="flex items-center gap-1 text-xs text-muted-foreground">
                  <Loader2 className="size-3 animate-spin" />
                  Updating…
                </div>
              )}
              <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
                {models.map((m) => (
                  <ModelCard key={m.id} model={m} />
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
                    <ChevronLeft className="size-4" />
                    <span className="sr-only">Previous page</span>
                  </Button>
                  <span className="text-xs text-muted-foreground px-4">
                    Page {page} of {totalPages}
                  </span>
                  <Button
                    variant="outline"
                    size="icon-sm"
                    onClick={() => setPage((p) => p + 1)}
                    disabled={page >= totalPages}
                  >
                    <ChevronRight className="size-4" />
                    <span className="sr-only">Next page</span>
                  </Button>
                </div>
              )}
            </>
          )}
        </TabsContent>

        {/* ── Statistics tab ── */}
        <TabsContent value="statistics" className="pt-4">
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <Card>
              <CardHeader>
                <CardTitle className="text-sm font-medium text-muted-foreground">
                  Total Models
                </CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-3xl font-bold">{totalItems}</p>
              </CardContent>
            </Card>
            <Card>
              <CardHeader>
                <CardTitle className="text-sm font-medium text-muted-foreground">
                  Flight Sessions
                </CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-3xl font-bold text-muted-foreground/40">—</p>
                <p className="text-xs text-muted-foreground mt-1">Coming soon</p>
              </CardContent>
            </Card>
          </div>
        </TabsContent>
      </Tabs>
    </div>
  );
}
