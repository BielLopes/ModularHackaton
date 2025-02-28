import { Button } from "@/components/ui/button"
import { Info } from "lucide-react"
import { useSidebar } from "@/components/ui/sidebar"

export function ArbitrumBanner() {
  const { state } = useSidebar()

  return (
    <div className="bg-[#12AAFF]/10 border-b border-[#12AAFF]/20 py-2 pl-2 pr-6">
      <div
        className={`${state === "expanded" ? "container" : "px-4"} flex items-center justify-between transition-all duration-300`}
      >
        <div className="flex items-center gap-2">
          <Info className="mr-2 h-4 w-4 text-[#12AAFF]" />
          <p className="text-sm text-[#12AAFF]">Support GitFreedom development by donating with $ARB</p>
        </div>
        <Button size="sm" variant="outline" className="border-[#12AAFF] text-[#12AAFF]">
          Donate
          <img
            src="https://hebbkx1anhila5yf.public.blob.vercel-storage.com/Arbitrum-WR0Slodrcm6Df4wGGWgf39whCst31G.png"
            alt="Arbitrum"
            className="h-6 w-6"
          />
        </Button>
      </div>
    </div>
  )
}

