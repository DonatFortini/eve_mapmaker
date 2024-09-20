"use client"

import { useState, useEffect } from 'react'
import { Map, Loader2 } from 'lucide-react'
import { Progress } from "@/components/ui/progress"
import {  } from "";

export default function Component() {
  const [progress, setProgress] = useState(0)

  useEffect(() => {
    const timer = setInterval(() => {
      setProgress((prevProgress) => {
        if (prevProgress >= 100) {
          clearInterval(timer)
          return 100
        }
        return prevProgress + 10
      })
    }, 500)

    return () => clearInterval(timer)
  }, [])

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gradient-to-b from-blue-100 to-blue-200 p-4">
      <div className="text-center space-y-4">
        <Map className="w-16 h-16 text-blue-500 mx-auto animate-pulse" />
        <h1 className="text-2xl font-bold text-blue-800">Loading Your Maps</h1>
        <p className="text-blue-600">Please wait while we prepare your personalized map experience.</p>
        <div className="flex items-center justify-center space-x-2">
          <Loader2 className="w-4 h-4 animate-spin text-blue-500" />
          <span className="text-sm text-blue-700">Loading data...</span>
        </div>
        <Progress value={progress} className="w-64 mx-auto" />
        <p className="text-sm text-blue-600">{progress}% complete</p>
      </div>
    </div>
  )
}