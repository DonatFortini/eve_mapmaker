"use client";
import { useState } from "react";
import "./App.css";
import HomeScreen from "@/components/home-screen";
import MainScreen from "@/components/main-screen";
import LoaderScreen from "@/components/map-loading-screen";
import NewProjectScreen from "@/components/new-project-screen";

type ScreenType = "home" | "newProject" | "main" | "loader";

const screenComponents: Record<ScreenType, React.ComponentType<any>> = {
  home: HomeScreen,
  newProject: NewProjectScreen,
  main: MainScreen,
  loader: LoaderScreen,
};

const App = () => {
  const [currentScreen, setCurrentScreen] = useState<ScreenType>("home");
  const [department, setDepartment] = useState("");
  const [projectName, setProjectName] = useState("");

  const handleLoading = (dept: string, project: string) => {
    setDepartment(dept);
    setProjectName(project);
    setCurrentScreen("loader");
  };

  const ScreenComponent = screenComponents[currentScreen];

  return (
    <div className="App">
      {currentScreen === "loader" ? (
        <LoaderScreen department={department} projectName={projectName} />
      ) : (
        <ScreenComponent
          onNewProject={() => setCurrentScreen("newProject")}
          onMainScreen={() => setCurrentScreen("main")}
          onGoBack={() => setCurrentScreen("home")}
          onLoading={handleLoading}
        />
      )}
    </div>
  );
};

export default App;
