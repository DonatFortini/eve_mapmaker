"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Slider } from "@/components/ui/slider";
import { Switch } from "@/components/ui/switch";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import {
  MapPin,
  Layers,
  ZoomIn,
  ZoomOut,
  RotateCw,
  Download,
  Share2,
  Settings,
  ChevronLeft,
  ChevronRight,
  ArrowLeft,
} from "lucide-react";

interface MainScreenProps {
  onGoBack: () => void;
}

const MainScreen: React.FC<MainScreenProps> = ({ onGoBack }) => {
  const [zoomLevel, setZoomLevel] = useState(50);
  const [showLayers, setShowLayers] = useState(false);
  const [isCollapsed, setIsCollapsed] = useState(false);

  return (
    <div className="flex h-screen bg-[#2D2D30] text-[#CCCCCC]">
      <div
        className={`bg-[#252526] flex flex-col transition-all duration-300 ease-in-out ${
          isCollapsed ? "w-16" : "w-64"
        }`}
      >
        <Button variant="ghost" className="mb-4" onClick={onGoBack}>
          <ArrowLeft size={24} />
        </Button>
        <div className="p-4 flex items-center justify-between">
          <h2
            className={`text-xl font-semibold ${
              isCollapsed ? "hidden" : "block"
            }`}
          >
            Map Controls
          </h2>
          <Button
            variant="ghost"
            size="icon"
            onClick={() => setIsCollapsed(!isCollapsed)}
          >
            {isCollapsed ? (
              <ChevronRight className="h-4 w-4" />
            ) : (
              <ChevronLeft className="h-4 w-4" />
            )}
          </Button>
        </div>

        <div
          className={`flex-grow overflow-y-auto ${
            isCollapsed ? "hidden" : "block"
          }`}
        >
          <div className="p-4 space-y-4">
            <div>
              <label
                htmlFor="search"
                className="block text-sm font-medium mb-1"
              >
                Search Location
              </label>
              <Input
                id="search"
                placeholder="Enter location..."
                className="bg-[#3E3E42] border-[#3E3E42]"
              />
            </div>

            <div>
              <label
                htmlFor="layers"
                className="block text-sm font-medium mb-1"
              >
                Show Layers
              </label>
              <div className="flex items-center">
                <Switch
                  id="layers"
                  checked={showLayers}
                  onCheckedChange={setShowLayers}
                />
                <label htmlFor="layers" className="ml-2 text-sm">
                  {showLayers ? "On" : "Off"}
                </label>
              </div>
            </div>

            <Button variant="secondary" className="w-full justify-start">
              <Layers className="mr-2 h-4 w-4" />
              Manage Layers
            </Button>

            <Button variant="secondary" className="w-full justify-start">
              <RotateCw className="mr-2 h-4 w-4" />
              Reset View
            </Button>
          </div>
        </div>

        <div
          className={`mt-auto p-4 space-y-2 ${
            isCollapsed ? "hidden" : "block"
          }`}
        >
          <Button variant="ghost" className="w-full justify-start">
            <Download className="mr-2 h-4 w-4" />
            Export Map
          </Button>
          <Button variant="ghost" className="w-full justify-start">
            <Share2 className="mr-2 h-4 w-4" />
            Share
          </Button>
          <Button variant="ghost" className="w-full justify-start">
            <Settings className="mr-2 h-4 w-4" />
            Settings
          </Button>
        </div>
      </div>

      {/* Main map display area */}
      <div className="flex-1 relative">
        {/* Placeholder for map */}
        <div className="absolute inset-0 bg-[#1E1E1E] flex items-center justify-center">
          <MapPin className="h-16 w-16 text-gray-600" />
        </div>

        {/* Top right controls */}
        <div className="absolute top-4 right-4">
          <Avatar className="h-8 w-8">
            <AvatarImage src="/placeholder-avatar.jpg" alt="User" />
            <AvatarFallback>JD</AvatarFallback>
          </Avatar>
        </div>

        {/* Bottom right zoom controls */}
        <div className="absolute bottom-4 right-4 flex flex-col items-center space-y-2 bg-[#252526] p-2 rounded-md">
          <Button
            variant="ghost"
            size="icon"
            onClick={() => setZoomLevel(Math.min(zoomLevel + 10, 100))}
          >
            <ZoomIn className="h-4 w-4" />
          </Button>
          <Slider
            orientation="vertical"
            value={[zoomLevel]}
            onValueChange={([value]) => setZoomLevel(value)}
            className="h-32"
          />
          <Button
            variant="ghost"
            size="icon"
            onClick={() => setZoomLevel(Math.max(zoomLevel - 10, 0))}
          >
            <ZoomOut className="h-4 w-4" />
          </Button>
        </div>
      </div>
    </div>
  );
};

export default MainScreen;
