"use client"

import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { GitBranch, Info, Code, AlertCircle, GitPullRequestIcon, Play, BookText, Users } from 'lucide-react'
import { SidebarTrigger } from "@/components/ui/sidebar"
import { usePathname } from 'next/navigation'
import { Button } from "@/components/ui/button"

export function MainNav({ viewMode, onViewModeChange }: { viewMode?: string, onViewModeChange?: (mode: string) => void }) {
  const pathname = usePathname()
  const isRepositoryPage = pathname.startsWith('/repository/')

  return (
    <div className="border-b">
      <div className="container flex h-14 items-center max-w-max">
        <div className="flex items-center gap-4">
          <SidebarTrigger className="h-8 w-8 text-muted-foreground hover:text-foreground" />
          {isRepositoryPage && (
            <>
              <Select defaultValue="main">
                <SelectTrigger className="w-[180px]">
                  <GitBranch className="mr-2 h-4 w-4" />
                  <SelectValue placeholder="Select branch" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="main">main</SelectItem>
                  <SelectItem value="develop">develop</SelectItem>
                  <SelectItem value="feature">feature</SelectItem>
                </SelectContent>
              </Select>
              <div className="flex items-center space-x-2">
                <Button 
                  variant="ghost" 
                  size="sm" 
                  onClick={() => onViewModeChange?.('info')}
                  className={viewMode === 'info' ? 'bg-muted' : ''}
                >
                  <Info className="h-4 w-4 mr-2" />
                  Info
                </Button>
                <Button 
                  variant="ghost" 
                  size="sm" 
                  onClick={() => onViewModeChange?.('code')}
                  className={viewMode === 'code' ? 'bg-muted' : ''}
                >
                  <Code className="h-4 w-4 mr-2" />
                  Code
                </Button>
                <Button variant="ghost" size="sm">
                  <GitPullRequestIcon className="h-4 w-4 mr-2" />
                  Pull Requests
                </Button>
                <Button variant="ghost" size="sm">
                  <AlertCircle className="h-4 w-4 mr-2" />
                  Issues
                </Button>
                <Button variant="ghost" size="sm">
                  <Users className="h-4 w-4 mr-2" />
                  Contributors
                </Button>
                <Button variant="ghost" size="sm">
                  <BookText className="h-4 w-4 mr-2" />
                  Wiki
                </Button>
                <Button variant="ghost" size="sm">
                  <Play className="h-4 w-4 mr-2" />
                  Actions
                </Button>
              </div>
            </>
          )}
        </div>
      </div>
    </div>
  )
}

