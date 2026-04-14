import { useRef, useState } from "react";
import { useParams, Link, useNavigate } from "react-router-dom";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { ArrowLeft, Camera, Loader2, Pencil, Trash2, X } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import { modelsApi } from "@/lib/api/models";
import { usePhotoPath } from "@/hooks/usePhotoPath";
import { getPhotoUrl } from "@/models/asset/photo";
import { getModelTypeIcon, getModelTypeLabel, ALL_MODEL_TYPES } from "@/models/model/type";
import type { Type } from "@/models/model/type";

export function ModelDetailsPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  // Edit state
  const [isEditing, setIsEditing] = useState(false);
  const [editName, setEditName] = useState("");
  const [editType, setEditType] = useState<Type>("Plane");

  // Delete confirmation dialog
  const [deleteOpen, setDeleteOpen] = useState(false);

  // Photo upload
  const photoInputRef = useRef<HTMLInputElement>(null);

  const {
    data: model,
    isLoading,
    isError,
    error,
  } = useQuery({
    queryKey: ["models", id],
    queryFn: () => modelsApi.getById(id!),
    enabled: !!id,
  });

  const { data: photoPathData } = usePhotoPath(model?.photoAssetId ?? null);
  const photoSrc = photoPathData
    ? getPhotoUrl(photoPathData.largePath ?? photoPathData.mediumPath ?? photoPathData.smallPath)
    : null;

  // Update name/type
  const updateMutation = useMutation({
    mutationFn: () => modelsApi.update(id!, { name: editName.trim(), type: editType }),
    onSuccess: (updated) => {
      queryClient.setQueryData(["models", id], updated);
      queryClient.invalidateQueries({ queryKey: ["models"] });
      setIsEditing(false);
    },
  });

  // Delete model
  const deleteMutation = useMutation({
    mutationFn: () => modelsApi.delete(id!),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["models"] });
      navigate("/profile");
    },
  });

  // Update photo
  const updatePhotoMutation = useMutation({
    mutationFn: (file: File) => modelsApi.updatePhoto(id!, file),
    onSuccess: (updated) => {
      const prevAssetId = model?.photoAssetId;
      queryClient.setQueryData(["models", id], updated);
      queryClient.invalidateQueries({ queryKey: ["models"] });
      if (prevAssetId) {
        queryClient.invalidateQueries({ queryKey: ["photo-path", prevAssetId] });
      }
    },
  });

  // Remove photo
  const removePhotoMutation = useMutation({
    mutationFn: () => modelsApi.removePhoto(id!),
    onSuccess: () => {
      const prevAssetId = model?.photoAssetId;
      queryClient.invalidateQueries({ queryKey: ["models", id] });
      queryClient.invalidateQueries({ queryKey: ["models"] });
      if (prevAssetId) {
        queryClient.invalidateQueries({ queryKey: ["photo-path", prevAssetId] });
      }
    },
  });

  const startEditing = () => {
    if (!model) return;
    setEditName(model.name);
    setEditType(model.type);
    setIsEditing(true);
  };

  const handlePhotoChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    updatePhotoMutation.mutate(file);
    if (photoInputRef.current) photoInputRef.current.value = "";
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full p-12">
        <Loader2 className="animate-spin text-muted-foreground size-8" />
      </div>
    );
  }

  if (isError || !model) {
    return (
      <div className="p-8 flex flex-col gap-4">
        <Link to="/profile">
          <Button variant="ghost" size="sm">
            <ArrowLeft data-icon="inline-start" />
            Back to Profile
          </Button>
        </Link>
        <p className="text-destructive text-sm">
          {error instanceof Error ? error.message : "Model not found."}
        </p>
      </div>
    );
  }

  const isPhotoBusy = updatePhotoMutation.isPending || removePhotoMutation.isPending;

  return (
    <div className="p-4 md:p-8 flex flex-col gap-6 max-w-2xl mx-auto w-full">
      <Link to="/profile">
        <Button variant="ghost" size="sm">
          <ArrowLeft data-icon="inline-start" />
          Back to Profile
        </Button>
      </Link>

      {/* Photo area */}
      <div className="group relative w-full aspect-video bg-muted/30 border border-border/50 flex items-center justify-center overflow-hidden">
        {isPhotoBusy && (
          <div className="absolute inset-0 flex items-center justify-center bg-background/50 z-10">
            <Loader2 className="animate-spin text-muted-foreground size-6" />
          </div>
        )}
        {photoSrc ? (
          <>
            <img src={photoSrc} alt={model.name} className="w-full h-full object-cover" />
            {/* Hover overlay for change / remove */}
            <div className="absolute inset-0 bg-black/50 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center gap-3">
              <Button
                size="sm"
                variant="secondary"
                onClick={() => photoInputRef.current?.click()}
                disabled={isPhotoBusy}
              >
                <Camera data-icon="inline-start" size={14} />
                Change
              </Button>
              <Button
                size="sm"
                variant="destructive"
                onClick={() => removePhotoMutation.mutate()}
                disabled={isPhotoBusy}
              >
                <X data-icon="inline-start" size={14} />
                Remove
              </Button>
            </div>
          </>
        ) : (
          <button
            type="button"
            onClick={() => photoInputRef.current?.click()}
            disabled={isPhotoBusy}
            className="flex flex-col items-center gap-2 text-muted-foreground/40 hover:text-muted-foreground/60 transition-colors py-4 w-full h-full justify-center"
          >
            <div className="text-muted-foreground/20">{getModelTypeIcon(model.type, 64)}</div>
            <span className="flex items-center gap-1 text-xs">
              <Camera size={12} />
              Upload Photo
            </span>
          </button>
        )}
        <input
          ref={photoInputRef}
          type="file"
          accept="image/jpeg,image/png,image/webp"
          className="hidden"
          onChange={handlePhotoChange}
        />
      </div>

      {updatePhotoMutation.isError && (
        <p className="text-xs text-destructive -mt-4">
          {updatePhotoMutation.error instanceof Error
            ? updatePhotoMutation.error.message
            : "Failed to upload photo"}
        </p>
      )}

      {/* Info / Edit area */}
      {isEditing ? (
        <form
          onSubmit={(e) => {
            e.preventDefault();
            if (editName.trim()) updateMutation.mutate();
          }}
          className="flex flex-col gap-3"
        >
          <Input value={editName} onChange={(e) => setEditName(e.target.value)} autoFocus />
          <Select.Root value={editType} onValueChange={(v) => setEditType(v as Type)}>
            <SelectTrigger>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {ALL_MODEL_TYPES.map((type) => (
                <SelectItem key={type} value={type}>
                  {getModelTypeLabel(type)}
                </SelectItem>
              ))}
            </SelectContent>
          </Select.Root>
          {updateMutation.isError && (
            <p className="text-xs text-destructive">
              {updateMutation.error instanceof Error
                ? updateMutation.error.message
                : "Failed to update model"}
            </p>
          )}
          <div className="flex gap-2">
            <Button type="submit" size="sm" disabled={!editName.trim() || updateMutation.isPending}>
              {updateMutation.isPending ? "Saving…" : "Save"}
            </Button>
            <Button
              type="button"
              size="sm"
              variant="ghost"
              onClick={() => setIsEditing(false)}
              disabled={updateMutation.isPending}
            >
              Cancel
            </Button>
          </div>
        </form>
      ) : (
        <div className="flex flex-col gap-2">
          <div className="flex items-start justify-between gap-2">
            <h1 className="text-2xl font-bold tracking-tight">{model.name}</h1>
            <Button variant="ghost" size="sm" onClick={startEditing} aria-label="Edit model">
              <Pencil size={14} />
            </Button>
          </div>
          <div className="flex items-center gap-2 text-sm text-muted-foreground">
            {getModelTypeIcon(model.type, 16)}
            <span>{getModelTypeLabel(model.type)}</span>
          </div>
        </div>
      )}

      {/* Delete zone */}
      <div className="mt-4 pt-4 border-t border-border/50">
        <Button
          variant="destructive"
          size="sm"
          onClick={() => setDeleteOpen(true)}
          disabled={deleteMutation.isPending}
        >
          <Trash2 data-icon="inline-start" size={14} />
          Delete Model
        </Button>
      </div>

      {/* Delete confirmation dialog */}
      <Dialog open={deleteOpen} onOpenChange={setDeleteOpen}>
        <DialogContent showCloseButton={false}>
          <DialogHeader>
            <DialogTitle>Delete "{model.name}"?</DialogTitle>
          </DialogHeader>
          <p className="text-sm text-muted-foreground">
            This will permanently delete the model and its photo. This action cannot be undone.
          </p>
          {deleteMutation.isError && (
            <p className="text-xs text-destructive">
              {deleteMutation.error instanceof Error
                ? deleteMutation.error.message
                : "Failed to delete model"}
            </p>
          )}
          <DialogFooter className="flex gap-2">
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setDeleteOpen(false)}
              disabled={deleteMutation.isPending}
            >
              Cancel
            </Button>
            <Button
              variant="destructive"
              size="sm"
              onClick={() => deleteMutation.mutate()}
              disabled={deleteMutation.isPending}
            >
              {deleteMutation.isPending ? "Deleting…" : "Delete"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}
