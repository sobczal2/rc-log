import type { VehicleType } from "./vehicle";

export type DifficultyLevel = "Level1" | "Level2" | "Level3" | "Level4" | "Level5" | "Level6" | "Level7";

export interface DifficultyInfo {
  levelNumber: number;
  name: string;
}

export function getDifficultyInfo(vehicleType: VehicleType, difficulty: DifficultyLevel): DifficultyInfo {
  const levelNumber = parseInt(difficulty.replace("Level", ""), 10);
  let name = "";

  if (vehicleType === "Drone") {
    switch (difficulty) {
      case "Level1": name = "Beginner"; break;
      case "Level2": name = "Basic Freestyle"; break;
      case "Level3": name = "Intermediate Freestyle"; break;
      case "Level4": name = "Advanced Freestyle"; break;
      case "Level5": name = "Technical Freestyle"; break;
      case "Level6": name = "Pro Freestyle"; break;
      case "Level7": name = "Extreme Freestyle"; break;
    }
  } else {
    switch (difficulty) {
      case "Level1": name = "Beginner"; break;
      case "Level2": name = "Basic Sport"; break;
      case "Level3": name = "Intermediate Sport"; break;
      case "Level4": name = "Advanced Sport"; break;
      case "Level5": name = "Basic 3D"; break;
      case "Level6": name = "Intermediate 3D"; break;
      case "Level7": name = "Advanced 3D"; break;
    }
  }

  return { levelNumber, name };
}
