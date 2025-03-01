"use client"

import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { Heart, Search } from "lucide-react"
import { useState } from "react"
import { AppSidebar } from "@/components/app-sidebar"
import { ArbitrumBanner } from "@/components/arbitrum-banner"
import { MainNav } from "@/components/main-nav"
import Link from "next/link"


type Repository = {
  id: string
  name: string
  description: string
  language: string
  languageColor: string
  updatedAt: string
  stars: number
}

const repositories: Repository[] = [
  {
    id: "1",
    name: "gitlab-runner",
    description:
      "GitLab Runner is the open source project that is used to run your CI/CD jobs and send the results back to GitLab",
    language: "Go",
    languageColor: "bg-cyan-500",
    updatedAt: "3 hours ago",
    stars: 2400,
  },
  {
    id: "2",
    name: "gitfreedom",
    description: "A modern Git platform for the decentralized web",
    language: "TypeScript",
    languageColor: "bg-blue-500",
    updatedAt: "2 days ago",
    stars: 1200,
  },
  {
    id: "3",
    name: "gitfreedom-ui",
    description: "UI components for GitFreedom",
    language: "TypeScript",
    languageColor: "bg-blue-500",
    updatedAt: "1 week ago",
    stars: 800,
  },
  {
    id: "4",
    name: "actix-web",
    description: "Web framework for Rust",
    language: "Rust",
    languageColor: "bg-orange-500",
    updatedAt: "2 weeks ago",
    stars: 3200,
  },
]

export default function HomePage() {
  const [search, setSearch] = useState("")

  const filteredRepositories = repositories.filter(
    (repo) =>
      repo.name.toLowerCase().includes(search.toLowerCase()) ||
      repo.description.toLowerCase().includes(search.toLowerCase()),
  )

  return (
    <div className="overflow-x-hidden w-full min-h-screen bg-background">
      <div className="flex">
        <AppSidebar />
        <main className="flex-1">
          <ArbitrumBanner />
          <MainNav />
          <div className="container py-6 px-6">
            <div className="flex items-center gap-4 mb-6">
              <div className="relative flex-1">
                <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
                <Input
                  placeholder="Find a repository..."
                  className="pl-9"
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                />
              </div>
              <Button>New</Button>
            </div>

            <div className="space-y-4">
              {filteredRepositories.map((repo) => (
                <Card key={repo.id} className="p-4">
                  <div className="flex items-start justify-between">
                    <div className="space-y-1">
                      <Link href={"/repositories/details"} className="text-lg font-semibold hover:underline">
                        {repo.name}
                      </Link>
                      <p className="text-sm text-muted-foreground">{repo.description}</p>
                      <div className="flex items-center gap-4">
                        <div className="flex items-center gap-2">
                          <div className={`h-3 w-3 rounded-full ${repo.languageColor}`} />
                          <span className="text-sm">{repo.language}</span>
                        </div>
                        <span className="text-sm text-muted-foreground">Updated {repo.updatedAt}</span>
                      </div>
                    </div>
                    <Button variant="outline" size="sm">
                      <Heart className="mr-2 h-4 w-4" />
                      Fan
                    </Button>
                  </div>
                </Card>
              ))}
            </div>
          </div>
        </main>
      </div>
    </div>
  )
}

function useEffect(arg0: () => void, arg1: never[]) {
  throw new Error("Function not implemented.")
}

