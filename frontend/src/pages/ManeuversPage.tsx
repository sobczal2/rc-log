import { ManeuverCard } from "@/components/maneuvers/ManeuverCard";
import type { ManeuverDto } from "@/domain/maneuver";

const placeholderManeuvers: ManeuverDto[] = [
  {
    id: "1",
    name: "Knife Edge Spin",
    vehicleType: "Plane",
    difficulty: "Level5",
    tags: [{ id: "t1", name: "3D" }, { id: "t2", name: "Aerobatic" }],
    videoUrl: "https://www.w3schools.com/html/mov_bbb.mp4"
  },
  {
    id: "2",
    name: "Piroflip",
    vehicleType: "Helicopter",
    difficulty: "Level6",
    tags: [{ id: "t3", name: "Advanced" }, { id: "t4", name: "Pitch Control" }],
    videoUrl: ""
  },
  {
    id: "3",
    name: "Hover",
    vehicleType: "Plane",
    difficulty: "Level3",
    tags: [{ id: "t1", name: "3D" }, { id: "t5", name: "Basics" }],
    videoUrl: "https://www.w3schools.com/html/mov_bbb.mp4"
  },
  {
    id: "4",
    name: "Power Loop",
    vehicleType: "Drone",
    difficulty: "Level4",
    tags: [{ id: "t6", name: "Freestyle" }, { id: "t7", name: "Momentum" }],
    videoUrl: ""
  },
  {
    id: "5",
    name: "Blender",
    vehicleType: "Plane",
    difficulty: "Level7",
    tags: [{ id: "t1", name: "3D" }, { id: "t2", name: "Aerobatic" }, { id: "t8", name: "Snap" }],
    videoUrl: "https://www.w3schools.com/html/mov_bbb.mp4"
  }
];

export function ManeuversPage() {
  return (
    <div className="p-4 md:p-8 flex flex-col gap-6 h-full w-full">
      <div className="flex items-end justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight mb-2">Maneuvers Catalog</h1>
          <p className="text-muted-foreground max-w-2xl text-sm">
            Browse and learn different maneuvers for your RC vehicles.
          </p>
        </div>
      </div>
      
      <div className="grid grid-cols-1 xs:grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-4">
        {placeholderManeuvers.map(m => (
          <ManeuverCard key={m.id} maneuver={m} />
        ))}
      </div>
    </div>
  );
}
