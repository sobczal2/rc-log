import type { VehicleType } from "./vehicle-type";

export type DifficultyLevel =
  | "level1"
  | "level2"
  | "level3"
  | "level4"
  | "level5"
  | "level6"
  | "level7";

export function getDifficultyLevelName(
  vehicleType: VehicleType,
  difficulty: DifficultyLevel,
): string {
  if (vehicleType === "Drone") {
    switch (difficulty) {
      case "level1":
        return "Beginner";
      case "level2":
        return "Basic Freestyle";
      case "level3":
        return "Intermediate Freestyle";
      case "level4":
        return "Advanced Freestyle";
      case "level5":
        return "Technical Freestyle";
      case "level6":
        return "Pro Freestyle";
      case "level7":
        return "Extreme Freestyle";
    }
  } else {
    switch (difficulty) {
      case "level1":
        return "Beginner";
      case "level2":
        return "Basic Sport";
      case "level3":
        return "Intermediate Sport";
      case "level4":
        return "Advanced Sport";
      case "level5":
        return "Basic 3D";
      case "level6":
        return "Intermediate 3D";
      case "level7":
        return "Advanced 3D";
    }
  }
}

export function getDifficultyLevelColor(difficulty: DifficultyLevel): string {
  switch (difficulty) {
    case "level1":
      return "bg-green-500/10 text-green-500 border-green-500/20";
    case "level2":
      return "bg-emerald-500/10 text-emerald-500 border-emerald-500/20";
    case "level3":
      return "bg-yellow-500/10 text-yellow-500 border-yellow-500/20";
    case "level4":
      return "bg-amber-500/10 text-amber-500 border-amber-500/20";
    case "level5":
      return "bg-orange-500/10 text-orange-500 border-orange-500/20";
    case "level6":
      return "bg-red-500/10 text-red-500 border-red-500/20";
    case "level7":
      return "bg-rose-600/10 text-rose-600 border-rose-600/20";
  }
}

export function getDifficultyLevelLabel(
  vehicleType: VehicleType,
  difficulty: DifficultyLevel,
): string {
  const level = difficulty.charAt(5);
  return `L${level}: ${getDifficultyLevelName(vehicleType, difficulty)}`;
}

export function getDifficultyLevelNumber(difficulty: DifficultyLevel): number {
  switch (difficulty) {
    case "level1":
      return 1;
    case "level2":
      return 2;
    case "level3":
      return 3;
    case "level4":
      return 4;
    case "level5":
      return 5;
    case "level6":
      return 6;
    case "level7":
      return 7;
  }
}
