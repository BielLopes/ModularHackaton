import {
  FileIcon,
  FolderIcon,
  GitCompareIcon as GitIgnoreIcon,
  DiscIcon as LicenseIcon,
  BookMarkedIcon as MarkdownIcon,
  GemIcon as YamlIcon,
} from "lucide-react"
import { cn } from "@/lib/utils"

export const getFileIcon = (filename: string) => {
  const ext = filename.split(".").pop()?.toLowerCase()

  switch (ext) {
    case "md":
      return MarkdownIcon
    case "yml":
    case "yaml":
      return YamlIcon
    case "gitignore":
      return GitIgnoreIcon
    case "license":
      return LicenseIcon
    default:
      return FileIcon
  }
}

export function FolderDot({ color }: { color?: string }) {
  return (
    <div className="relative">
      <FolderIcon className="h-5 w-5" />
      {color && <div className={cn("absolute -bottom-0.5 -left-0.5 h-2 w-2 rounded-full", color)} />}
    </div>
  )
}

