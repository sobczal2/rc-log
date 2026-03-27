import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { AppLayout } from "./components/layout/AppLayout";
import { AuthProvider } from "./context/AuthContext";
import { HomePage } from "./pages/HomePage";
import { ManeuverDetailsPage } from "./pages/ManeuverDetailsPage";
import { ManeuversPage } from "./pages/ManeuversPage";
import { SignInPage } from "./pages/SignInPage";
import { SignUpPage } from "./pages/SignUpPage";

const router = createBrowserRouter([
  {
    path: "/",
    element: <AppLayout />,
    children: [
      {
        path: "/",
        element: <HomePage />,
      },
      {
        path: "/maneuvers",
        element: <ManeuversPage />,
      },
      {
        path: "/maneuvers/:id",
        element: <ManeuverDetailsPage />,
      },
      {
        path: "/sign-in",
        element: <SignInPage />,
      },
      {
        path: "/sign-up",
        element: <SignUpPage />,
      },
    ],
  },
]);

function App() {
  return (
    <AuthProvider>
      <RouterProvider router={router} />
    </AuthProvider>
  );
}

export default App;
