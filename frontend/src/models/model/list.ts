import type { VehicleType } from "@/models/shared";

export interface ListModelDto {
  id: string;
  name: string;
  vehicleType: VehicleType;
  photoAssetName: string | null;
}
