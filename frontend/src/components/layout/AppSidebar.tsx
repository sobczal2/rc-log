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
} from "@/components/ui/sidebar";
import { List, LogIn, LogOut, Plane, UserPlus } from "lucide-react";
import { Link, useLocation, useNavigate } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { useAuth } from "@/hooks/useAuth";

export function AppSidebar() {
  const location = useLocation();
  const navigate = useNavigate();
  const { isAuthenticated, user, signOut } = useAuth();

  const handleSignOut = () => {
    signOut();
    navigate("/");
  };

  return (
    <Sidebar className="border-r border-border/50">
      <SidebarHeader className="flex flex-row items-center h-16 px-4 gap-2">
        <Plane size={28} className="text-primary" />
        <span className="text-xl font-bold tracking-widest uppercase">rc-log</span>
      </SidebarHeader>

      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel className="text-xs uppercase tracking-wider text-muted-foreground/70">
            Knowledge Base
          </SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              <SidebarMenuItem>
                <SidebarMenuButton
                  render={<Link to="/maneuvers" />}
                  isActive={location.pathname === "/maneuvers"}
                  tooltip="Maneuvers"
                >
                  <List size={20} />
                  <span>Maneuvers</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>

      <SidebarFooter className="p-4 border-t border-border/50">
        {isAuthenticated ? (
          <div className="flex flex-col gap-2">
            <p className="text-xs text-muted-foreground truncate px-1">
              Signed in as <span className="font-medium text-foreground">{user?.username}</span>
            </p>
            <Button
              variant="outline"
              className="w-full justify-start gap-2"
              size="sm"
              onClick={handleSignOut}
            >
              <LogOut size={18} />
              <span>Sign Out</span>
            </Button>
          </div>
        ) : (
          <div className="flex flex-col gap-2">
            <Button
              variant="outline"
              className="w-full justify-start gap-2"
              size="sm"
              onClick={() => navigate("/sign-in")}
            >
              <LogIn size={18} />
              <span>Sign In</span>
            </Button>
            <Button
              variant="default"
              className="w-full justify-start gap-2"
              size="sm"
              onClick={() => navigate("/sign-up")}
            >
              <UserPlus size={18} />
              <span>Register</span>
            </Button>
          </div>
        )}
      </SidebarFooter>
    </Sidebar>
  );
}
