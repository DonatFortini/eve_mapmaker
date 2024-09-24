"use client"
import { useState } from 'react'
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import  HomeScreen from "./components/home-screen";
import  MainScreen from "./components/main-screen";
import  LoaderScreen from "./components/map-loading-screen";
import  NewProjectScreen from "./components/new-project-screen";

function App() {
  const [currentScreen, setCurrentScreen] = useState('home'); 

  const renderScreen = () => {
    switch (currentScreen) {
      case "home":
        return <HomeScreen onNewProject={() => setCurrentScreen("newProject")} onMainScreen={() => setCurrentScreen("main")} />;
      case "newProject":
        return <NewProjectScreen onGoBack={() => setCurrentScreen("home")} />;
      case "main":
        return <MainScreen />;
      default:
        return <HomeScreen onNewProject={() => setCurrentScreen("newProject")} onMainScreen={() => setCurrentScreen("main")} />;
    }
  };

  return (
    <div className="App">
      {renderScreen()}
    </div>
  );
}

export default App;
