// HomeScreenMainContent.tsx
import MapPreview from "@/components/HomeScreen/HomeScreenMapPreview";
import { RecentMap } from "@/components/types/map";

interface HomeScreenMainContentProps {
  recentMaps: RecentMap[];
}

const HomeScreenMainContent: React.FC<HomeScreenMainContentProps> = ({
  recentMaps,
}) => (
  <div className="flex flex-col p-8 overflow-auto">
    <div className="max-w-4xl mx-auto">
      <h2 className="text-xl font-semibold mb-4">Projet Récents</h2>
      <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
        {recentMaps.map((map) => (
          <MapPreview
            key={map.title}
            title={map.title}
            image_path={map.data[0].image_path}
            project_path={map.data[0].project_path}
            onClick={() => console.log("Open map", map.title)}
          />
        ))}
      </div>
    </div>
  </div>
);

export default HomeScreenMainContent;
