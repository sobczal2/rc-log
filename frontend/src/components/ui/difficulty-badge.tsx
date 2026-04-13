import { Badge } from "@/components/ui/badge";
import { cn } from "@/lib/utils";
import type { DifficultyLevel } from "@/models/shared/difficulty";
import type { VehicleType } from "@/models/shared/vehicle-type";

// ── Helpers ──────────────────────────────────────────────────────────────────

function difficultyNumber(difficulty: DifficultyLevel): number {
  return parseInt(difficulty.charAt(5), 10);
}

function difficultyName(vehicleType: VehicleType, difficulty: DifficultyLevel): string {
  if (vehicleType === "Drone") {
    switch (difficulty) {
      case "level1": return "Beginner";
      case "level2": return "Basic Freestyle";
      case "level3": return "Intermediate Freestyle";
      case "level4": return "Advanced Freestyle";
      case "level5": return "Technical Freestyle";
      case "level6": return "Pro Freestyle";
      case "level7": return "Extreme Freestyle";
    }
  } else {
    switch (difficulty) {
      case "level1": return "Beginner";
      case "level2": return "Basic Sport";
      case "level3": return "Intermediate Sport";
      case "level4": return "Advanced Sport";
      case "level5": return "Basic 3D";
      case "level6": return "Intermediate 3D";
      case "level7": return "Advanced 3D";
    }
  }
}

function difficultyColor(difficulty: DifficultyLevel): string {
  switch (difficulty) {
    case "level1": return "bg-green-500/10 text-green-500 border-green-500/20";
    case "level2": return "bg-emerald-500/10 text-emerald-500 border-emerald-500/20";
    case "level3": return "bg-yellow-500/10 text-yellow-500 border-yellow-500/20";
    case "level4": return "bg-amber-500/10 text-amber-500 border-amber-500/20";
    case "level5": return "bg-orange-500/10 text-orange-500 border-orange-500/20";
    case "level6": return "bg-red-500/10 text-red-500 border-red-500/20";
    case "level7": return "bg-rose-600/10 text-rose-600 border-rose-600/20";
  }
}

// ── Badge components ──────────────────────────────────────────────────────────

/** Short single-level badge. Renders: L1 */
export function DifficultyBadgeShort({
  difficulty,
  className,
}: {
  difficulty: DifficultyLevel;
  className?: string;
}) {
  return (
    <Badge
      variant="outline"
      className={cn("rounded-sm font-mono font-bold", difficultyColor(difficulty), className)}
    >
      L{difficultyNumber(difficulty)}
    </Badge>
  );
}

/** Long single-level badge. Renders: L1: Beginner */
export function DifficultyBadgeLong({
  vehicleType,
  difficulty,
  className,
}: {
  vehicleType: VehicleType;
  difficulty: DifficultyLevel;
  className?: string;
}) {
  const n = difficultyNumber(difficulty);
  return (
    <Badge
      variant="outline"
      className={cn("rounded-sm font-mono font-bold", difficultyColor(difficulty), className)}
    >
      L{n}: {difficultyName(vehicleType, difficulty)}
    </Badge>
  );
}

/** Short range badge. Renders: L1–L3 (or L1 when min === max) */
export function DifficultyRangeBadgeShort({
  minDifficulty,
  maxDifficulty,
  className,
}: {
  minDifficulty: DifficultyLevel;
  maxDifficulty: DifficultyLevel;
  className?: string;
}) {
  const minN = difficultyNumber(minDifficulty);
  const maxN = difficultyNumber(maxDifficulty);
  const label = minN === maxN ? `L${minN}` : `L${minN}–L${maxN}`;
  return (
    <Badge
      variant="outline"
      className={cn("rounded-sm font-mono font-bold", difficultyColor(maxDifficulty), className)}
    >
      {label}
    </Badge>
  );
}

/** Long range badge. Renders: L1: Beginner – L3: Intermediate Sport (or L1: Beginner when min === max) */
export function DifficultyRangeBadgeLong({
  vehicleType,
  minDifficulty,
  maxDifficulty,
  className,
}: {
  vehicleType: VehicleType;
  minDifficulty: DifficultyLevel;
  maxDifficulty: DifficultyLevel;
  className?: string;
}) {
  const minN = difficultyNumber(minDifficulty);
  const maxN = difficultyNumber(maxDifficulty);
  const label =
    minN === maxN
      ? `L${minN}: ${difficultyName(vehicleType, minDifficulty)}`
      : `L${minN}: ${difficultyName(vehicleType, minDifficulty)} – L${maxN}: ${difficultyName(vehicleType, maxDifficulty)}`;
  return (
    <Badge
      variant="outline"
      className={cn("rounded-sm font-mono font-bold", difficultyColor(maxDifficulty), className)}
    >
      {label}
    </Badge>
  );
}
