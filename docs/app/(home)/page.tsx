'use client';

import Link from 'next/link';
import { useState } from 'react';
import {
  GitBranch,
  Database,
  WifiOff,
  Network,
  BookOpen,
  Map,
  ArrowRight,
  Github,
  Bot,
  Code2,
  Users,
  Terminal,
  Search,
  Share2,
  Sparkles,
  FileText,
  Cpu,
  ExternalLink
} from 'lucide-react';

export default function HomePage() {
  return (
    <div className="flex flex-col min-h-screen">
      {/* Hero Section */}
      <section className="relative flex flex-col items-center justify-center py-20 text-center lg:py-28 overflow-hidden">
        {/* Enhanced background effects */}
        <div className="absolute inset-0 -z-10">
          <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-sky-900/30 via-transparent to-transparent"></div>
          <div className="absolute inset-0 bg-[radial-gradient(ellipse_at_bottom_right,_var(--tw-gradient-stops))] from-indigo-900/20 via-transparent to-transparent"></div>
          {/* Animated grid */}
          <div className="absolute inset-0 bg-[linear-gradient(rgba(14,165,233,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(14,165,233,0.03)_1px,transparent_1px)] bg-[size:60px_60px] [mask-image:radial-gradient(ellipse_at_center,black_20%,transparent_70%)]"></div>
        </div>

        <div className="inline-flex items-center gap-2 px-4 py-1.5 mb-6 text-sm font-medium rounded-full bg-sky-500/10 text-sky-400 border border-sky-500/20 animate-in fade-in slide-in-from-bottom-4 duration-500">
          <span className="relative flex h-2 w-2">
            <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-sky-400 opacity-75"></span>
            <span className="relative inline-flex rounded-full h-2 w-2 bg-sky-500"></span>
          </span>
          AI-Native Developer Tools
        </div>

        <h1 className="text-4xl font-extrabold tracking-tight sm:text-6xl md:text-7xl mb-6 max-w-4xl mx-auto px-4 animate-in fade-in slide-in-from-bottom-8 duration-700 delay-100">
          Tools that <span className="text-gradient">Remember</span>
          <br /> for the Agentic Era
        </h1>

        <p className="max-w-2xl mx-auto text-lg text-slate-400 mb-10 px-4 animate-in fade-in slide-in-from-bottom-8 duration-700 delay-200">
          Give your AI agents persistent, structured memory. Graph traversal meets vector search, powered by HelixDB.
        </p>

        <div className="flex flex-col sm:flex-row gap-4 mb-16 animate-in fade-in slide-in-from-bottom-8 duration-700 delay-300">
          <Link
            href="/docs/getting-started"
            className="inline-flex items-center justify-center px-8 py-3 text-base font-medium text-white transition-all bg-sky-500 rounded-lg hover:bg-sky-400 hover:shadow-[0_0_30px_-5px_rgba(14,165,233,0.5)] active:scale-95"
          >
            Get Started
            <ArrowRight className="ml-2 h-4 w-4" />
          </Link>
          <a
            href="https://github.com/kevinmichaelchen/ixchel"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center justify-center px-8 py-3 text-base font-medium text-slate-300 transition-all bg-white/5 border border-white/10 rounded-lg hover:bg-white/10 hover:text-white active:scale-95"
          >
            <Github className="mr-2 h-4 w-4" />
            View on GitHub
          </a>
        </div>

        {/* Terminal Preview */}
        <div className="w-full max-w-3xl mx-auto px-4 animate-in fade-in slide-in-from-bottom-12 duration-1000 delay-500">
          <TerminalPreview />
        </div>
      </section>

      {/* Use Case Tabs Section */}
      <section className="px-4 py-20 mx-auto max-w-6xl sm:px-6 lg:px-8">
        <div className="text-center mb-12">
          <h2 className="text-3xl font-bold text-slate-100 mb-4">Built for Everyone</h2>
          <p className="text-slate-400 max-w-2xl mx-auto">Whether you're an AI agent, a developer, or part of a team—Ixchel adapts to your workflow.</p>
        </div>
        <UseCaseTabs />
      </section>

      {/* Architecture Diagram */}
      <section className="px-4 py-20 mx-auto max-w-6xl sm:px-6 lg:px-8">
        <div className="text-center mb-12">
          <h2 className="text-3xl font-bold text-slate-100 mb-4">How It Works</h2>
          <p className="text-slate-400 max-w-2xl mx-auto">A simple architecture that keeps your data in git while enabling powerful queries.</p>
        </div>
        <ArchitectureDiagram />
      </section>

      {/* Core Principles */}
      <section className="px-4 py-20 mx-auto max-w-7xl sm:px-6 lg:px-8">
        <h2 className="text-2xl font-bold text-center mb-12 text-slate-200">Core Principles</h2>
        <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-4">
          <FeatureCard
            icon={<GitBranch className="h-6 w-6 text-sky-400" />}
            title="Git-First"
            description="Data stored in human-readable Markdown, YAML, and JSONL. Your repo is the source of truth."
          />
          <FeatureCard
            icon={<WifiOff className="h-6 w-6 text-indigo-400" />}
            title="Offline-First"
            description="Full functionality without network. Local embeddings via fastembed means no cloud dependencies."
          />
          <FeatureCard
            icon={<Network className="h-6 w-6 text-purple-400" />}
            title="Graph Intelligence"
            description="Traverse relationships between entities. Navigate your codebase structure naturally."
          />
          <FeatureCard
            icon={<Database className="h-6 w-6 text-emerald-400" />}
            title="Hybrid Search"
            description="Combine vector semantic search with traditional BM25 keyword matching for best results."
          />
        </div>
      </section>

      {/* Quick Start */}
      <section className="px-4 py-20 mx-auto max-w-4xl sm:px-6 lg:px-8">
        <div className="glow-card">
          <div className="flex items-center gap-3 mb-6">
            <div className="p-2 bg-sky-500/10 rounded-lg">
              <Terminal className="h-5 w-5 text-sky-400" />
            </div>
            <h2 className="text-2xl font-bold text-slate-100">Quick Start</h2>
          </div>
          <div className="bg-slate-950 rounded-lg border border-white/10 overflow-hidden">
            <div className="flex items-center gap-2 px-4 py-2 bg-white/5 border-b border-white/10">
              <div className="w-3 h-3 rounded-full bg-red-500/80"></div>
              <div className="w-3 h-3 rounded-full bg-yellow-500/80"></div>
              <div className="w-3 h-3 rounded-full bg-green-500/80"></div>
              <span className="ml-2 text-xs text-slate-500 font-mono">terminal</span>
            </div>
            <pre className="p-4 text-sm font-mono overflow-x-auto">
              <code>
                <span className="text-slate-500"># Install</span>{'\n'}
                <span className="text-emerald-400">cargo</span> <span className="text-slate-300">install --path apps/ix-cli</span>{'\n\n'}
                <span className="text-slate-500"># Initialize in your project</span>{'\n'}
                <span className="text-emerald-400">ixchel</span> <span className="text-slate-300">init</span>{'\n\n'}
                <span className="text-slate-500"># Create an issue</span>{'\n'}
                <span className="text-emerald-400">ixchel</span> <span className="text-slate-300">create issue</span> <span className="text-amber-300">"Add user authentication"</span>{'\n\n'}
                <span className="text-slate-500"># Search semantically</span>{'\n'}
                <span className="text-emerald-400">ixchel</span> <span className="text-slate-300">search</span> <span className="text-amber-300">"how to handle auth"</span>
              </code>
            </pre>
          </div>
          <div className="mt-6 flex justify-end">
            <Link
              href="/docs/getting-started"
              className="inline-flex items-center text-sm text-sky-400 hover:text-sky-300 transition-colors"
            >
              Full installation guide
              <ArrowRight className="ml-1 h-4 w-4" />
            </Link>
          </div>
        </div>
      </section>

      {/* The Toolkit */}
      <section className="px-4 py-20 mx-auto max-w-7xl sm:px-6 lg:px-8">
        <div className="flex flex-col md:flex-row items-end justify-between mb-8 gap-4">
          <div>
            <h2 className="text-3xl font-bold text-slate-100">The Toolkit</h2>
            <p className="mt-2 text-slate-400">Specialized tools composing via standard interfaces</p>
          </div>
        </div>

        <div className="grid grid-cols-1 gap-6 md:grid-cols-2">
          <ToolCard
            icon={<BookOpen className="h-8 w-8" />}
            name="ixchel"
            tag="MVP"
            tagColor="bg-sky-500/10 text-sky-400 border-sky-500/20"
            description="Git-first knowledge weaving system. Canonical Markdown artifacts + semantic search over your project memory."
            href="/docs/tools/ixchel"
          />
          <ToolCard
            icon={<Map className="h-8 w-8" />}
            name="ideas"
            tag="Planned"
            tagColor="bg-slate-500/10 text-slate-400 border-slate-500/20"
            description="Future tools and experiments are tracked as Ixchel ideas under .ixchel/ideas/ (git-first Markdown)."
          />
        </div>
      </section>

      {/* Inspirations Section */}
      <section className="px-4 py-20 mx-auto max-w-6xl sm:px-6 lg:px-8 border-t border-white/5">
        <div className="text-center mb-12">
          <h2 className="text-2xl font-bold text-slate-100 mb-4">Standing on Giants' Shoulders</h2>
          <p className="text-slate-400">Inspired by brilliant projects in the AI tooling ecosystem</p>
        </div>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <InspirationCard name="beads" author="Steve Yegge" color="#ff6b35" />
          <InspirationCard name="swarm-tools" author="Joel Hooks" color="#f7931e" />
          <InspirationCard name="dots" author="Joel Reymont" color="#fdb833" />
          <InspirationCard name=".context" author="Andre Figueira" color="#c1272d" />
        </div>
        <div className="text-center mt-8">
          <Link
            href="/docs/acknowledgments"
            className="inline-flex items-center text-sm text-slate-400 hover:text-sky-400 transition-colors"
          >
            See all acknowledgments
            <ArrowRight className="ml-1 h-4 w-4" />
          </Link>
        </div>
      </section>

      <footer className="mt-auto py-12 border-t border-white/5 bg-black/20 text-center">
        <p className="text-slate-500 text-sm">
          Built with <span className="text-sky-500">HelixDB</span> & Next.js
        </p>
      </footer>
    </div>
  );
}

