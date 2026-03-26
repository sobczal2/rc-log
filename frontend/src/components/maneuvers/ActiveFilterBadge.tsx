import { X } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";

interface ActiveFilterBadgeProps {
  label: string;
  onRemove: () => void;
}

export function ActiveFilterBadge({ label, onRemove }: ActiveFilterBadgeProps) {
  return (
    <Badge variant="secondary" className="gap-1 pr-1">
      <span>{label}</span>
      <Button
        variant="ghost"
        size="icon-xs"
        className="ml-0.5 h-4 w-4"
        onClick={(e) => {
          e.preventDefault();
          e.stopPropagation();
          onRemove();
        }}
      >
        <X className="h-3 w-3" />
        <span className="sr-only">Remove filter</span>
      </Button>
    </Badge>
  );
}
