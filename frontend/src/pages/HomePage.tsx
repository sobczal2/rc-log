export function HomePage() {
  return (
    <div className="flex flex-col items-center justify-center h-full p-8 text-center">
      <h1 className="text-4xl font-bold tracking-tight mb-4">Welcome to rc-log</h1>
      <p className="text-muted-foreground text-lg max-w-2xl">
        Your personal catalog for radio-controlled model maneuvers. Select a maneuver category
        from the sidebar to get started.
      </p>
    </div>
  );
}
