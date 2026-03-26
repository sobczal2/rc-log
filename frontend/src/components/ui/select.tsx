import { Select } from "@base-ui/react/select";
import { ChevronDown } from "lucide-react"
import { cn } from "@/lib/utils"

function SelectContent({
  className,
  children,
  ...props
}: Select.Popup.Props) {
  return (
    <Select.Portal>
      <Select.Backdrop className="fixed inset-0 z-40 bg-black/10 backdrop-blur-sm" />
      <Select.Positioner sideOffset={4} className="z-50">
        <Select.Popup
          className={cn(
            "min-w-[8rem] overflow-hidden rounded-none border bg-popover p-1 text-popover-foreground shadow-md",
            className
          )}
          {...props}
        >
          {children}
        </Select.Popup>
      </Select.Positioner>
    </Select.Portal>
  )
}

function SelectItem({
  className,
  children,
  ...props
}: Select.Item.Props) {
  return (
    <Select.Item
      className={cn(
        "relative flex w-full cursor-default select-none items-center rounded-none py-1 pl-2 pr-8 text-xs outline-none focus:bg-accent focus:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
        className
      )}
      {...props}
    >
      {children}
      <Select.ItemIndicator className="absolute right-2 flex size-3.5 items-center justify-center">
        <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path d="M11.4669 3.72684C11.7558 3.91574 11.8369 4.30308 11.648 4.59198L7.39799 11.092C7.29783 11.2452 7.13556 11.3467 6.95402 11.3699C6.77247 11.3931 6.58989 11.3355 6.45446 11.2124L3.70446 8.71241C3.44905 8.48022 3.43023 8.08494 3.66242 7.82953C3.89461 7.57412 4.28989 7.55529 4.5453 7.78749L6.75292 9.79441L10.6018 3.90792C10.7907 3.61902 11.178 3.53795 11.4669 3.72684Z" fill="currentColor" stroke="currentColor" strokeWidth="0" fillRule="evenodd" clipRule="evenodd"></path>
        </svg>
      </Select.ItemIndicator>
    </Select.Item>
  )
}

function SelectValue({
  placeholder = "Select...",
  ...props
}: Select.Value.Props) {
  return (
    <Select.Value placeholder={placeholder} {...props} />
  )
}

function SelectTrigger({
  className,
  children,
  ...props
}: Select.Trigger.Props) {
  return (
    <Select.Trigger
      className={cn(
        "inline-flex h-8 w-full items-center justify-between gap-2 rounded-none border border-input bg-transparent px-3 py-1 text-xs transition-colors placeholder:text-muted-foreground focus:border-ring focus:ring-1 focus:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-1 aria-invalid:ring-destructive/20 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40 [&>span]:line-clamp-1",
        className
      )}
      {...props}
    >
      {children}
      <Select.Icon>
        <ChevronDown className="size-3.5 opacity-50" />
      </Select.Icon>
    </Select.Trigger>
  )
}

export {
  Select,
  SelectContent,
  SelectItem,
  SelectValue,
  SelectTrigger,
}
