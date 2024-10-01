import { useState } from "react";
import { Card, CardContent } from "@/components/ui/card";
import { MapPin } from "lucide-react";

interface MapPreviewProps {
  title: string;
  imageUrl: string;
  onClick: () => void;
}

export default function MapPreview({
  title,
  imageUrl,
  onClick,
}: MapPreviewProps) {
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
          {imageUrl ? (
            <img
              src={imageUrl}
              alt={`Preview of ${title}`}
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
      </CardContent>
    </Card>
  );
}
