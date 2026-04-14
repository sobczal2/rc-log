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
import { ALL_MODEL_TYPES, getModelTypeLabel } from "@/models/model/type";
import type { Type } from "@/models/model/type";
import { ALL_DIFFICULTY_LEVELS, getDifficultyLabel } from "@/models/shared/difficulty";
import type { DifficultyLevel } from "@/models/shared/difficulty";
import {
  ALL_MANEUVER_SORT_FIELDS,
  ALL_SORT_DIRECTIONS,
  getManeuverSortFieldLabel,
  getSortDirectionLabel,
} from "@/models/maneuver/list";
import type { ManeuverSortField, SortDirection } from "@/models/maneuver/list";
import type { ManeuverFilters, ManeuverFiltersActions } from "@/hooks/useManeuverFilters";
import { ActiveFilterBadge } from "./ActiveFilterBadge";

interface ManeuverFiltersProps {
  filters: ManeuverFilters;
  actions: ManeuverFiltersActions;
  isOpen: boolean;
  onToggle: () => void;
}

export function ManeuverFilters({ filters, actions, isOpen, onToggle }: ManeuverFiltersProps) {
  const [searchInput, setSearchInput] = useState("");

  const hasActiveFilters = filters.searchQuery || filters.model_type || filters.difficulty;

  const difficultyValue = filters.difficulty ? filters.difficulty : "all";

  const activeFilterCount = [filters.searchQuery, filters.model_type, filters.difficulty].filter(
    Boolean,
  ).length;

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

        <Button variant="outline" size="sm" onClick={onToggle} className="gap-2">
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
          {filters.model_type && (
            <ActiveFilterBadge
              label={getModelTypeLabel(filters.model_type)}
              onRemove={() => actions.removeFilter("model_type")}
            />
          )}
          {filters.difficulty && (
            <ActiveFilterBadge
              label={getDifficultyLabel(filters.difficulty)}
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
            <label className="text-xs text-muted-foreground">Model</label>
            <Select.Root
              value={filters.model_type || "all"}
              onValueChange={(value) =>
                actions.setType(!value || value === "all" ? null : (value as Type))
              }
            >
              <SelectTrigger className="w-[140px]">
                <SelectValue placeholder="All models" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All models</SelectItem>
                {ALL_MODEL_TYPES.map((t) => (
                  <SelectItem key={t} value={t}>
                    {getModelTypeLabel(t)}
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
                actions.setDifficulty(!value || value === "all" ? null : (value as DifficultyLevel))
              }
            >
              <SelectTrigger className="w-[160px]">
                <SelectValue placeholder="All difficulties" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All difficulties</SelectItem>
                {ALL_DIFFICULTY_LEVELS.map((lvl) => (
                  <SelectItem key={lvl} value={lvl}>
                    {getDifficultyLabel(lvl)}
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
                  if (value) actions.setSortField(value as ManeuverSortField);
                }}
              >
                <SelectTrigger className="w-[120px]">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {ALL_MANEUVER_SORT_FIELDS.map((f) => (
                    <SelectItem key={f} value={f}>
                      {getManeuverSortFieldLabel(f)}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select.Root>

              <Select.Root
                value={filters.sortDirection}
                onValueChange={(value) => {
                  if (value) actions.setSortDirection(value as SortDirection);
                }}
              >
                <SelectTrigger className="w-[110px]">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {ALL_SORT_DIRECTIONS.map((d) => (
                    <SelectItem key={d} value={d}>
                      {getSortDirectionLabel(d)}
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
