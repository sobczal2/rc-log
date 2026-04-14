import { useEffect, useMemo, useState } from "react";
import { Link, useLocation, useNavigate, useParams } from "react-router-dom";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { ArrowLeft, Loader2, Plus, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { sessionsApi } from "@/lib/api/sessions";
import { modelsApi } from "@/lib/api/models";
import { maneuversApi } from "@/lib/api/maneuvers";
import type {
  AddPerformedVariationComfortDto,
  AddPerformedVariationQualityDto,
  AddPerformedVariationRepeatabilityDto,
  ListSessionDto,
  UpdatePerformedVariationComfortDto,
  UpdatePerformedVariationQualityDto,
  UpdatePerformedVariationRepeatabilityDto,
} from "@/models/session";
import { useDebounce } from "@/hooks/useDebounce";
import { getApiErrorMessage } from "@/lib/api/errors";

interface SessionLocationState {
  session?: ListSessionDto;
  note?: string | null;
}

interface EditablePerformedVariation {
  performedVariationId: string;
  variationId: string;
  maneuverName: string | null;
  variationName: string | null;
  quality: RatingChoice;
  comfort: RatingChoice;
  repeatability: RatingChoice;
  note?: string | null;
}

type RatingChoice = "one" | "two" | "three" | "four" | "five";

const RATING_LEVELS: RatingChoice[] = ["one", "two", "three", "four", "five"];

const RATING_LABEL: Record<RatingChoice, string> = {
  one: "1",
  two: "2",
  three: "3",
  four: "4",
  five: "5",
};

export function SessionDetailsPage() {
  const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();
  const location = useLocation();
  const queryClient = useQueryClient();

  const routeState = (location.state as SessionLocationState | null) ?? null;
  const session = routeState?.session;
  const sessionId = id ?? session?.id ?? null;
  const fallbackDate = useMemo(() => {
    const now = new Date();
    return new Date(now.getTime() - now.getTimezoneOffset() * 60000)
      .toISOString()
      .slice(0, 10);
  }, []);

  const [date, setDate] = useState(session?.date ?? fallbackDate);
  const [modelId, setModelId] = useState<string | null>(session?.modelId ?? null);
  const [note, setNote] = useState(routeState?.note ?? "");
  const [lastSavedNote, setLastSavedNote] = useState(routeState?.note ?? "");
  const [performedVariations, setPerformedVariations] = useState<EditablePerformedVariation[]>(
    session?.performedVariations.map((v) => ({ ...v, note: null })) ?? [],
  );

  const [selectedManeuverId, setSelectedManeuverId] = useState<string>("");
  const [selectedVariationId, setSelectedVariationId] = useState<string>("");
  const [addQuality, setAddQuality] = useState<RatingChoice>("three");
  const [addComfort, setAddComfort] = useState<RatingChoice>("three");
  const [addRepeatability, setAddRepeatability] = useState<RatingChoice>("three");
  const [addNote, setAddNote] = useState("");

  const debouncedNote = useDebounce(note, 500);

  const modelsQuery = useQuery({
    queryKey: ["session-editor-model-options"],
    queryFn: () => modelsApi.list({ page: 1, pageSize: 100 }),
    enabled: !!sessionId,
  });

  const maneuversQuery = useQuery({
    queryKey: ["session-editor-maneuver-options"],
    queryFn: () =>
      maneuversApi.list({
        page: 1,
        pageSize: 100,
        sort: { field: "name", direction: "asc" },
      }),
    enabled: !!sessionId,
  });

  const selectedManeuverQuery = useQuery({
    queryKey: ["session-editor-maneuver", selectedManeuverId],
    queryFn: () => maneuversApi.getById({ id: selectedManeuverId }),
    enabled: !!selectedManeuverId,
  });

  const variationOptions = useMemo(() => {
    const maneuver = selectedManeuverQuery.data;
    if (!maneuver) return [];

    return [maneuver.defaultVariation, ...maneuver.variations].map((variation) => ({
      id: variation.id,
      name: variation.name,
    }));
  }, [selectedManeuverQuery.data]);

  const maneuverNameById = useMemo(() => {
    const lookup = new Map<string, string>();
    for (const maneuver of maneuversQuery.data?.items ?? []) {
      lookup.set(maneuver.id, maneuver.name);
    }
    return lookup;
  }, [maneuversQuery.data?.items]);

  const variationNameById = useMemo(() => {
    const lookup = new Map<string, string>();
    for (const variation of variationOptions) {
      lookup.set(variation.id, variation.name);
    }
    return lookup;
  }, [variationOptions]);

  const modelNameById = useMemo(() => {
    const lookup = new Map<string, string>();
    for (const modelOption of modelsQuery.data?.items ?? []) {
      lookup.set(modelOption.id, modelOption.name);
    }
    return lookup;
  }, [modelsQuery.data?.items]);

  const renderRatingValue = (value: unknown) => {
    if (typeof value !== "string") return null;
    if (!Object.prototype.hasOwnProperty.call(RATING_LABEL, value)) return value;
    return RATING_LABEL[value as RatingChoice];
  };

  const updateSessionMutation = useMutation({
    mutationFn: (payload: { date: string; modelId: string | null; note: string | null }) =>
      sessionsApi.update(sessionId!, payload),
    onSuccess: (updated) => {
      queryClient.invalidateQueries({ queryKey: ["sessions"] });
      setLastSavedNote(updated.note ?? "");
    },
  });

  const upsertPerformedMutation = useMutation({
    mutationFn: (payload: Omit<EditablePerformedVariation, "performedVariationId">) =>
      sessionsApi.addPerformedVariation({
        sessionId: sessionId!,
        payload: {
          variationId: payload.variationId,
          quality: payload.quality as AddPerformedVariationQualityDto,
          comfort: payload.comfort as AddPerformedVariationComfortDto,
          repeatability: payload.repeatability as AddPerformedVariationRepeatabilityDto,
          note: payload.note ?? null,
        },
      }),
    onSuccess: (data, payload) => {
      setPerformedVariations((prev) => [
        {
          ...payload,
          performedVariationId: data.performedVariationId,
        },
        ...prev,
      ]);
      queryClient.invalidateQueries({ queryKey: ["sessions"] });
      setAddNote("");
    },
  });

  const removePerformedMutation = useMutation({
    mutationFn: (performedVariationId: string) =>
      sessionsApi.removePerformedVariation(sessionId!, performedVariationId),
    onSuccess: (_data, performedVariationId) => {
      setPerformedVariations((prev) =>
        prev.filter((v) => v.performedVariationId !== performedVariationId),
      );
      queryClient.invalidateQueries({ queryKey: ["sessions"] });
    },
  });

  const updatePerformedMutation = useMutation({
    mutationFn: (payload: EditablePerformedVariation) =>
      sessionsApi.updatePerformedVariation({
        sessionId: sessionId!,
        performedVariationId: payload.performedVariationId,
        payload: {
          quality: payload.quality as UpdatePerformedVariationQualityDto,
          comfort: payload.comfort as UpdatePerformedVariationComfortDto,
          repeatability: payload.repeatability as UpdatePerformedVariationRepeatabilityDto,
          note: payload.note ?? null,
        },
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["sessions"] });
    },
  });

  const deleteSessionMutation = useMutation({
    mutationFn: () => sessionsApi.delete(sessionId!),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["sessions"] });
      navigate("/sessions");
    },
  });

  useEffect(() => {
    if (!sessionId || !date) return;
    if (debouncedNote === lastSavedNote) return;

    updateSessionMutation.mutate({
      date,
      modelId,
      note: debouncedNote.trim() ? debouncedNote : null,
    });
  }, [debouncedNote, lastSavedNote, sessionId, date, modelId, updateSessionMutation]);

  if (!sessionId || !session) {
    return (
      <div className="p-4 md:p-8 flex flex-col gap-4 max-w-2xl">
        <Link to="/sessions">
          <Button variant="ghost" size="sm">
            <ArrowLeft data-icon="inline-start" />
            Back to Sessions
          </Button>
        </Link>
        <p className="text-sm text-muted-foreground">
          Session editor needs session state from navigation. Open a session from the list or create a
          new one from the Sessions page.
        </p>
      </div>
    );
  }

  const handleDateChange = (value: string) => {
    setDate(value);
    updateSessionMutation.mutate({
      date: value,
      modelId,
      note: note.trim() ? note : null,
    });
  };

  const handleModelChange = (value: string | null) => {
    const nextModelId = value === "none" || !value ? null : value;
    setModelId(nextModelId);
    updateSessionMutation.mutate({
      date,
      modelId: nextModelId,
      note: note.trim() ? note : null,
    });
  };

  const handleAddVariation = () => {
    if (!selectedVariationId) return;

    const selected = variationOptions.find((v) => v.id === selectedVariationId);
    const maneuverName = selectedManeuverQuery.data?.name ?? null;

    upsertPerformedMutation.mutate({
      variationId: selectedVariationId,
      maneuverName,
      variationName: selected?.name ?? null,
      quality: addQuality,
      comfort: addComfort,
      repeatability: addRepeatability,
      note: addNote.trim() ? addNote : null,
    });
  };

  const updateExistingVariation = (
    performedVariationId: string,
    patch: Partial<Pick<EditablePerformedVariation, "quality" | "comfort" | "repeatability" | "note">>,
  ) => {
    const current = performedVariations.find(
      (v) => v.performedVariationId === performedVariationId,
    );
    if (!current) return;

    const next: EditablePerformedVariation = { ...current, ...patch };

    setPerformedVariations((prev) =>
      prev.map((v) => (v.performedVariationId === performedVariationId ? next : v)),
    );
    updatePerformedMutation.mutate(next);
  };

  return (
    <div className="p-4 md:p-8 flex flex-col gap-6 w-full max-w-5xl mx-auto">
      <div className="flex items-center justify-between">
        <Link to="/sessions">
          <Button variant="ghost" size="sm">
            <ArrowLeft data-icon="inline-start" />
            Back to Sessions
          </Button>
        </Link>

        <div className="flex items-center gap-3">
          {(updateSessionMutation.isPending ||
            upsertPerformedMutation.isPending ||
            updatePerformedMutation.isPending) && (
            <span className="text-xs text-muted-foreground flex items-center gap-1">
              <Loader2 className="size-3 animate-spin" />
              Saving...
            </span>
          )}

          <Button
            variant="destructive"
            size="sm"
            disabled={deleteSessionMutation.isPending}
            onClick={() => {
              if (!sessionId) return;
              const confirmed = window.confirm(
                "Delete this session? This will also remove all logged performed variations.",
              );
              if (!confirmed) return;
              deleteSessionMutation.mutate();
            }}
          >
            <Trash2 data-icon="inline-start" size={14} />
            {deleteSessionMutation.isPending ? "Deleting..." : "Delete Session"}
          </Button>
        </div>
      </div>

      {deleteSessionMutation.isError && (
        <p className="text-xs text-destructive">
          {getApiErrorMessage(deleteSessionMutation.error) ?? "Failed to delete session"}
        </p>
      )}

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Session Setup</CardTitle>
        </CardHeader>
        <CardContent className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="flex flex-col gap-1">
            <p className="text-xs text-muted-foreground uppercase tracking-wide">Date</p>
            <Input type="date" value={date} onChange={(e) => handleDateChange(e.target.value)} />
          </div>

          <div className="flex flex-col gap-1">
            <p className="text-xs text-muted-foreground uppercase tracking-wide">Model</p>
            <Select.Root value={modelId ?? "none"} onValueChange={handleModelChange}>
              <SelectTrigger>
                <SelectValue placeholder="No model">
                  {(value) => {
                    if (value === "none") return "No model";
                    if (typeof value !== "string") return null;
                    return modelNameById.get(value) ?? value;
                  }}
                </SelectValue>
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="none">No model</SelectItem>
                {(modelsQuery.data?.items ?? []).map((modelOption) => (
                  <SelectItem key={modelOption.id} value={modelOption.id}>
                    {modelOption.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select.Root>
          </div>

          <div className="md:col-span-2 flex flex-col gap-1">
            <p className="text-xs text-muted-foreground uppercase tracking-wide">Session note</p>
            <Input
              value={note}
              onChange={(e) => setNote(e.target.value)}
              placeholder="Wind conditions, battery setup, gyro tuning notes..."
            />
            <p className="text-[11px] text-muted-foreground/70">Autosaves while typing.</p>
          </div>

          {updateSessionMutation.isError && (
            <p className="md:col-span-2 text-xs text-destructive">
              {getApiErrorMessage(updateSessionMutation.error) ?? "Failed to update session"}
            </p>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Add Maneuver</CardTitle>
        </CardHeader>
        <CardContent className="grid grid-cols-1 md:grid-cols-6 gap-3">
          <div className="md:col-span-2">
            <p className="text-xs text-muted-foreground uppercase tracking-wide mb-1">Maneuver</p>
            <Select.Root
              value={selectedManeuverId || undefined}
              onValueChange={(value) => {
                setSelectedManeuverId(value ?? "");
                setSelectedVariationId("");
              }}
            >
              <SelectTrigger>
                <SelectValue placeholder="Choose maneuver">
                  {(value) => {
                    if (typeof value !== "string") return null;
                    return maneuverNameById.get(value) ?? value;
                  }}
                </SelectValue>
              </SelectTrigger>
              <SelectContent>
                {(maneuversQuery.data?.items ?? []).map((maneuver) => (
                  <SelectItem key={maneuver.id} value={maneuver.id}>
                    {maneuver.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select.Root>
          </div>

          <div className="md:col-span-2">
            <p className="text-xs text-muted-foreground uppercase tracking-wide mb-1">Variation</p>
            <Select.Root
              value={selectedVariationId || undefined}
              onValueChange={(value) => setSelectedVariationId(value ?? "")}
            >
              <SelectTrigger>
                <SelectValue placeholder="Choose variation">
                  {(value) => {
                    if (typeof value !== "string") return null;
                    return variationNameById.get(value) ?? value;
                  }}
                </SelectValue>
              </SelectTrigger>
              <SelectContent>
                {variationOptions.map((variation) => (
                  <SelectItem key={variation.id} value={variation.id}>
                    {variation.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select.Root>
          </div>

          <div className="md:col-span-2">
            <p className="text-xs text-muted-foreground uppercase tracking-wide mb-1">Variation note</p>
            <Input
              value={addNote}
              onChange={(e) => setAddNote(e.target.value)}
              placeholder="How did this attempt feel?"
            />
          </div>

          <div>
            <p className="text-xs text-muted-foreground uppercase tracking-wide mb-1">Quality</p>
            <Select.Root value={addQuality} onValueChange={(v) => setAddQuality(v as RatingChoice)}>
              <SelectTrigger>
                <SelectValue>{renderRatingValue}</SelectValue>
              </SelectTrigger>
              <SelectContent>
                {RATING_LEVELS.map((lvl) => (
                  <SelectItem key={lvl} value={lvl}>
                    {RATING_LABEL[lvl]}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select.Root>
          </div>

          <div>
            <p className="text-xs text-muted-foreground uppercase tracking-wide mb-1">Comfort</p>
            <Select.Root value={addComfort} onValueChange={(v) => setAddComfort(v as RatingChoice)}>
              <SelectTrigger>
                <SelectValue>{renderRatingValue}</SelectValue>
              </SelectTrigger>
              <SelectContent>
                {RATING_LEVELS.map((lvl) => (
                  <SelectItem key={lvl} value={lvl}>
                    {RATING_LABEL[lvl]}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select.Root>
          </div>

          <div>
            <p className="text-xs text-muted-foreground uppercase tracking-wide mb-1">Repeatability</p>
            <Select.Root
              value={addRepeatability}
              onValueChange={(v) => setAddRepeatability(v as RatingChoice)}
            >
              <SelectTrigger>
                <SelectValue>{renderRatingValue}</SelectValue>
              </SelectTrigger>
              <SelectContent>
                {RATING_LEVELS.map((lvl) => (
                  <SelectItem key={lvl} value={lvl}>
                    {RATING_LABEL[lvl]}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select.Root>
          </div>

          <div className="md:col-span-3 flex items-end">
            <Button
              onClick={handleAddVariation}
              disabled={!selectedVariationId || upsertPerformedMutation.isPending}
              className="w-full md:w-auto"
            >
              <Plus data-icon="inline-start" size={14} />
              Add / Update Maneuver
            </Button>
          </div>

          {upsertPerformedMutation.isError && (
            <p className="md:col-span-6 text-xs text-destructive">
              {getApiErrorMessage(upsertPerformedMutation.error) ?? "Failed to add maneuver"}
            </p>
          )}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">Logged Maneuvers</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-3">
          {performedVariations.length === 0 ? (
            <p className="text-sm text-muted-foreground">No performed maneuvers yet.</p>
          ) : (
            performedVariations.map((item) => (
              <div key={item.performedVariationId} className="border border-border/60 p-3 flex flex-col gap-3">
                <div className="flex items-start justify-between gap-2">
                  <div>
                    <p className="text-sm font-semibold">{item.variationName ?? "Unknown variation"}</p>
                    <p className="text-xs text-muted-foreground">{item.maneuverName ?? "Unknown maneuver"}</p>
                  </div>
                  <Button
                    variant="ghost"
                    size="icon-sm"
                    onClick={() => removePerformedMutation.mutate(item.performedVariationId)}
                    disabled={removePerformedMutation.isPending}
                  >
                    <Trash2 className="size-3.5" />
                    <span className="sr-only">Remove variation</span>
                  </Button>
                </div>

                <div className="grid grid-cols-3 gap-2">
                  <div>
                    <p className="text-[11px] uppercase tracking-wide text-muted-foreground mb-1">Quality</p>
                    <Select.Root
                      value={item.quality}
                      onValueChange={(v) =>
                        updateExistingVariation(item.performedVariationId, {
                          quality: v as RatingChoice,
                        })
                      }
                    >
                      <SelectTrigger>
                        <SelectValue>{renderRatingValue}</SelectValue>
                      </SelectTrigger>
                      <SelectContent>
                        {RATING_LEVELS.map((lvl) => (
                          <SelectItem key={lvl} value={lvl}>
                            {RATING_LABEL[lvl]}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select.Root>
                  </div>

                  <div>
                    <p className="text-[11px] uppercase tracking-wide text-muted-foreground mb-1">Comfort</p>
                    <Select.Root
                      value={item.comfort}
                      onValueChange={(v) =>
                        updateExistingVariation(item.performedVariationId, {
                          comfort: v as RatingChoice,
                        })
                      }
                    >
                      <SelectTrigger>
                        <SelectValue>{renderRatingValue}</SelectValue>
                      </SelectTrigger>
                      <SelectContent>
                        {RATING_LEVELS.map((lvl) => (
                          <SelectItem key={lvl} value={lvl}>
                            {RATING_LABEL[lvl]}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select.Root>
                  </div>

                  <div>
                    <p className="text-[11px] uppercase tracking-wide text-muted-foreground mb-1">
                      Repeatability
                    </p>
                    <Select.Root
                      value={item.repeatability}
                      onValueChange={(v) =>
                        updateExistingVariation(item.performedVariationId, {
                          repeatability: v as RatingChoice,
                        })
                      }
                    >
                      <SelectTrigger>
                        <SelectValue>{renderRatingValue}</SelectValue>
                      </SelectTrigger>
                      <SelectContent>
                        {RATING_LEVELS.map((lvl) => (
                          <SelectItem key={lvl} value={lvl}>
                            {RATING_LABEL[lvl]}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select.Root>
                  </div>
                </div>

              </div>
            ))
          )}

          {removePerformedMutation.isError && (
            <p className="text-xs text-destructive">
              {getApiErrorMessage(removePerformedMutation.error) ?? "Failed to remove maneuver"}
            </p>
          )}
          {updatePerformedMutation.isError && (
            <p className="text-xs text-destructive">
              {getApiErrorMessage(updatePerformedMutation.error) ?? "Failed to update maneuver"}
            </p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
