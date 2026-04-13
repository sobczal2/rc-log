import type { VehicleType } from "@/models/shared";

export interface GetByIdModelDto {
  id: string;
  name: string;
  vehicleType: VehicleType;
  photoAssetName: string | null;
}
