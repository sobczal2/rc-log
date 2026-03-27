import { useCallback, useMemo } from "react";
import { useSearchParams } from "react-router-dom";
import type { VehicleType, DifficultyLevel } from "@/models/shared";

const VALID_VEHICLE_TYPES = ["Helicopter", "Plane", "Drone"] as const;
const VALID_DIFFICULTIES = ["level1", "level2", "level3", "level4", "level5", "level6", "level7"] as const;
const VALID_SORT_FIELDS = ["name", "difficulty"] as const;
const VALID_SORT_DIRECTIONS = ["asc", "desc"] as const;

export interface ManeuverFilters {
  searchQuery: string;
  vehicleType: VehicleType | null;
  difficulty: DifficultyLevel | null;
  sortField: "name" | "difficulty";
  sortDirection: "asc" | "desc";
  page: number;
}

export interface ManeuverFiltersActions {
  setSearchQuery: (query: string) => void;
  setVehicleType: (vehicleType: VehicleType | null) => void;
  setDifficulty: (difficulty: DifficultyLevel | null) => void;
  setSortField: (field: "name" | "difficulty") => void;
  setSortDirection: (direction: "asc" | "desc") => void;
  setPage: (page: number) => void;
  clearAll: () => void;
  removeFilter: (filterType: "searchQuery" | "vehicleType" | "difficulty") => void;
}

export function useManeuverFilters(): [ManeuverFilters, ManeuverFiltersActions] {
  const [searchParams, setSearchParams] = useSearchParams();

  const filters = useMemo<ManeuverFilters>(() => {
    const page = searchParams.get("page");
    const parsedPage = page ? parseInt(page, 10) : 1;
    
    const vehicleType = searchParams.get("vehicleType");
    const parsedVehicleType = vehicleType && VALID_VEHICLE_TYPES.includes(vehicleType as VehicleType)
      ? (vehicleType as VehicleType)
      : null;

    const difficulty = searchParams.get("difficulty");
    const parsedDifficulty = difficulty && VALID_DIFFICULTIES.includes(difficulty as DifficultyLevel)
      ? (difficulty as DifficultyLevel)
      : null;

    const sortField = searchParams.get("sortField");
    const parsedSortField = sortField && VALID_SORT_FIELDS.includes(sortField as "name" | "difficulty")
      ? (sortField as "name" | "difficulty")
      : "name";

    const sortDirection = searchParams.get("sortDirection");
    const parsedSortDirection = sortDirection && VALID_SORT_DIRECTIONS.includes(sortDirection as "asc" | "desc")
      ? (sortDirection as "asc" | "desc")
      : "asc";

    return {
      searchQuery: searchParams.get("searchQuery") || "",
      vehicleType: parsedVehicleType,
      difficulty: parsedDifficulty,
      sortField: parsedSortField,
      sortDirection: parsedSortDirection,
      page: isNaN(parsedPage) || parsedPage < 1 ? 1 : parsedPage,
    };
  }, [searchParams]);

  const updateParams = useCallback((updates: Partial<ManeuverFilters>, resetPage = false) => {
    setSearchParams((prev: URLSearchParams) => {
      const newParams = new URLSearchParams(prev);
      
      Object.entries(updates).forEach(([key, value]) => {
        if (value === null || value === undefined || value === "") {
          newParams.delete(key);
        } else if (key === "page" && resetPage) {
          newParams.set("page", "1");
        } else {
          newParams.set(key, String(value));
        }
      });

      if (resetPage && !("page" in updates)) {
        newParams.set("page", "1");
      }

      return newParams;
    });
  }, [setSearchParams]);

  const actions = useMemo<ManeuverFiltersActions>(() => ({
    setSearchQuery: (query) => updateParams({ searchQuery: query }, true),
    setVehicleType: (vehicleType) => updateParams({ vehicleType }, true),
    setDifficulty: (difficulty) => updateParams({ difficulty }, true),
    setSortField: (field) => updateParams({ sortField: field }, true),
    setSortDirection: (direction) => updateParams({ sortDirection: direction }),
    setPage: (page) => updateParams({ page }),
    clearAll: () => {
      setSearchParams(new URLSearchParams({
        sortField: "name",
        sortDirection: "asc",
        page: "1",
      }));
    },
    removeFilter: (filterType) => {
      setSearchParams((prev: URLSearchParams) => {
        const newParams = new URLSearchParams(prev);
        newParams.delete(filterType);
        if (filterType !== "difficulty") {
          newParams.set("page", "1");
        }
        return newParams;
      });
    },
  }), [updateParams, setSearchParams]);

  return [filters, actions];
}