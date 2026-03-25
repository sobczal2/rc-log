import { useEffect, useState } from "react";
import { Button } from "./components/ui/button";
import { maneuversApi } from "./lib/api/maneuvers";

function App() {
  const [count, setCount] = useState(0);

  useEffect(() => {
    // Proof of integration
    maneuversApi
      .list({ page: 1, pageSize: 20 })
      .then((res) => console.log("API response:", res))
      .catch((err) => console.error("API error:", err));
  }, []);

  return <Button onClick={() => setCount((c) => c + 1)}>Test {count}</Button>;
}

export default App;
