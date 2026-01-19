# docs

Next.js 16 documentation site using Fumadocs. Static export to GitHub Pages.

See `docs/AGENTS.md` for repo-specific notes and conventions.

## Development

```bash
cd docs
bun install
bun run dev
```

Open http://localhost:3000 to view the site.

## Content

- Docs pages live in `content/docs/` (MDX).
- Navigation lives in `content/docs/meta.json`.

## Build

```bash
cd docs
bun run build
```
