import { useRef, useState } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Camera, Plus, X } from "lucide-react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { modelsApi } from "@/lib/api/models";
import type { VehicleType } from "@/models/shared";
import { getVehicleIcon, getVehicleLabel } from "@/models/shared";

const VEHICLE_TYPES: VehicleType[] = ["Helicopter", "Plane", "Drone"];

export function CreateModelDialog() {
  const [open, setOpen] = useState(false);
  const [name, setName] = useState("");
  const [vehicleType, setVehicleType] = useState<VehicleType>("Plane");
  const [photoFile, setPhotoFile] = useState<File | null>(null);
  const [photoPreview, setPhotoPreview] = useState<string | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const queryClient = useQueryClient();

  const reset = () => {
    setName("");
    setVehicleType("Plane");
    setPhotoFile(null);
    setPhotoPreview(null);
  };

  const handlePhotoChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0] ?? null;
    if (!file) return;
    setPhotoFile(file);
    const url = URL.createObjectURL(file);
    setPhotoPreview(url);
  };

  const clearPhoto = () => {
    setPhotoFile(null);
    if (photoPreview) URL.revokeObjectURL(photoPreview);
    setPhotoPreview(null);
    if (fileInputRef.current) fileInputRef.current.value = "";
  };

  const { mutate, isPending, error } = useMutation({
    mutationFn: async () => {
      const model = await modelsApi.create({ name: name.trim(), vehicleType });
      if (photoFile) {
        await modelsApi.updatePhoto(model.id, photoFile);
      }
      return model;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["models"] });
      setOpen(false);
      reset();
    },
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (name.trim()) mutate();
  };

  const handleOpenChange = (next: boolean) => {
    setOpen(next);
    if (!next) reset();
  };

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogTrigger render={<Button size="sm" />}>
        <Plus data-icon="inline-start" />
        Add Model
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add New Model</DialogTitle>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="flex flex-col gap-4 pt-2">
          {/* Photo picker */}
          <div className="flex flex-col gap-1.5">
            <label className="text-xs text-muted-foreground">Photo (optional)</label>
            <div className="relative w-full aspect-video bg-muted/30 border border-border/50 flex items-center justify-center overflow-hidden">
              {photoPreview ? (
                <>
                  <img src={photoPreview} alt="preview" className="w-full h-full object-cover" />
                  <button
                    type="button"
                    onClick={clearPhoto}
                    className="absolute top-1.5 right-1.5 bg-background/80 rounded-full p-0.5 hover:bg-background transition-colors"
                    aria-label="Remove photo"
                  >
                    <X size={14} />
                  </button>
                </>
              ) : (
                <button
                  type="button"
                  onClick={() => fileInputRef.current?.click()}
                  className="flex flex-col items-center gap-2 text-muted-foreground/40 hover:text-muted-foreground/60 transition-colors py-4"
                >
                  <div className="text-muted-foreground/20">{getVehicleIcon(vehicleType, 48)}</div>
                  <span className="flex items-center gap-1 text-xs">
                    <Camera size={12} />
                    Add photo
                  </span>
                </button>
              )}
            </div>
            <input
              ref={fileInputRef}
              type="file"
              accept="image/jpeg,image/png,image/webp"
              className="hidden"
              onChange={handlePhotoChange}
            />
          </div>

          <div className="flex flex-col gap-1.5">
            <label className="text-xs text-muted-foreground">Name</label>
            <Input
              placeholder="e.g. My T-Rex 550"
              value={name}
              onChange={(e) => setName(e.target.value)}
              autoFocus
            />
          </div>
          <div className="flex flex-col gap-1.5">
            <label className="text-xs text-muted-foreground">Vehicle Type</label>
            <Select.Root
              value={vehicleType}
              onValueChange={(value) => setVehicleType(value as VehicleType)}
            >
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {VEHICLE_TYPES.map((type) => (
                  <SelectItem key={type} value={type}>
                    {getVehicleLabel(type)}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select.Root>
          </div>
          {error && (
            <p className="text-xs text-destructive">
              {error instanceof Error ? error.message : "Failed to create model"}
            </p>
          )}
          <DialogFooter>
            <Button type="submit" size="sm" disabled={!name.trim() || isPending}>
              {isPending ? "Creating…" : "Create"}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
