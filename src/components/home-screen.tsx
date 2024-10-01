"use client";

import { Button } from "@/components/ui/button";
import MapPreview from "@/components/map-preview";
import { FolderPlus, Book, Settings, FolderInput } from "lucide-react";
import { open as openLink } from "@tauri-apps/api/shell";
interface HomeScreenProps {
  onNewProject: () => void;
  onMainScreen: () => void;
}

export default function HomeScreen({
  onNewProject,
  onMainScreen,
}: HomeScreenProps) {
  const recentMaps = [
    { id: 1, name: "City Center Map" },
    { id: 2, name: "National Park Trails" },
    { id: 3, name: "Historical District" },
    { id: 4, name: "Local Bike Paths" },
    { id: 5, name: "Tourist Attractions" },
    { id: 6, name: "Local Parks" },
  ];

  return (
    <div className="flex h-screen w-screen bg-[#2D2D30] text-[#CCCCCC]">
      <div className="bg-[#252526] p-4 flex flex-col w-80 transition-all duration-300 ease-in-out">
        <div className="mb-8">
          <h1 className="text-4xl font-bold text-white tracking-wider">
            <span className="text-[#FFD700]">Map</span>Maker
          </h1>
        </div>

        <div className="flex flex-col">
          <Button
            variant="ghost"
            size="lg"
            className="justify-start text-left mb-2 p-4 hover:bg-[#3E3E42] hover:text-white"
            onClick={onMainScreen}
          >
            <FolderInput className="mr-2 h-6 w-6" />
            Charger un projet
          </Button>
          <Button
            variant="ghost"
            size="lg"
            className="justify-start text-left mb-2 p-4 hover:bg-[#3E3E42] hover:text-white"
            onClick={onNewProject}
          >
            <FolderPlus className="mr-2 h-6 w-6" />
            Nouveau projet
          </Button>
        </div>

        <div className="mt-auto">
          <div className="grid grid-cols-2 gap-2 w-32">
            <Button
              variant="ghost"
              size="lg"
              className="justify-start p-4"
              onClick={async () =>
                await openLink("https://github.com/DonatFortini/eve_mapmaker")
              }
            >
              <Book className="mr-2 h-6 w-6" />
            </Button>
            <Button variant="ghost" size="lg" className="justify-start p-4">
              <Settings className="mr-2 h-6 w-6" />
            </Button>
          </div>
        </div>
      </div>

      <div className="flex flex-col p-8 overflow-auto">
        <div className="max-w-4xl mx-auto">
          <h2 className="text-xl font-semibold mb-4">Projet Récents</h2>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
            {recentMaps.map((map) => (
              <MapPreview
                title={map.name}
                imageUrl={`/path/to/map/images/${map.id}.png`}
                onClick={() => console.log(`Clicked on map ${map.id}`)}
              />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
