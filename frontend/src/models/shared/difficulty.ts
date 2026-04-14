export type DifficultyLevel =
  | "level1"
  | "level2"
  | "level3"
  | "level4"
  | "level5"
  | "level6"
  | "level7";

export const ALL_DIFFICULTY_LEVELS: readonly DifficultyLevel[] = [
  "level1",
  "level2",
  "level3",
  "level4",
  "level5",
  "level6",
  "level7",
];

export function getDifficultyLabel(level: DifficultyLevel): string {
  switch (level) {
    case "level1":
      return "Level 1 - Beginner";
    case "level2":
      return "Level 2";
    case "level3":
      return "Level 3";
    case "level4":
      return "Level 4";
    case "level5":
      return "Level 5";
    case "level6":
      return "Level 6";
    case "level7":
      return "Level 7 - Expert";
  }
}
