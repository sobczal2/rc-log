import { useRef, useState } from "react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import {
  ChevronLeft,
  ChevronRight,
  Loader2,
  Pencil,
  User,
  X,
  Check,
  Camera,
  Trash2,
} from "lucide-react";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { ModelCard } from "@/components/models/ModelCard";
import { CreateModelDialog } from "@/components/models/CreateModelDialog";
import { modelsApi } from "@/lib/api/models";
import { usersApi } from "@/lib/api/users";
import { useAuth } from "@/hooks/useAuth";
import { usePhotoPath } from "@/hooks/usePhotoPath";
import { getPhotoUrl } from "@/models/asset/photo";
import { getApiErrorMessage } from "@/lib/api/errors";

const PAGE_SIZE = 20;

export function ProfilePage() {
  const { user, updateUser } = useAuth();
  const queryClient = useQueryClient();
  const [page, setPage] = useState(1);

  // Username editing
  const [editingUsername, setEditingUsername] = useState(false);
  const [usernameInput, setUsernameInput] = useState("");
  const [usernameError, setUsernameError] = useState<string | null>(null);

  // Photo file input ref
  const photoInputRef = useRef<HTMLInputElement>(null);

  const { data, isLoading, isError, error, isFetching } = useQuery({
    queryKey: ["models", { page }],
    queryFn: () => modelsApi.list({ page, pageSize: PAGE_SIZE }),
  });

  const { data: photoPaths } = usePhotoPath(user?.photoAssetName ?? null);

  const photoUrl = photoPaths?.smallPath ? getPhotoUrl(photoPaths.smallPath) : null;

  const updateUsernameMutation = useMutation({
    mutationFn: (newUsername: string) => usersApi.update({ newUsername }),
    onSuccess: (updatedUser) => {
      updateUser(updatedUser);
      setEditingUsername(false);
      setUsernameError(null);
    },
    onError: (err) => {
      setUsernameError(getApiErrorMessage(err) ?? "Failed to update username");
    },
  });

  const updatePhotoMutation = useMutation({
    mutationFn: (file: File) => usersApi.updatePhoto(file),
    onSuccess: (updatedUser) => {
      updateUser(updatedUser);
      queryClient.invalidateQueries({ queryKey: ["photo-path", user?.photoAssetName] });
    },
  });

  const removePhotoMutation = useMutation({
    mutationFn: () => usersApi.removePhoto(),
    onSuccess: () => {
      if (user) updateUser({ ...user, photoAssetName: null });
    },
  });

  const handleStartEditUsername = () => {
    setUsernameInput(user?.username ?? "");
    setUsernameError(null);
    setEditingUsername(true);
  };

  const handleSaveUsername = () => {
    if (!usernameInput.trim()) return;
    updateUsernameMutation.mutate(usernameInput.trim());
  };

  const handlePhotoFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    updatePhotoMutation.mutate(file);
    e.target.value = "";
  };

  const models = data?.items ?? [];
  const totalPages = data?.totalPages ?? 1;
  const totalItems = data?.total ?? 0;

  return (
    <div className="p-4 md:p-8 flex flex-col gap-6 h-full w-full max-w-5xl mx-auto">
      {/* Profile header */}
      <div className="flex items-center gap-4">
        {/* Avatar with photo upload controls */}
        <div className="relative flex-shrink-0 group">
          <div className="size-16 rounded-full bg-muted flex items-center justify-center text-muted-foreground/60 overflow-hidden">
            {photoUrl ? (
              <img src={photoUrl} alt="Profile" className="size-full object-cover" />
            ) : updatePhotoMutation.isPending ? (
              <Loader2 className="animate-spin size-6" />
            ) : (
              <User size={28} />
            )}
          </div>
          {/* Upload overlay */}
          <button
            className="absolute inset-0 rounded-full bg-black/40 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center"
            onClick={() => photoInputRef.current?.click()}
            disabled={updatePhotoMutation.isPending || removePhotoMutation.isPending}
            title="Change photo"
          >
            <Camera size={18} className="text-white" />
          </button>
          {/* Remove photo button */}
          {user?.photoAssetName && (
            <button
              className="absolute -top-1 -right-1 size-5 rounded-full bg-destructive text-destructive-foreground flex items-center justify-center shadow opacity-0 group-hover:opacity-100 transition-opacity"
              onClick={() => removePhotoMutation.mutate()}
              disabled={removePhotoMutation.isPending}
              title="Remove photo"
            >
              <Trash2 size={10} />
            </button>
          )}
          <input
            ref={photoInputRef}
            type="file"
            accept="image/jpeg,image/png,image/webp"
            className="hidden"
            onChange={handlePhotoFileChange}
          />
        </div>

        {/* Username + email */}
        <div className="flex flex-col gap-1 min-w-0">
          {editingUsername ? (
            <div className="flex items-center gap-2">
              <Input
                className="h-8 text-lg font-bold w-48"
                value={usernameInput}
                onChange={(e) => setUsernameInput(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === "Enter") handleSaveUsername();
                  if (e.key === "Escape") setEditingUsername(false);
                }}
                autoFocus
              />
              <Button
                size="icon-sm"
                variant="ghost"
                onClick={handleSaveUsername}
                disabled={updateUsernameMutation.isPending}
              >
                {updateUsernameMutation.isPending ? (
                  <Loader2 className="size-3 animate-spin" />
                ) : (
                  <Check className="size-3" />
                )}
              </Button>
              <Button
                size="icon-sm"
                variant="ghost"
                onClick={() => setEditingUsername(false)}
                disabled={updateUsernameMutation.isPending}
              >
                <X className="size-3" />
              </Button>
            </div>
          ) : (
            <div className="flex items-center gap-2">
              <h1 className="text-2xl font-bold tracking-tight truncate">{user?.username}</h1>
              <Button
                size="icon-sm"
                variant="ghost"
                onClick={handleStartEditUsername}
                className="text-muted-foreground"
                title="Edit username"
              >
                <Pencil className="size-3" />
              </Button>
            </div>
          )}
          {usernameError && (
            <p className="text-xs text-destructive">{usernameError}</p>
          )}
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
