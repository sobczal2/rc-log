import { Plane, Helicopter, Drone } from "lucide-react";
import type { ReactNode } from "react";

export type VehicleType = "Helicopter" | "Plane" | "Drone";

export function getVehicleIcon(vehicleType: VehicleType, size = 18): ReactNode {
  switch (vehicleType) {
    case "Plane":
      return <Plane size={size} />;
    case "Helicopter":
      return <Helicopter size={size} />;
    case "Drone":
      return <Drone size={size} />;
  }
}

export function getVehicleLabel(vehicleType: VehicleType): string {
  return vehicleType;
}
