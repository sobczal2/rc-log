import { useState } from "react";
import { Search, SlidersHorizontal, X } from "lucide-react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import type { VehicleType, DifficultyLevel } from "@/domain/maneuver";
import { useDebounce } from "@/hooks/useDebounce";
import type { ManeuverFilters, ManeuverFiltersActions } from "@/hooks/useManeuverFilters";
import { ActiveFilterBadge } from "./ActiveFilterBadge";

interface ManeuverFiltersProps {
  filters: ManeuverFilters;
  actions: ManeuverFiltersActions;
  isOpen: boolean;
  onToggle: () => void;
}

const VEHICLE_OPTIONS = [
  { value: "Helicopter", label: "Helicopter" },
  { value: "Plane", label: "Plane" },
  { value: "Drone", label: "Drone" },
];

const DIFFICULTY_OPTIONS = [
  { value: "1", label: "Level 1 - Beginner" },
  { value: "2", label: "Level 2" },
  { value: "3", label: "Level 3" },
  { value: "4", label: "Level 4" },
  { value: "5", label: "Level 5" },
  { value: "6", label: "Level 6" },
  { value: "7", label: "Level 7 - Expert" },
];

const SORT_FIELD_OPTIONS = [
  { value: "name", label: "Name" },
  { value: "difficulty", label: "Difficulty" },
];

const SORT_DIRECTION_OPTIONS = [
  { value: "asc", label: "Ascending" },
  { value: "desc", label: "Descending" },
];

export function ManeuverFilters({
  filters,
  actions,
  isOpen,
  onToggle,
}: ManeuverFiltersProps) {
  const [searchInput, setSearchInput] = useState("");
  useDebounce(searchInput, 300);

  const hasActiveFilters =
    filters.searchQuery ||
    filters.vehicleType ||
    filters.difficulty;

  const difficultyValue = filters.difficulty
    ? filters.difficulty.toString()
    : "all";

  const activeFilterCount = [filters.searchQuery, filters.vehicleType, filters.difficulty].filter(Boolean).length;

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center gap-2">
        <div className="relative flex-1">
          <Search className="absolute left-2.5 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
          <Input
            placeholder="Search maneuvers..."
            className="pl-8"
            value={searchInput}
            onChange={(e) => setSearchInput(e.target.value)}
            onBlur={() => {
              if (searchInput !== filters.searchQuery) {
                actions.setSearchQuery(searchInput);
              }
            }}
            onKeyDown={(e) => {
              if (e.key === "Enter" && searchInput !== filters.searchQuery) {
                actions.setSearchQuery(searchInput);
              }
              if (e.key === "Escape") {
                setSearchInput(filters.searchQuery);
              }
            }}
          />
        </div>

        <Button
          variant="outline"
          size="sm"
          onClick={onToggle}
          className="gap-2"
        >
          <SlidersHorizontal className="h-3.5 w-3.5" />
          <span className="hidden sm:inline">Filters</span>
          {hasActiveFilters && (
            <span className="flex h-4 w-4 items-center justify-center rounded-full bg-primary text-[10px] text-primary-foreground">
              {activeFilterCount}
            </span>
          )}
        </Button>
      </div>

      {hasActiveFilters && (
        <div className="flex flex-wrap items-center gap-2">
          <span className="text-xs text-muted-foreground">Active:</span>
          {filters.searchQuery && (
            <ActiveFilterBadge
              label={`"${filters.searchQuery}"`}
              onRemove={() => {
                setSearchInput("");
                actions.removeFilter("searchQuery");
              }}
            />
          )}
          {filters.vehicleType && (
            <ActiveFilterBadge
              label={filters.vehicleType}
              onRemove={() => actions.removeFilter("vehicleType")}
            />
          )}
          {filters.difficulty && (
            <ActiveFilterBadge
              label={`Level ${filters.difficulty}`}
              onRemove={() => actions.removeFilter("difficulty")}
            />
          )}
          <Button
            variant="ghost"
            size="sm"
            className="h-6 gap-1 px-2 text-xs text-muted-foreground hover:text-foreground"
            onClick={() => {
              setSearchInput("");
              actions.clearAll();
            }}
          >
            <X className="h-3 w-3" />
            Clear all
          </Button>
        </div>
      )}

      {isOpen && (
        <div className="flex flex-wrap items-center gap-3 rounded-none border bg-card p-4">
          <div className="flex flex-col gap-1.5">
            <label className="text-xs text-muted-foreground">Vehicle</label>
            <Select.Root
              value={filters.vehicleType || "all"}
              onValueChange={(value) =>
                actions.setVehicleType(!value || value === "all" ? null : value as VehicleType)
              }
            >
              <SelectTrigger className="w-[140px]">
                <SelectValue placeholder="All vehicles" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All vehicles</SelectItem>
                {VEHICLE_OPTIONS.map((opt) => (
                  <SelectItem key={opt.value} value={opt.value}>
                    {opt.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select.Root>
          </div>

          <div className="flex flex-col gap-1.5">
            <label className="text-xs text-muted-foreground">Difficulty</label>
            <Select.Root
              value={difficultyValue}
              onValueChange={(value) =>
                actions.setDifficulty(!value || value === "all" ? null : parseInt(value, 10) as DifficultyLevel)
              }
            >
              <SelectTrigger className="w-[160px]">
                <SelectValue placeholder="All difficulties" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All difficulties</SelectItem>
                {DIFFICULTY_OPTIONS.map((opt) => (
                  <SelectItem key={opt.value} value={opt.value}>
                    {opt.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select.Root>
          </div>

          <div className="flex flex-col gap-1.5">
            <label className="text-xs text-muted-foreground">Sort by</label>
            <div className="flex items-center gap-2">
              <Select.Root
                value={filters.sortField}
                onValueChange={(value) => {
                  if (value) actions.setSortField(value as "name" | "difficulty");
                }}
              >
                <SelectTrigger className="w-[120px]">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {SORT_FIELD_OPTIONS.map((opt) => (
                    <SelectItem key={opt.value} value={opt.value}>
                      {opt.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select.Root>

              <Select.Root
                value={filters.sortDirection}
                onValueChange={(value) => {
                  if (value) actions.setSortDirection(value as "asc" | "desc");
                }}
              >
                <SelectTrigger className="w-[110px]">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {SORT_DIRECTION_OPTIONS.map((opt) => (
                    <SelectItem key={opt.value} value={opt.value}>
                      {opt.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select.Root>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}