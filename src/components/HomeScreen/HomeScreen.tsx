"use client";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import HomeScreenSidebar from "@/components/HomeScreen/HomeScreenSidebar";
import HomeScreenMainContent from "@/components/HomeScreen/HomeScreenMainContent";
import { RecentMap } from "@/components/types/map";

interface HomeScreenProps {
  onNewProject: () => void;
}

const HomeScreen: React.FC<HomeScreenProps> = ({ onNewProject }) => {
  const [recentMaps, setRecentMaps] = useState<RecentMap[]>([]);

  useEffect(() => {
    const fetchProjects = async () => {
      try {
        const maps = await invoke("get_projects");
        if (typeof maps === "object" && maps !== null) {
          const processedMaps = Object.entries(maps).map(([title, paths]) => ({
            title,
            data: [
              {
                image_path: (paths as string[])[0],
                project_path: (paths as string[])[1],
              },
            ],
          }));
          setRecentMaps(processedMaps);
        } else {
          console.error("Expected an object but got:", maps);
        }
      } catch (error) {
        console.error("Failed to fetch projects:", error);
      }
    };

    fetchProjects();
  }, []);

  return (
    <div className="flex h-screen w-screen bg-[#2D2D30] text-[#CCCCCC]">
      <HomeScreenSidebar onNewProject={onNewProject} />
      <HomeScreenMainContent recentMaps={recentMaps} />
    </div>
  );
};

export default HomeScreen;
