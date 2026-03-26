import { type ManeuverDto, type VehicleType, type DifficultyLevel } from "@/domain/maneuver";
import type { ManeuverDto as ApiManeuverDto } from "@/lib/api/models/maneuvers";

export function mapManeuverDto(apiData: ApiManeuverDto): ManeuverDto {
    return {
        id: apiData.id,
        name: apiData.name,
        vehicleType: apiData.vehicle_type as VehicleType,
        difficulty: `Level${apiData.difficulty}` as DifficultyLevel,
        tags: apiData.tags.map(t => ({ id: t.id, name: t.name })),
        description: apiData.description,
        videoUrl: apiData.video_path || undefined,
    }
}
