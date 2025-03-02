"use client"

import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import { Progress } from "@/components/ui/progress"
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs"
import { Download, Eye, GitBranch, GitCommit, GitFork, Heart, Book, HeartHandshake, Scale } from "lucide-react"
import { FileBrowser } from "@/components/file-browser"
import { useState } from "react"

type ViewMode = "info" | "code"

export function RepositoryView() {
  const [viewMode, setViewMode] = useState<ViewMode>("info")

  return (
    <div className="container py-6">
      <div className="flex items-start justify-between">
        <div>
          <h1 className="text-2xl font-bold">gitlab-runner</h1>
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

      {viewMode === "info" ? (
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mt-6">
          <div className="md:col-span-2">
            <Tabs defaultValue="readme" className="w-full">
              <TabsList className="w-full justify-start h-auto p-0 bg-transparent border-b rounded-none">
                <TabsTrigger
                  value="readme"
                  className="data-[state=active]:border-b-2 data-[state=active]:border-red-500 rounded-none border-b-2 border-transparent px-4 py-2"
                >
                  <Book className="mr-2 h-4 w-4" />
                  README
                </TabsTrigger>
                <TabsTrigger
                  value="conduct"
                  className="data-[state=active]:border-b-2 data-[state=active]:border-red-500 rounded-none border-b-2 border-transparent px-4 py-2"
                >
                  <HeartHandshake className="mr-2 h-4 w-4" />
                  Code of conduct
                </TabsTrigger>
                <TabsTrigger
                  value="license"
                  className="data-[state=active]:border-b-2 data-[state=active]:border-red-500 rounded-none border-b-2 border-transparent px-4 py-2"
                >
                  <Scale className="mr-2 h-4 w-4" />
                  License
                </TabsTrigger>
              </TabsList>
              <TabsContent value="readme">
                <Card className="p-6 mt-6">
                  <div className="prose dark:prose-invert max-w-none">
                    <h1>GitLab Runner</h1>
                    <p>
                      GitLab Runner is the open source project that is used to run your CI/CD jobs and send the results
                      back to GitLab.
                    </p>
                    <h2>Installation</h2>
                    <p>
                      GitLab Runner can be installed and used on GNU/Linux, macOS, FreeBSD, and Windows. There are three
                      ways to install it:
                    </p>
                    <ul>
                      <li>Repository installation (recommended)</li>
                      <li>Manual installation</li>
                      <li>Docker installation</li>
                    </ul>
                  </div>
                </Card>
              </TabsContent>
              <TabsContent value="conduct">
                <Card className="p-6 mt-6">
                  <div className="prose dark:prose-invert max-w-none">
                    <h1>Code of Conduct</h1>
                    <h2>Our Pledge</h2>
                    <p>
                      In the interest of fostering an open and welcoming environment, we as contributors and maintainers
                      pledge to making participation in our project and our community a harassment-free experience for
                      everyone.
                    </p>
                    <h2>Our Standards</h2>
                    <p>Examples of behavior that contributes to creating a positive environment include:</p>
                    <ul>
                      <li>Using welcoming and inclusive language</li>
                      <li>Being respectful of differing viewpoints and experiences</li>
                      <li>Gracefully accepting constructive criticism</li>
                      <li>Focusing on what is best for the community</li>
                    </ul>
                  </div>
                </Card>
              </TabsContent>
              <TabsContent value="license">
                <Card className="p-6 mt-6">
                  <div className="prose dark:prose-invert max-w-none">
                    <h1>BSD 3-Clause License</h1>
                    <p>Copyright (c) 2023, GitLab Inc.</p>
                    <p>
                      Redistribution and use in source and binary forms, with or without modification, are permitted
                      provided that the following conditions are met:
                    </p>
                    <ol>
                      <li>
                        Redistributions of source code must retain the above copyright notice, this list of conditions
                        and the following disclaimer.
                      </li>
                      <li>
                        Redistributions in binary form must reproduce the above copyright notice, this list of
                        conditions and the following disclaimer in the documentation and/or other materials provided
                        with the distribution.
                      </li>
                      <li>
                        Neither the name of the copyright holder nor the names of its contributors may be used to
                        endorse or promote products derived from this software without specific prior written
                        permission.
                      </li>
                    </ol>
                  </div>
                </Card>
              </TabsContent>
            </Tabs>
          </div>

          <div className="space-y-6">
            <Card className="p-4">
              <h3 className="font-medium mb-4">Repository Stats</h3>
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <Heart className="h-4 w-4 text-red-500" />
                    Fans
                  </div>
                  <Badge variant="secondary">2.4k</Badge>
                </div>
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <Eye className="h-4 w-4" />
                    Watching
                  </div>
                  <Badge variant="secondary">142</Badge>
                </div>
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <GitFork className="h-4 w-4" />
                    Forks
                  </div>
                  <Badge variant="secondary">2.3k</Badge>
                </div>
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <GitCommit className="h-4 w-4" />
                    Commits
                  </div>
                  <Badge variant="secondary">12.4k</Badge>
                </div>
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <GitBranch className="h-4 w-4" />
                    Branches
                  </div>
                  <Badge variant="secondary">887</Badge>
                </div>
              </div>
            </Card>

            <Card className="p-4">
              <h3 className="font-medium mb-4">Languages</h3>
              <div className="space-y-4">
                <div>
                  <div className="flex justify-between mb-1 text-sm">
                    <span>Go</span>
                    <span>64.2%</span>
                  </div>
                  <Progress value={64.2} className="h-2" />
                </div>
                <div>
                  <div className="flex justify-between mb-1 text-sm">
                    <span>Shell</span>
                    <span>20.3%</span>
                  </div>
                  <Progress value={20.3} className="h-2" />
                </div>
                <div>
                  <div className="flex justify-between mb-1 text-sm">
                    <span>Ruby</span>
                    <span>15.5%</span>
                  </div>
                  <Progress value={15.5} className="h-2" />
                </div>
              </div>
            </Card>
          </div>
        </div>
      ) : (
        <FileBrowser />
      )}
    </div>
  )
}

