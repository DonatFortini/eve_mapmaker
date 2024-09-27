"use client";
import { useState, useEffect, useRef } from "react";
import { Map, Loader2, CheckCircle2 } from "lucide-react";
import { Card, CardContent } from "@/components/ui/card";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

const steps = [
  "Mise en Place",
  "Recherche des fichiers",
  "Téléchargement des données",
  "Preparation des Couches",
  "Finalisation",
];

interface LoaderScreenProps {
  department: string;
  projectName: string;
}

export default function LoaderScreen({
  department,
  projectName,
}: LoaderScreenProps) {
  const [currentStep, setCurrentStep] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const isInitialMount = useRef(true);

  useEffect(() => {
    if (!isInitialMount.current) return;

    isInitialMount.current = false;
    console.log("Initial Mount");

    const handleProgressUpdate = (event: any) => {
      const step = event.payload as string;
      const stepIndex = steps.indexOf(step);
      if (stepIndex !== -1) {
        setCurrentStep(stepIndex);
      }
    };

    const unlisten = listen("progress-update", handleProgressUpdate);
    const processMapCreation = () => {
      setCurrentStep(0);
      invoke("open_new_project", { code: department, name: projectName }).catch(
        (err) => {
          setError(err.message);
        }
      );
    };
    processMapCreation();

    return () => {
      unlisten.then((unsub) => unsub());
    };
  }, [department, projectName]);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-[#2D2D30] text-[#CCCCCC] p-4">
      <Card className="w-full max-w-md bg-[#252526] border-none shadow-xl">
        <CardContent className="p-6">
          <div className="text-center space-y-4">
            <Map className="w-16 h-16 text-blue-400 mx-auto animate-pulse" />
            <h1 className="text-2xl font-bold text-blue-400">
              Chargement de la Carte
            </h1>
            {error ? (
              <p className="text-red-500">{error}</p>
            ) : (
              <p className="text-gray-400">
                Veuillez patienter pendant que nous chargeons les données de la
                carte.
              </p>
            )}
            <div className="flex items-center justify-center space-x-2">
              {!error && (
                <Loader2 className="w-4 h-4 animate-spin text-blue-400" />
              )}
              <span className="text-sm text-gray-400">
                {error ? "Failed to load data." : "Loading data..."}
              </span>
            </div>
            <div className="space-y-2">
              {steps.map((step, index) => (
                <div key={index} className="flex items-center space-x-2">
                  {index < currentStep ? (
                    <CheckCircle2 className="w-5 h-5 text-green-500" />
                  ) : index === currentStep ? (
                    <div className="w-5 h-5 rounded-full bg-blue-500 animate-pulse" />
                  ) : (
                    <div className="w-5 h-5 rounded-full bg-gray-600" />
                  )}
                  <span
                    className={
                      index <= currentStep ? "text-blue-400" : "text-gray-600"
                    }
                  >
                    {step}
                  </span>
                </div>
              ))}
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
