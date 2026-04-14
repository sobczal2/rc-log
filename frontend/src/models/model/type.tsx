import { Plane, Helicopter, Drone } from "lucide-react";
import type { ReactNode } from "react";
import type { TypeDto } from "@/models/__generated/model/shared/type";

export type Type = TypeDto;

export const ALL_MODEL_TYPES: readonly Type[] = ["Helicopter", "Plane", "Drone"];

export function getModelTypeIcon(type: Type, size = 18): ReactNode {
  switch (type) {
    case "Plane":
      return <Plane size={size} />;
    case "Helicopter":
      return <Helicopter size={size} />;
    case "Drone":
      return <Drone size={size} />;
  }
}

export function getModelTypeLabel(type: Type): string {
  return type;
}
