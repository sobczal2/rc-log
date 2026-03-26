import type { VehicleType } from "./vehicle";

export type DifficultyLevel =
  | "Level1"
  | "Level2"
  | "Level3"
  | "Level4"
  | "Level5"
  | "Level6"
  | "Level7";

export function getDifficultyLevelName(
  vehicleType: VehicleType,
  difficulty: DifficultyLevel,
): string {
  if (vehicleType === "Drone") {
    switch (difficulty) {
      case "Level1":
        return "Beginner";
      case "Level2":
        return "Basic Freestyle";
      case "Level3":
        return "Intermediate Freestyle";
      case "Level4":
        return "Advanced Freestyle";
      case "Level5":
        return "Technical Freestyle";
      case "Level6":
        return "Pro Freestyle";
      case "Level7":
        return "Extreme Freestyle";
    }
  } else {
    switch (difficulty) {
      case "Level1":
        return "Beginner";
      case "Level2":
        return "Basic Sport";
      case "Level3":
        return "Intermediate Sport";
      case "Level4":
        return "Advanced Sport";
      case "Level5":
        return "Basic 3D";
      case "Level6":
        return "Intermediate 3D";
      case "Level7":
        return "Advanced 3D";
    }
  }
}

export function getDifficultyLevelColor(difficulty: DifficultyLevel): string {
  switch (difficulty) {
    case "Level1":
      return "bg-green-500/10 text-green-500 border-green-500/20";
    case "Level2":
      return "bg-emerald-500/10 text-emerald-500 border-emerald-500/20";
    case "Level3":
      return "bg-yellow-500/10 text-yellow-500 border-yellow-500/20";
    case "Level4":
      return "bg-amber-500/10 text-amber-500 border-amber-500/20";
    case "Level5":
      return "bg-orange-500/10 text-orange-500 border-orange-500/20";
    case "Level6":
      return "bg-red-500/10 text-red-500 border-red-500/20";
    case "Level7":
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
    case "Level1":
      return 1;
    case "Level2":
      return 2;
    case "Level3":
      return 3;
    case "Level4":
      return 4;
    case "Level5":
      return 5;
    case "Level6":
      return 6;
    case "Level7":
      return 7;
  }
}
