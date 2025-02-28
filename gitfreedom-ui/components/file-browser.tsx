import { Badge } from "@/components/ui/badge"
import { Card } from "@/components/ui/card"
import { getFileIcon, FolderDot } from "@/components/file-icons"
import * as React from "react" // Import React

type FileEntry = {
  name: string
  type: "file" | "folder"
  lastCommit: string
  lastCommitTime: string
  specialFolder?: string
}

const files: FileEntry[] = [
  {
    name: ".github",
    type: "folder",
    lastCommit: "Add team-ios label to relevant PRs (#162491)",
    lastCommitTime: "last week",
    specialFolder: "bg-blue-500",
  },
  {
    name: ".vscode",
    type: "folder",
    lastCommit: "Auto-format Framework (#160545)",
    lastCommitTime: "2 months ago",
    specialFolder: "bg-blue-500",
  },
  {
    name: "bin",
    type: "folder",
    lastCommit: "Alter update_engine_version.sh to lookup engine hash based on version",
    lastCommitTime: "1 hour ago",
  },
  {
    name: "docs",
    type: "folder",
    lastCommit: "Update iOS / macOS triage links (#163171)",
    lastCommitTime: "2 days ago",
    specialFolder: "bg-yellow-500",
  },
  {
    name: ".gitignore",
    type: "file",
    lastCommit: "Added gclient files to gitignore to support Flock and other Flutter tools",
    lastCommitTime: "1 hour ago",
  },
  {
    name: "CHANGELOG.md",
    type: "file",
    lastCommit: "Merge CHANGELOG for 3.27.4 stable release (#162761)",
    lastCommitTime: "last week",
  },
  {
    name: "LICENSE",
    type: "file",
    lastCommit: "License update (#45373)",
    lastCommitTime: "6 years ago",
  },
  {
    name: "README.md",
    type: "file",
    lastCommit: "Update documentation links",
    lastCommitTime: "3 days ago",
  },
]

export function FileBrowser() {
  return (
    <Card className="mt-6">
      <div className="divide-y divide-border">
        {files.map((file) => (
          <div key={file.name} className="flex items-center justify-between px-4 py-3 hover:bg-muted/50 group">
            <div className="flex items-center gap-3">
              {file.type === "folder" ? (
                <FolderDot color={file.specialFolder} />
              ) : (
                React.createElement(getFileIcon(file.name), {
                  className: "h-5 w-5",
                })
              )}
              <span className="font-medium">{file.name}</span>
            </div>
            <div className="flex items-center gap-2">
              <p className="text-sm text-muted-foreground">{file.lastCommit}</p>
              <Badge variant="secondary" className="text-xs">
                {file.lastCommitTime}
              </Badge>
            </div>
          </div>
        ))}
      </div>
    </Card>
  )
}

