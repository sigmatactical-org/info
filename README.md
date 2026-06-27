# sigma-info

Markdown-fed information sharing site for Sigma Tactical Group. Documents live in **`content/`** in this repository (or synced from another repo before build), are embedded at compile time, and served with shared [sigma-theme](https://github.com/sigmatactical-org/sigma-theme) chrome.

Repository: https://github.com/sigmatactical-org/info

## Content

Add `.md` files under `content/` with optional YAML front matter:

```markdown
---
title: Policy overview
order: 10
---

Body in **Markdown**.
```

| Field | Purpose |
|-------|---------|
| `title` | Page title and nav label (default: slug) |
| `order` | Index sort order (lower first) |

Pages are served at `/doc/{slug}` (e.g. `/doc/welcome`). The index lists all documents.

### Separate content repo

Point CI or local builds at another repository:

```bash
export INFO_CONTENT_REPO=git@github.com:sigmatactical-org/sigma-info-content.git
./scripts/prepare-local.sh   # clones into content-repo/ and rsyncs to content/
```

Or place a checkout at `../sigma-info-content` (rsynced automatically).

## Development

```bash
./scripts/prepare-local.sh
cargo run -p sigma-info
```

Open http://localhost:8080

## Docker

Release is in **`.github/workflows/release.yml`** when configured. Locally:

```bash
./scripts/docker-build.sh
docker build -f Dockerfile build/image
```

Runtime image is Debian 13 distroless; Markdown is baked into the binary (no runtime volume required).

## License

MIT OR Apache-2.0
