import "@/App.css";
import { WindowProvider } from "@/contexts/WindowContext";
import { Desktop } from "@/components/os/Desktop";

function App() {
  return (
    <WindowProvider>
      <Desktop />
    </WindowProvider>
  );
}

export default App;
