import { useCallback, useMemo } from "react";
import { useSearchParams } from "react-router-dom";
import { ALL_MODEL_TYPES } from "@/models/model/type";
import type { Type } from "@/models/model/type";
import { ALL_DIFFICULTY_LEVELS } from "@/models/shared/difficulty";
import type { DifficultyLevel } from "@/models/shared/difficulty";
import { ALL_MANEUVER_SORT_FIELDS, ALL_SORT_DIRECTIONS } from "@/models/maneuver/list";
import type { ManeuverSortField, SortDirection } from "@/models/maneuver/list";

export interface ManeuverFilters {
  searchQuery: string;
  model_type: Type | null;
  difficulty: DifficultyLevel | null;
  sortField: ManeuverSortField;
  sortDirection: SortDirection;
  page: number;
}

export interface ManeuverFiltersActions {
  setSearchQuery: (query: string) => void;
  setType: (type: Type | null) => void;
  setDifficulty: (difficulty: DifficultyLevel | null) => void;
  setSortField: (field: ManeuverSortField) => void;
  setSortDirection: (direction: SortDirection) => void;
  setPage: (page: number) => void;
  clearAll: () => void;
  removeFilter: (filterType: "searchQuery" | "model_type" | "difficulty") => void;
}

export function useManeuverFilters(): [ManeuverFilters, ManeuverFiltersActions] {
  const [searchParams, setSearchParams] = useSearchParams();

  const filters = useMemo<ManeuverFilters>(() => {
    const page = searchParams.get("page");
    const parsedPage = page ? parseInt(page, 10) : 1;

    const type = searchParams.get("type");
    const parsedType = type && ALL_MODEL_TYPES.includes(type as Type) ? (type as Type) : null;

    const difficulty = searchParams.get("difficulty");
    const parsedDifficulty =
      difficulty && ALL_DIFFICULTY_LEVELS.includes(difficulty as DifficultyLevel)
        ? (difficulty as DifficultyLevel)
        : null;

    const sortField = searchParams.get("sortField");
    const parsedSortField =
      sortField && ALL_MANEUVER_SORT_FIELDS.includes(sortField as ManeuverSortField)
        ? (sortField as ManeuverSortField)
        : "name";

    const sortDirection = searchParams.get("sortDirection");
    const parsedSortDirection =
      sortDirection && ALL_SORT_DIRECTIONS.includes(sortDirection as SortDirection)
        ? (sortDirection as SortDirection)
        : "asc";

    return {
      searchQuery: searchParams.get("searchQuery") || "",
      model_type: parsedType,
      difficulty: parsedDifficulty,
      sortField: parsedSortField,
      sortDirection: parsedSortDirection,
      page: isNaN(parsedPage) || parsedPage < 1 ? 1 : parsedPage,
    };
  }, [searchParams]);

  const updateParams = useCallback(
    (updates: Partial<ManeuverFilters>, resetPage = false) => {
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
    },
    [setSearchParams],
  );

  const actions = useMemo<ManeuverFiltersActions>(
    () => ({
      setSearchQuery: (query) => updateParams({ searchQuery: query }, true),
      setType: (type) => updateParams({ model_type: type }, true),
      setDifficulty: (difficulty) => updateParams({ difficulty }, true),
      setSortField: (field) => updateParams({ sortField: field }, true),
      setSortDirection: (direction) => updateParams({ sortDirection: direction }),
      setPage: (page) => updateParams({ page }),
      clearAll: () => {
        setSearchParams(
          new URLSearchParams({
            sortField: "name",
            sortDirection: "asc",
            page: "1",
          }),
        );
      },
      removeFilter: (filterType) => {
        setSearchParams((prev: URLSearchParams) => {
          const newParams = new URLSearchParams(prev);
          newParams.delete(filterType);
          newParams.set("page", "1");
          return newParams;
        });
      },
    }),
    [updateParams, setSearchParams],
  );

  return [filters, actions];
}
