import Link from "next/link"
import { Button } from "@/components/ui/button"
import {
  ArrowRight,
  Network,
  FileCode2,
  Github,
  Twitter,
  MessageSquare,
  TextIcon as Telegram,
  Mail,
  Youtube,
  SunMediumIcon as Medium,
} from "lucide-react"
import Image from "next/image"

export default function LandingPage() {
  return (
    <div className="flex-1 h-full bg-[#070e21] text-cyan-50 m-4">
      {/* Header */}
      <header className="sticky top-0 z-50 w-full border-b border-cyan-900/50 bg-[#070e21]/95 backdrop-blur supports-[backdrop-filter]:bg-[#070e21]/60">
        <div className="container flex h-16 items-center justify-between">
          <div className="flex items-center gap-2">

            <span className="text-xl font-bold text-cyan-400">&lt;git_freedom&gt;</span>
          </div>
          <nav className="hidden md:flex gap-6">
            <Link
              className="text-sm font-medium text-cyan-100 hover:text-cyan-400 hover:underline underline-offset-4"
              href="#features"
            >
              Features
            </Link>
            <Link
              className="text-sm font-medium text-cyan-100 hover:text-cyan-400 hover:underline underline-offset-4"
              href="#about"
            >
              About
            </Link>
            <Link
              className="text-sm font-medium text-cyan-100 hover:text-cyan-400 hover:underline underline-offset-4"
              href="#docs"
            >
              Docs
            </Link>
          </nav>
          <div className="flex items-center gap-4">
            <Link href={"/repositories/"}>
              <Button className="bg-cyan-500 hover:bg-cyan-600 text-[#070e21]">Launch App</Button>
            </Link>
          </div>
        </div>
      </header>

      <main className="flex-1">
        {/* Hero Section */}
        <section className="w-full py-6 md:py-12 lg:py-16 xl:py-24 bg-gradient-to-b from-[#070e21] to-[#0f1729]">
          <div className="container px-4 md:px-6">
            <div className="flex flex-col items-center text-center max-w-3xl mx-auto mb-12">
              <Image
                src="https://hebbkx1anhila5yf.public.blob.vercel-storage.com/4601249a-0e85-4811-b389-94f36d492a50_removalai_preview-1-700HdtUgdYNtVLtz2mhpgOqQZ7811y.png"
                alt="git_freedom hero"
                width={200}
                height={200}
                className="mb-8"
              />
              <h1 className="text-4xl font-bold tracking-tighter sm:text-6xl xl:text-7xl/none text-cyan-400 mb-6">
                Community Driven and Community Hosted
              </h1>
              <p className="max-w-[800px] text-cyan-100 text-lg md:text-xl mb-8">
                Giving Web3 wings to soar beyond traditional boundaries. Join us in revolutionizing how open source
                software is stored, shared, and maintained through the power of decentralized networks.
              </p>
              <div className="flex flex-col gap-4 min-[400px]:flex-row justify-center">
              <Link href={"/repositoriess"}>
                <Button size="lg" className="bg-cyan-500 hover:bg-cyan-600 text-[#070e21] gap-2">
                  Launch App
                  <ArrowRight className="h-4 w-4" />
                </Button>             
              </Link>

                <Button size="lg" variant="outline" className="border-cyan-700 text-cyan-400 hover:bg-cyan-950">
                  View Documentation
                </Button>
              </div>
            </div>
          </div>
        </section>

        {/* Features Section */}
        <section id="features" className="w-full py-12 md:py-24 lg:py-32 bg-[#0f1729]">
          <div className="container px-4 md:px-6">
            <div className="flex flex-col items-center justify-center space-y-4 text-center">
              <div className="space-y-2">
                <h2 className="text-3xl font-bold tracking-tighter md:text-4xl text-cyan-400">
                  Revolutionary Features
                </h2>
                <p className="max-w-[900px] text-cyan-100 md:text-xl">
                  Empowering developers with next-generation collaboration tools.
                </p>
              </div>
            </div>
            <div className="mx-auto grid max-w-5xl items-center gap-6 py-12 lg:grid-cols-2">
              <div className="flex flex-col items-center space-y-4 text-center">
                <div className="flex h-16 w-16 items-center justify-center rounded-full bg-cyan-500/10">
                  <Network className="h-8 w-8 text-cyan-500" />
                </div>
                <h3 className="text-xl font-bold text-cyan-400">Peer-to-Peer Repository Sharing</h3>
                <p className="text-cyan-100">
                  Clone and share repositories directly through secure peer-to-peer connections. Eliminate central
                  server dependencies and enhance collaboration with distributed networking.
                </p>
              </div>
              <div className="flex flex-col items-center space-y-4 text-center">
                <div className="flex h-16 w-16 items-center justify-center rounded-full bg-cyan-500/10">
                  <FileCode2 className="h-8 w-8 text-cyan-500" />
                </div>
                <h3 className="text-xl font-bold text-cyan-400">Smart Contract Integration</h3>
                <p className="text-cyan-100">
                  Store repository metadata securely on the blockchain through smart contracts. Enable transparent,
                  immutable record-keeping and collaborative governance of your projects.
                </p>
              </div>
            </div>
          </div>
        </section>

        {/* CTA Section */}
        <section className="w-full py-12 md:py-24 lg:py-32 bg-gradient-to-b from-[#0f1729] to-[#070e21]">
          <div className="container px-4 md:px-6">
            <div className="flex flex-col items-center justify-center space-y-4 text-center">
              <div className="space-y-2">
                <h2 className="text-3xl font-bold tracking-tighter md:text-4xl text-cyan-400">Join the Revolution</h2>
                <p className="max-w-[600px] text-cyan-100 md:text-xl">
                  Be part of the movement that's reshaping the future of open source collaboration.
                </p>
              </div>
              <div className="flex flex-col gap-2 min-[400px]:flex-row">
              <Link href={"/repositoriess"}>
                <Button size="lg" className="bg-cyan-500 hover:bg-cyan-600 text-[#070e21]">
                  Launch App
                </Button>              
              </Link>

                <Button size="lg" variant="outline" className="border-cyan-700 text-cyan-400 hover:bg-cyan-950">
                  Join Community
                </Button>
              </div>
            </div>
          </div>
        </section>
      </main>

      {/* Enhanced Footer */}
      <footer className="w-full border-t border-cyan-900/50 py-12 bg-[#070e21] mb-7">
        <div className="container px-4 md:px-6">
          {/* Brand Section */}
          <div className="flex flex-col items-center text-center space-y-4 mb-12">
            <div className="flex items-center gap-2">
              <Image
                src="https://hebbkx1anhila5yf.public.blob.vercel-storage.com/4601249a-0e85-4811-b389-94f36d492a50_removalai_preview-1-700HdtUgdYNtVLtz2mhpgOqQZ7811y.png"
                alt="git_freedom logo"
                width={32}
                height={32}
                className="h-8 w-8"
              />
              <span className="text-xl font-bold text-cyan-400">&lt;git_freedom&gt;</span>
            </div>
            <p className="text-sm text-cyan-100 max-w-md">
              Revolutionizing open source collaboration through decentralized networks and blockchain technology.
            </p>
          </div>

          {/* Navigation Sections */}
          <div className="flex flex-col md:flex-row justify-center gap-16 mb-12">
            {/* Product Column */}
            <div className="flex flex-col items-center">
              <h3 className="text-lg font-semibold text-cyan-400 mb-4">Product</h3>
              <ul className="space-y-2 text-center">
                <li>
                  <Link href="#" className="text-cyan-100 hover:text-cyan-400 text-sm">
                    Documentation
                  </Link>
                </li>
                <li>
                  <Link href="#" className="text-cyan-100 hover:text-cyan-400 text-sm">
                    Getting Started
                  </Link>
                </li>
                <li>
                  <Link href="#" className="text-cyan-100 hover:text-cyan-400 text-sm">
                    API Reference
                  </Link>
                </li>
                <li>
                  <Link href="#" className="text-cyan-100 hover:text-cyan-400 text-sm">
                    Examples
                  </Link>
                </li>
              </ul>
            </div>

            {/* Community Column */}
            <div className="flex flex-col items-center">
              <h3 className="text-lg font-semibold text-cyan-400 mb-4">Community</h3>
              <ul className="space-y-2 text-center">
                <li>
                  <Link href="#" className="text-cyan-100 hover:text-cyan-400 text-sm inline-flex items-center gap-2">
                    <MessageSquare className="h-4 w-4" />
                    Discord
                  </Link>
                </li>
                <li>
                  <Link href="#" className="text-cyan-100 hover:text-cyan-400 text-sm inline-flex items-center gap-2">
                    <Telegram className="h-4 w-4" />
                    Telegram
                  </Link>
                </li>
                <li>
                  <Link href="#" className="text-cyan-100 hover:text-cyan-400 text-sm inline-flex items-center gap-2">
                    <Github className="h-4 w-4" />
                    GitHub
                  </Link>
                </li>
                <li>
                  <Link href="#" className="text-cyan-100 hover:text-cyan-400 text-sm inline-flex items-center gap-2">
                    <Twitter className="h-4 w-4" />
                    Twitter
                  </Link>
                </li>
              </ul>
            </div>

            {/* Resources Column */}
            <div className="flex flex-col items-center">
              <h3 className="text-lg font-semibold text-cyan-400 mb-4">Resources</h3>
              <ul className="space-y-2 text-center">
                <li>
                  <Link href="#" className="text-cyan-100 hover:text-cyan-400 text-sm inline-flex items-center gap-2">
                    <Youtube className="h-4 w-4" />
                    Tutorials
                  </Link>
                </li>
                <li>
                  <Link href="#" className="text-cyan-100 hover:text-cyan-400 text-sm inline-flex items-center gap-2">
                    <Medium className="h-4 w-4" />
                    Blog
                  </Link>
                </li>
                <li>
                  <Link href="#" className="text-cyan-100 hover:text-cyan-400 text-sm inline-flex items-center gap-2">
                    <Mail className="h-4 w-4" />
                    Newsletter
                  </Link>
                </li>
              </ul>
            </div>
          </div>

          {/* Bottom Footer */}
          <div className="pt-8 border-t border-cyan-900/50">
            <div className="flex flex-col md:flex-row justify-between items-center gap-4">
              <div className="text-sm text-cyan-100">
                © {new Date().getFullYear()} git_freedom. All rights reserved.
              </div>
              <div className="flex gap-4">
                <Link href="#" className="text-xs text-cyan-100 hover:text-cyan-400">
                  Privacy Policy
                </Link>
                <Link href="#" className="text-xs text-cyan-100 hover:text-cyan-400">
                  Terms of Service
                </Link>
                <Link href="#" className="text-xs text-cyan-100 hover:text-cyan-400">
                  Code of Conduct
                </Link>
              </div>
            </div>
          </div>
        </div>
      </footer>
    </div>
  )
}