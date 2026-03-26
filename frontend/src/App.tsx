import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { AppLayout } from "./components/layout/AppLayout";
import { HomePage } from "./pages/HomePage";
import { ManeuversPage } from "./pages/ManeuversPage";
import { ManeuverDetailsPage } from "./pages/ManeuverDetailsPage";

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
    ],
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;
