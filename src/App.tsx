import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  
  return (
    <div className="App">
      <header className="App-header">
        <button onClick={() => invoke("greet").then(console.log)}>New Map</button>
        <button onClick={() => invoke("greet").then(console.log)}>Open Map Folder</button>
      </header>
    </div>
  );
}

export default App;