function TerminalPreview() {
  return (
    <div className="bg-slate-950/80 backdrop-blur-sm rounded-xl border border-white/10 shadow-2xl shadow-sky-500/10 overflow-hidden">
      {/* Terminal header */}
      <div className="flex items-center justify-between px-4 py-3 bg-white/5 border-b border-white/10">
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded-full bg-red-500/80"></div>
          <div className="w-3 h-3 rounded-full bg-yellow-500/80"></div>
          <div className="w-3 h-3 rounded-full bg-green-500/80"></div>
        </div>
        <span className="text-xs text-slate-500 font-mono">~/my-project</span>
        <div className="w-16"></div>
      </div>

      {/* Terminal content */}
      <div className="p-4 font-mono text-sm space-y-3">
        <div className="flex items-start gap-2">
          <span className="text-sky-400">$</span>
          <span className="text-slate-300">ixchel search <span className="text-amber-300">"authentication flow"</span></span>
        </div>

        <div className="pl-4 text-slate-400 text-xs space-y-2 border-l-2 border-sky-500/30 ml-1">
          <div className="flex items-center gap-2">
            <Search className="h-3 w-3 text-sky-400" />
            <span>Searching with hybrid (vector + BM25)...</span>
          </div>
          <div className="space-y-1">
            <div className="flex items-center gap-2 text-slate-300">
              <FileText className="h-3 w-3 text-emerald-400" />
              <span className="text-emerald-400">iss-a1b2c3</span>
              <span className="text-slate-500">•</span>
              <span>Add OAuth2 authentication</span>
              <span className="text-sky-400 ml-auto">0.94</span>
            </div>
            <div className="flex items-center gap-2 text-slate-300">
              <FileText className="h-3 w-3 text-emerald-400" />
              <span className="text-emerald-400">iss-d4e5f6</span>
              <span className="text-slate-500">•</span>
              <span>Implement JWT refresh tokens</span>
              <span className="text-sky-400 ml-auto">0.87</span>
            </div>
            <div className="flex items-center gap-2 text-slate-300">
              <FileText className="h-3 w-3 text-emerald-400" />
              <span className="text-emerald-400">iss-g7h8i9</span>
              <span className="text-slate-500">•</span>
              <span>Session management refactor</span>
              <span className="text-sky-400 ml-auto">0.82</span>
            </div>
          </div>
        </div>

        <div className="flex items-start gap-2 pt-2">
          <span className="text-sky-400">$</span>
          <span className="text-slate-300">ixchel graph <span className="text-emerald-400">iss-a1b2c3</span></span>
        </div>

        <div className="pl-4 text-slate-400 text-xs border-l-2 border-purple-500/30 ml-1">
          <div className="flex items-center gap-2 mb-1">
            <Share2 className="h-3 w-3 text-purple-400" />
            <span>Dependency graph for iss-a1b2c3</span>
          </div>
          <pre className="text-[10px] leading-relaxed text-slate-500">
{`  iss-a1b2c3 (Add OAuth2 authentication)
  ├── blocks → iss-d4e5f6 (JWT refresh tokens)
  └── related → iss-g7h8i9 (Session management)`}
          </pre>
        </div>

        <div className="flex items-start gap-2 pt-2">
          <span className="text-sky-400">$</span>
          <span className="text-slate-400 animate-pulse">▊</span>
        </div>
      </div>
    </div>
  );
}

