import { FolderPlus, Book, Settings, FolderInput } from "lucide-react";
import { open as openLink } from "@tauri-apps/api/shell";
import { Button } from "@/components/ui/button";

interface HomeScreenSidebarProps {
  onNewProject: () => void;
  onMainScreen: () => void;
}

const HomeScreenSidebar: React.FC<HomeScreenSidebarProps> = ({
  onNewProject,
  onMainScreen,
}) => (
  <div className="bg-[#252526] p-4 flex flex-col w-80 transition-all duration-300 ease-in-out">
    <div className="mb-8">
      <h1 className="text-4xl font-bold text-white tracking-wider">
        <span className="text-[#FFD700]">Map</span>Maker
      </h1>
    </div>
    <div className="flex flex-col">
      <HomeScreenSidebarButton
        onClick={onMainScreen}
        icon={<FolderInput className="mr-2 h-6 w-6" />}
        text="Charger un projet"
      />
      <HomeScreenSidebarButton
        onClick={onNewProject}
        icon={<FolderPlus className="mr-2 h-6 w-6" />}
        text="Nouveau projet"
      />
    </div>
    <div className="mt-auto">
      <div className="grid grid-cols-2 gap-2 w-32">
        <HomeScreenSidebarButton
          onClick={async () =>
            await openLink(
              "https://github.com/DonatFortini/eve_mapmaker/blob/main/README.md"
            )
          }
          icon={<Book className="mr-2 h-6 w-6" />}
        />
        <HomeScreenSidebarButton icon={<Settings className="mr-2 h-6 w-6" />} />
      </div>
    </div>
  </div>
);

interface HomeScreenSidebarButtonProps {
  onClick?: () => void;
  icon: React.ReactNode;
  text?: string;
}

const HomeScreenSidebarButton: React.FC<HomeScreenSidebarButtonProps> = ({
  onClick,
  icon,
  text,
}) => (
  <Button
    variant="ghost"
    size="lg"
    className="justify-start text-left mb-2 p-4 hover:bg-[#3E3E42] hover:text-white"
    onClick={onClick}
  >
    {icon}
    {text}
  </Button>
);

export default HomeScreenSidebar;
