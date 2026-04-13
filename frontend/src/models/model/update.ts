import type { VehicleType } from "@/models/shared";

export interface UpdateModelRequest {
  name: string;
  vehicleType: VehicleType;
}

export interface UpdateModelDto {
  id: string;
  name: string;
  vehicleType: VehicleType;
  photoAssetName: string | null;
}