function UseCaseTabs() {
  const [activeTab, setActiveTab] = useState<'agents' | 'developers' | 'teams'>('agents');

  const tabs = [
    { id: 'agents' as const, label: 'AI Agents', icon: Bot },
    { id: 'developers' as const, label: 'Developers', icon: Code2 },
    { id: 'teams' as const, label: 'Teams', icon: Users },
  ];

  const content = {
    agents: {
      title: 'Context-aware AI assistants',
      description: 'Give your AI agents the memory they deserve. Structured JSON output, semantic search, and full context retrieval.',
      features: [
        { icon: Terminal, text: '--json output on all commands' },
        { icon: Search, text: 'Semantic search for intelligent retrieval' },
        { icon: Sparkles, text: 'Token-aware context with ixchel context <id>' },
      ],
      code: `$ ixchel context iss-a1b2c3 --json
{
  "id": "iss-a1b2c3",
  "title": "Add OAuth2 authentication",
  "status": "in_progress",
  "blocks": ["iss-d4e5f6"],
  "related": ["iss-g7h8i9"],
  "embedding_similarity": 0.94
}`,
    },
    developers: {
      title: 'CLI-first workflow',
      description: 'Fast, scriptable commands that integrate with your existing tools. Pipes, scripts, and UNIX philosophy.',
      features: [
        { icon: Terminal, text: 'Composable CLI commands' },
        { icon: GitBranch, text: 'Git-native storage (Markdown files)' },
        { icon: Database, text: 'Local HelixDB for fast queries' },
      ],
      code: `$ ixchel create issue "Add caching layer" \\
    --status open --priority high

Created: iss-x1y2z3

$ ixchel link iss-x1y2z3 blocks iss-a1b2c3
Linked: iss-x1y2z3 blocks iss-a1b2c3`,
    },
    teams: {
      title: 'Collaborate via git',
      description: 'No separate service to sync. Issues merge like code. Branch, PR, and resolve conflicts naturally.',
      features: [
        { icon: GitBranch, text: 'Issues travel with your code' },
        { icon: Users, text: 'Standard git merge workflow' },
        { icon: WifiOff, text: 'Works offline, syncs when ready' },
      ],
      code: `$ git pull origin main
Updating issues...

$ ixchel sync
Indexed 3 new issues
Updated 2 existing issues
Regenerated 5 embeddings`,
    },
  };

  const current = content[activeTab];

  return (
    <div className="glow-card">
      {/* Tab buttons */}
      <div className="flex flex-wrap gap-2 mb-8 p-1 bg-white/5 rounded-lg w-fit">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={`flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-all ${
              activeTab === tab.id
                ? 'bg-sky-500 text-white shadow-lg shadow-sky-500/25'
                : 'text-slate-400 hover:text-slate-200 hover:bg-white/5'
            }`}
          >
            <tab.icon className="h-4 w-4" />
            {tab.label}
          </button>
        ))}
      </div>

      {/* Tab content */}
      <div className="grid md:grid-cols-2 gap-8">
        <div>
          <h3 className="text-xl font-bold text-slate-100 mb-3">{current.title}</h3>
          <p className="text-slate-400 mb-6">{current.description}</p>
          <ul className="space-y-3">
            {current.features.map((feature, i) => (
              <li key={i} className="flex items-center gap-3 text-slate-300">
                <div className="p-1.5 bg-sky-500/10 rounded">
                  <feature.icon className="h-4 w-4 text-sky-400" />
                </div>
                {feature.text}
              </li>
            ))}
          </ul>
        </div>
        <div className="bg-slate-950 rounded-lg border border-white/10 overflow-hidden">
          <div className="flex items-center gap-2 px-4 py-2 bg-white/5 border-b border-white/10">
            <div className="w-2.5 h-2.5 rounded-full bg-red-500/80"></div>
            <div className="w-2.5 h-2.5 rounded-full bg-yellow-500/80"></div>
            <div className="w-2.5 h-2.5 rounded-full bg-green-500/80"></div>
          </div>
          <pre className="p-4 text-xs font-mono text-slate-300 overflow-x-auto">
            {current.code}
          </pre>
        </div>
      </div>
    </div>
  );
}

