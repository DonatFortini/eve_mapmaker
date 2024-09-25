"use client"

import { Button } from "@/components/ui/button"
import { MapPin, Plus, History, Download, Book, Settings } from "lucide-react"
import { useState } from 'react'

interface HomeScreenProps {
  onNewProject: () => void;
  onMainScreen: () => void;
}


export default function HomeScreen({ onNewProject, onMainScreen }: HomeScreenProps) {
  const recentMaps = [
    { id: 1, name: "City Center Map" },
    { id: 2, name: "National Park Trails" }
  ]

  return (
    <div className="flex h-screen w-screen bg-[#2D2D30] text-[#CCCCCC]">
      <div className="bg-[#252526] p-4 flex flex-col w-64 transition-all duration-300 ease-in-out">
        <div className="mb-8">
          <h1 className="text-2xl font-semibold">MapMaker</h1>
        </div>

        <div className="flex flex-col">
          <Button 
            variant="ghost" 
            className="justify-start text-left mb-2 hover:bg-[#3E3E42] hover:text-white"
            onClick={onMainScreen}
          >
            <MapPin className="mr-2 h-4 w-4" />
            Charger un projet
          </Button>
          <Button 
            variant="ghost" 
            className="justify-start text-left mb-2 hover:bg-[#3E3E42] hover:text-white"
            onClick={onNewProject}
          >
            <Plus className="mr-2 h-4 w-4" />
            Générer un nouveau projet
          </Button>
        </div>
          
        <div className="mt-auto">
          <h2 className="text-sm font-semibold mb-2">Quick Access</h2>
          <div className="grid grid-cols-2 gap-2">
            <Button variant="ghost" size="sm" className="justify-start">
              <Book className="mr-2 h-4 w-4" />
              Docs
            </Button>
            <Button variant="ghost" size="sm" className="justify-start">
              <Settings className="mr-2 h-4 w-4" />
              Paramètres
            </Button>
          </div>
        </div>
        
      </div>

      
      <div className="flex flex-col p-8 overflow-auto">
        <div className="max-w-4xl mx-auto">
          <h2 className="text-xl font-semibold mb-4">Projet Récents</h2>
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
