import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar"
import { AppSidebar } from "./AppSidebar"
import { Outlet } from "react-router-dom"

export function AppLayout() {
  return (
    <SidebarProvider>
      <AppSidebar />
      <main className="flex-1 flex flex-col min-h-screen overflow-hidden bg-background">
        <header className="flex h-16 items-center shrink-0 border-b border-border/50 px-4 md:px-6">
          <SidebarTrigger className="-ml-2 md:-ml-4 mr-2" />
          <div className="flex flex-1 items-center justify-between">
            {/* Additional header content can go here */}
          </div>
        </header>
        <div className="flex-1 overflow-auto">
          <Outlet />
        </div>
      </main>
    </SidebarProvider>
  )
}