function ArchitectureDiagram() {
  return (
    <div className="glow-card">
      <div className="flex flex-col lg:flex-row items-center justify-between gap-8">
        {/* Source */}
        <div className="flex-1 text-center">
          <div className="inline-flex p-4 bg-emerald-500/10 rounded-2xl mb-4 border border-emerald-500/20">
            <FileText className="h-10 w-10 text-emerald-400" />
          </div>
          <h3 className="text-lg font-semibold text-slate-100 mb-2">Markdown Files</h3>
          <p className="text-sm text-slate-400">Human-readable, git-tracked</p>
          <code className="text-xs text-emerald-400 mt-2 block">.ixchel/issues/*.md</code>
        </div>

        {/* Arrow */}
        <div className="flex items-center text-slate-600">
          <div className="hidden lg:block w-16 h-px bg-gradient-to-r from-emerald-500/50 to-sky-500/50"></div>
          <ArrowRight className="h-6 w-6 text-slate-500 mx-2" />
          <div className="hidden lg:block w-16 h-px bg-gradient-to-r from-sky-500/50 to-purple-500/50"></div>
        </div>

        {/* Index */}
        <div className="flex-1 text-center">
          <div className="inline-flex p-4 bg-sky-500/10 rounded-2xl mb-4 border border-sky-500/20">
            <Cpu className="h-10 w-10 text-sky-400" />
          </div>
          <h3 className="text-lg font-semibold text-slate-100 mb-2">HelixDB Index</h3>
          <p className="text-sm text-slate-400">Graph + Vector + BM25</p>
          <code className="text-xs text-sky-400 mt-2 block">Rebuilds from source</code>
        </div>

        {/* Arrow */}
        <div className="flex items-center text-slate-600">
          <div className="hidden lg:block w-16 h-px bg-gradient-to-r from-sky-500/50 to-purple-500/50"></div>
          <ArrowRight className="h-6 w-6 text-slate-500 mx-2" />
          <div className="hidden lg:block w-16 h-px bg-gradient-to-r from-purple-500/50 to-amber-500/50"></div>
        </div>

        {/* Query */}
        <div className="flex-1 text-center">
          <div className="inline-flex p-4 bg-purple-500/10 rounded-2xl mb-4 border border-purple-500/20">
            <Search className="h-10 w-10 text-purple-400" />
          </div>
          <h3 className="text-lg font-semibold text-slate-100 mb-2">Smart Queries</h3>
          <p className="text-sm text-slate-400">Semantic + keyword search</p>
          <code className="text-xs text-purple-400 mt-2 block">ixchel search "..."</code>
        </div>
      </div>

      <div className="mt-8 pt-6 border-t border-white/10 text-center">
        <p className="text-sm text-slate-500">
          <span className="text-sky-400">Delete the database?</span> No problem. Run <code className="text-emerald-400">ixchel sync</code> and it rebuilds from your Markdown files.
        </p>
      </div>
    </div>
  );
}

