"use client"

import { ArbitrumBanner } from "@/components/arbitrum-banner"
import { MainNav } from "@/components/main-nav"
import { RepositoryInfo } from "@/components/repository-info"
import { FileBrowser } from "@/components/file-browser"
import { AppSidebar } from "@/components/app-sidebar"
import { Button } from "@/components/ui/button"
import { Download, GitFork, Heart } from 'lucide-react'
import { useState } from "react"
import { useSearchParams } from 'next/navigation'

export default function RepositoryPage() {
  const [viewMode, setViewMode] = useState<"info" | "code">("info")

  const searchParams = useSearchParams();
  const repoName = searchParams.get('name');
  

  interface MainNavProps {
    viewMode: "info" | "code";
    onViewModeChange: (mode: "info" | "code") => void;
  }

  return (
    <div className="overflow-hidden min-h-screen bg-background">
      <div className="flex">
        <AppSidebar />
        <main className="flex-1">
          <ArbitrumBanner />
          <MainNav viewMode={viewMode} onViewModeChange={setViewMode} />
          <div className="container py-6 px-5">
            <div className="flex items-start justify-between">
              <div>
                <h1 className="text-2xl font-bold">{repoName}</h1>
              </div>
              <div className="flex gap-2">
                <Button variant="outline" size="sm">
                  <Heart className="mr-2 h-4 w-4" />
                  Fan
                </Button>
                <Button variant="outline" size="sm">
                  <GitFork className="mr-2 h-4 w-4" />
                  Fork
                </Button>
                <Button variant="outline" size="sm">
                  <Download className="mr-2 h-4 w-4" />
                  Download
                </Button>
              </div>
            </div>

            {viewMode === "info" ? <RepositoryInfo /> : <FileBrowser />}
          </div>
        </main>
      </div>
    </div>
  )
}

