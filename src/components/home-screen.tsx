"use client"

import { Button } from "@/components/ui/button"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { MapPin, Plus, History, Download, Share2, Settings } from "lucide-react"

export default function HomeScreen() {
  const recentMaps = [
    { id: 1, name: "City Center Map" },
    { id: 2, name: "National Park Trails" },
    { id: 3, name: "Downtown Restaurant Guide" },
    { id: 4, name: "Subway Network" },
    { id: 5, name: "Historical Landmarks" },
    { id: 6, name: "Bike Paths" },
  ]

  return (
    <div className="flex h-screen bg-[#2D2D30] text-[#CCCCCC]">
      {/* Left sidebar */}
      <div className="w-64 bg-[#252526] p-4 flex flex-col">
        <div className="flex items-center justify-between mb-8">
          <h1 className="text-2xl font-semibold">MapMaster</h1>
          <Avatar className="h-8 w-8">
            <AvatarImage src="/placeholder-avatar.jpg" alt="User" />
            <AvatarFallback>JD</AvatarFallback>
          </Avatar>
        </div>
        <Button 
          variant="ghost" 
          className="justify-start text-left mb-2 hover:bg-[#3E3E42] hover:text-white"
        >
          <MapPin className="mr-2 h-4 w-4" />
          Load Previous Map
        </Button>
        <Button 
          variant="ghost" 
          className="justify-start text-left mb-2 hover:bg-[#3E3E42] hover:text-white"
        >
          <Plus className="mr-2 h-4 w-4" />
          Generate New Map
        </Button>
        <div className="mt-auto">
          <h2 className="text-sm font-semibold mb-2">Quick Access</h2>
          <div className="grid grid-cols-2 gap-2">
            <Button variant="ghost" size="sm" className="justify-start">
              <History className="mr-2 h-4 w-4" />
              History
            </Button>
            <Button variant="ghost" size="sm" className="justify-start">
              <Download className="mr-2 h-4 w-4" />
              Download
            </Button>
            <Button variant="ghost" size="sm" className="justify-start">
              <Share2 className="mr-2 h-4 w-4" />
              Share
            </Button>
            <Button variant="ghost" size="sm" className="justify-start">
              <Settings className="mr-2 h-4 w-4" />
              Settings
            </Button>
          </div>
        </div>
      </div>

      {/* Main content area */}
      <div className="flex-1 p-8 overflow-auto">
        <div className="max-w-4xl mx-auto">
          <h2 className="text-xl font-semibold mb-4">Recent Maps</h2>
          <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
            {recentMaps.map((map) => (
              <Button
                key={map.id}
                variant="outline"
                className="h-40 flex flex-col items-center justify-center text-center p-2 hover:bg-[#3E3E42] border-dashed border-2 border-gray-600"
              >
                <MapPin className="mb-2 h-8 w-8 text-gray-500" />
                <span className="text-sm">{map.name}</span>
              </Button>
            ))}
          </div>
        </div>
      </div>
    </div>
  )
}