function FeatureCard({ icon, title, description }: { icon: React.ReactNode, title: string, description: string }) {
  return (
    <div className="glow-card h-full flex flex-col">
      <div className="mb-4 p-3 bg-white/5 rounded-lg w-fit border border-white/5">
        {icon}
      </div>
      <h3 className="text-lg font-semibold text-slate-100 mb-2">{title}</h3>
      <p className="text-slate-400 text-sm leading-relaxed">{description}</p>
    </div>
  );
}

function ToolCard({ icon, name, description, tag, tagColor, href }: { icon: React.ReactNode, name: string, description: string, tag: string, tagColor: string, href?: string }) {
  const content = (
    <>
      <div className="flex items-start justify-between mb-4">
        <div className="p-3 bg-sky-500/10 text-sky-400 rounded-lg group-hover:bg-sky-500 group-hover:text-white transition-colors duration-300">
          {icon}
        </div>
        <span className={`px-2.5 py-0.5 rounded-full text-xs font-medium border ${tagColor}`}>
          {tag}
        </span>
      </div>
      <h3 className="text-xl font-bold text-slate-100 mb-2 group-hover:text-sky-400 transition-colors">{name}</h3>
      <p className="text-slate-400">{description}</p>
    </>
  );

  if (href) {
    return (
      <Link href={href} className="glow-card group block">
        {content}
      </Link>
    );
  }

  return (
    <div className="glow-card group">
      {content}
    </div>
  );
}

function InspirationCard({ name, author, color }: { name: string, author: string, color: string }) {
  return (
    <div
      className="p-4 rounded-xl border border-white/10 bg-white/5 hover:bg-white/10 transition-all hover:border-opacity-50 group"
      style={{ '--accent': color } as React.CSSProperties}
    >
      <div
        className="w-2 h-2 rounded-full mb-3 group-hover:scale-125 transition-transform"
        style={{ backgroundColor: color }}
      />
      <p className="font-mono text-sm text-slate-200 mb-1">{name}</p>
      <p className="text-xs text-slate-500">{author}</p>
    </div>
  );
}
