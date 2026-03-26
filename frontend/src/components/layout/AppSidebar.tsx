import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar"
import { List, LogIn, UserPlus, Plane } from "lucide-react"
import { Link, useLocation } from "react-router-dom"
import { Button } from "@/components/ui/button"

export function AppSidebar() {
  const location = useLocation()
  
  return (
    <Sidebar className="border-r border-border/50">
      <SidebarHeader className="flex flex-row items-center h-16 px-4 gap-2">
        <Plane size={28} className="text-primary" />
        <span className="text-xl font-bold tracking-widest uppercase">rc-log</span>
      </SidebarHeader>
      
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel className="text-xs uppercase tracking-wider text-muted-foreground/70">Records</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton render={<Link to="/maneuvers" />} isActive={location.pathname === "/maneuvers"} tooltip="Maneuvers">
                  <List size={20} />
                  <span>Maneuvers</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>

      <SidebarFooter className="p-4 border-t border-border/50">
        <div className="flex flex-col gap-2">
          <Button variant="outline" className="w-full justify-start gap-2" size="sm">
            <LogIn size={18} />
            <span>Sign In</span>
          </Button>
          <Button variant="default" className="w-full justify-start gap-2" size="sm">
            <UserPlus size={18} />
            <span>Register</span>
          </Button>
        </div>
      </SidebarFooter>
    </Sidebar>
  )
}
