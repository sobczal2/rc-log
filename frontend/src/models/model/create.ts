import type { VehicleType } from "@/models/shared";

export interface CreateModelRequest {
  name: string;
  vehicleType: VehicleType;
}

export interface CreateModelDto {
  id: string;
  name: string;
  vehicleType: VehicleType;
  photoAssetName: string | null;
}
