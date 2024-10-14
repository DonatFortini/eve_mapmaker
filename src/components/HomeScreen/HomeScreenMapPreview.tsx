import { useState } from "react";
import { Card, CardContent } from "@/components/ui/card";
import { MapPin } from "lucide-react";

interface HomeScreenMapPreviewProps {
  title: string;
  image_path: string;
  project_path: string;
  onClick: () => void;
}

export default function HomeScreenMapPreview({
  title,
  image_path,
  project_path,
  onClick,
}: HomeScreenMapPreviewProps) {
  const [isHovered, setIsHovered] = useState(false);

  return (
    <Card
      className={`w-full bg-[#252526] border-none shadow-xl transition-all duration-300 cursor-pointer ${
        isHovered ? "transform scale-105" : ""
      }`}
      onClick={onClick}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <CardContent className="p-2 sm:p-4">
        <div className="relative aspect-video mb-2 overflow-hidden rounded-md">
          {image_path ? (
            <img
              src={image_path}
              alt={`Preview of ${image_path}`}
              className="w-full h-full object-cover"
            />
          ) : (
            <div className="w-full h-full bg-[#3E3E42] flex items-center justify-center">
              <MapPin className="w-8 h-8 sm:w-12 sm:h-12 text-[#CCCCCC]" />
            </div>
          )}
        </div>
        <h3 className="text-sm sm:text-lg font-semibold text-[#CCCCCC] truncate">
          {title}
        </h3>
        <p className="text-xs sm:text-sm text-[#CCCCCC] truncate">
          {project_path}
        </p>
      </CardContent>
    </Card>
  );
}
