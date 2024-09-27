"use client";
import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/tauri";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Card, CardContent } from "@/components/ui/card";
import { Folder, ChevronRight, Building2, ArrowLeft } from "lucide-react";

interface NewProjectScreenProps {
  onGoBack: () => void;
  onLoading: (department: string, projectName: string) => void;
}

const NewProjectScreen: React.FC<NewProjectScreenProps> = ({
  onGoBack,
  onLoading,
}) => {
  const [projectName, setProjectName] = useState("");
  const [department, setDepartment] = useState("");
  const [departments, setDepartments] = useState<{ [key: string]: string }>({});

  useEffect(() => {
    const fetchDepartments = () => {
      invoke("get_dpts_list")
        .then((res) => setDepartments(res as { [key: string]: string }))
        .catch((err) => {
          console.error(err);
        });
    };
    fetchDepartments();
  }, []);

  return (
    <div className="min-h-screen bg-[#2D2D30] text-[#CCCCCC] p-8">
      <Button variant="ghost" className="mb-4" onClick={onGoBack}>
        <ArrowLeft size={24} />
      </Button>
      <Card className="w-full max-w-md bg-[#252526] border-none shadow-xl">
        <CardContent className="p-8">
          <h1 className="text-2xl font-bold text-blue-400 mb-10">
            Nouveau Projet
          </h1>
          <div className="space-y-10">
            <div className="space-y-2">
              <label
                htmlFor="project-name"
                className="block text-sm font-medium text-gray-400"
              >
                Nom du projet
              </label>
              <div className="relative">
                <Folder
                  className="absolute left-3 top-1/2 transform -translate-y-1/2 text-blue-400"
                  size={20}
                />
                <Input
                  id="project-name"
                  value={projectName}
                  onChange={(e) => setProjectName(e.target.value)}
                  placeholder="nom du projet"
                  className="pl-10 bg-[#3E3E42] border-[#3E3E42] text-white rounded-full h-12"
                />
              </div>
            </div>
            <div className="space-y-2">
              <label
                htmlFor="department"
                className="block text-sm font-medium text-gray-400"
              >
                Department
              </label>
              <div className="relative">
                <Building2
                  className="absolute left-3 top-1/2 transform -translate-y-1/2 text-blue-400"
                  size={20}
                />
                <Select value={department} onValueChange={setDepartment}>
                  <SelectTrigger
                    id="department"
                    className="pl-10 bg-[#3E3E42] border-[#3E3E42] text-white rounded-full h-12"
                  >
                    <SelectValue placeholder="Selectionez votre département" />
                  </SelectTrigger>
                  <SelectContent className="bg-[#3E3E42] border-[#3E3E42] text-white">
                    {Object.entries(departments).map(([key, value]) => (
                      <SelectItem key={key} value={key}>
                        {value}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>
            <Button
              onClick={() => onLoading(department, projectName)}
              className="w-full bg-blue-600 hover:bg-blue-700 rounded-full h-12 mt-6"
              disabled={!projectName || !department || !departments[department]}
            >
              Suivant
              <ChevronRight className="ml-2" size={20} />
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
};

export default NewProjectScreen;
