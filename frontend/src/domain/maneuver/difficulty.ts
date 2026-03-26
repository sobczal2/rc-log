import type { VehicleType } from "./vehicle";

export type DifficultyLevel = 1 | 2 | 3 | 4 | 5 | 6 | 7;

export interface DifficultyInfo {
  levelNumber: number;
  name: string;
}

export function getDifficultyInfo(vehicleType: VehicleType, difficulty: DifficultyLevel): DifficultyInfo {
  const levelNumber = difficulty;
  let name = "";

  if (vehicleType === "Drone") {
    switch (difficulty) {
      case 1: name = "Beginner"; break;
      case 2: name = "Basic Freestyle"; break;
      case 3: name = "Intermediate Freestyle"; break;
      case 4: name = "Advanced Freestyle"; break;
      case 5: name = "Technical Freestyle"; break;
      case 6: name = "Pro Freestyle"; break;
      case 7: name = "Extreme Freestyle"; break;
    }
  } else {
    switch (difficulty) {
      case 1: name = "Beginner"; break;
      case 2: name = "Basic Sport"; break;
      case 3: name = "Intermediate Sport"; break;
      case 4: name = "Advanced Sport"; break;
      case 5: name = "Basic 3D"; break;
      case 6: name = "Intermediate 3D"; break;
      case 7: name = "Advanced 3D"; break;
    }
  }

  return { levelNumber, name };
}

export function getDifficultyColor(difficulty: DifficultyLevel): string {
  const colors: Record<DifficultyLevel, string> = {
    1: "bg-green-500/10 text-green-500 border-green-500/20",
    2: "bg-emerald-500/10 text-emerald-500 border-emerald-500/20",
    3: "bg-yellow-500/10 text-yellow-500 border-yellow-500/20",
    4: "bg-amber-500/10 text-amber-500 border-amber-500/20",
    5: "bg-orange-500/10 text-orange-500 border-orange-500/20",
    6: "bg-red-500/10 text-red-500 border-red-500/20",
    7: "bg-rose-600/10 text-rose-600 border-rose-600/20",
  };
  return colors[difficulty];
}

export function getDifficultyLabel(vehicleType: VehicleType, difficulty: DifficultyLevel): string {
  const info = getDifficultyInfo(vehicleType, difficulty);
  return `L${info.levelNumber}: ${info.name}`;
}