# sigma-info

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV](https://img.shields.io/badge/MSRV-1.97.0-blue.svg)](https://www.rust-lang.org)

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

SIGMA-RACER build specifications are served at `/products/sigma-racer` with tabbed markdown fetched from [racer](https://github.com/sigmatactical-org/racer) (GitHub API, cached 30 minutes per instance). The storefront product page links here via **Details**.

| Variable | Purpose |
|----------|---------|
| `INFO_STORE_PUBLIC_URL` | Store base URL for the “Back to product” link (default `http://127.0.0.1:8082/`) |
| `INFO_RACER_SPECS_REPO` | GitHub repo for specs (`owner/name`, default `sigmatactical-org/racer`) |
| `INFO_RACER_SPECS_REF` | Git ref for racer specs (default `main`) |
| `INFO_RACER_SPECS_CACHE_TTL_SECS` | Cache TTL in seconds (default `1800`) |

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

## Brand & artwork

© Sigma Tactical Group. **All rights reserved.**

The Sigma Tactical Group name, logos, marks, artwork, and visual identity are **proprietary**. They are not covered by this repository's source-code license. See [BRANDING.md](BRANDING.md).

## License

MIT OR Apache-2.0 for **source code** only. Branding remains proprietary.
