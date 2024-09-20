import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import  HomeScreen from "./components/home-screen";

function App() {
  
  return (
    <div className="App">
      <HomeScreen />
    </div>
  );
}

export default App;
