# sigma-info

[![CI](https://github.com/sigmatactical-org/info/actions/workflows/ci.yml/badge.svg)](https://github.com/sigmatactical-org/info/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV](https://img.shields.io/badge/MSRV-1.97.0-blue.svg)](https://www.rust-lang.org)

Markdown-fed information sharing site for Sigma Tactical Group. Documents live in **`content/`** — read from GitHub at request time and cached in memory, so publishing a document does not require a rebuild — and are served with shared [sigma-theme](https://github.com/sigmatactical-org/sigma-theme) chrome.

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

Documents are fetched from the content repository's `content/` directory through
the GitHub API and cached for `INFO_CONTENT_CACHE_TTL_SECS`, so an edit appears
without a redeploy once the cache expires. `content/terms.md` is additionally
compiled into the binary and always merged into the listing, which keeps Terms
reachable when GitHub is unavailable or unreachable from the cluster; if a fetch
fails outright the site falls back to those built-ins rather than erroring.

SIGMA-RACER build specifications are served at `/products/sigma-racer` with tabbed markdown fetched from [sigma-racer-specs](https://github.com/sigmatactical-org/sigma-racer-specs) (GitHub API, cached 30 minutes per instance). The storefront product page links here via **Details**.

| Variable | Purpose |
|----------|---------|
| `INFO_STORE_PUBLIC_URL` | Store base URL for the “Back to product” link (default `http://127.0.0.1:8082/`) |
| `INFO_RACER_SPECS_REPO` | GitHub repo for specs (`owner/name`, default `sigmatactical-org/sigma-racer-specs`) |
| `INFO_RACER_SPECS_REF` | Git ref for racer specs (default `main`) |
| `INFO_RACER_SPECS_CACHE_TTL_SECS` | Cache TTL in seconds (default `1800`) |

### Separate content repo

Documents default to this repository's own `content/` on `main`. Point the
service at another repository, path, or ref instead:

| Variable | Purpose |
|----------|---------|
| `INFO_CONTENT_REPO` | Content repo as `owner/name` (default `sigmatactical-org/info`) |
| `INFO_CONTENT_PATH` | Directory within that repo holding the `.md` files (default `content`) |
| `INFO_CONTENT_REF` | Branch, tag, or commit to read (default `main`) |
| `INFO_CONTENT_CACHE_TTL_SECS` | How long a fetched listing is reused (default `3600`) |

```bash
export INFO_CONTENT_REPO=sigmatactical-org/sigma-info-content
cargo run -p sigma-info
```

## Development

```bash
cargo run -p sigma-info
```

Open http://localhost:8080

### Shared crates

`sigma-theme` and `sigma-pg` are pinned git dependencies, so a
fresh clone builds with nothing but `cargo`: the revision in `Cargo.toml` is
fetched, and `build.rs` writes the `askama.toml` that points at sigma-theme's
templates wherever Cargo put them.

When one of those crates is checked out beside this repo and you are editing it,
link the checkouts so your edits are picked up without a push:

```bash
./scripts/prepare-local.sh
```

That writes `[patch]` entries into `.cargo/config.toml` (gitignored) for the
crates it finds and leaves the rest on their pinned revision; it prints what it
linked. Undo by deleting the file. Note that building against a linked checkout
rewrites `Cargo.lock` into path form — don't commit that; `platform`'s
`scripts/relock.sh` restores the git-resolved lockfile CI expects.

Bumping a shared crate is `platform/scripts/pin-shared-revs.sh <crate>` after
that crate is pushed, which updates every consumer's pin at once.

## Docker

Release is in **`.github/workflows/release.yml`** when configured. Locally:

```bash
./scripts/docker-build.sh
docker build -f Dockerfile build/image
```

Runtime image is Debian 13 distroless. No runtime volume is required: documents are fetched from GitHub and cached in memory, with the built-in Terms page compiled in as a fallback. The container does need outbound access to the GitHub API.

## Brand & artwork

© Sigma Tactical Group. **All rights reserved.**

The Sigma Tactical Group name, logos, marks, artwork, and visual identity are **proprietary**. They are not covered by this repository's source-code license. See [BRANDING.md](BRANDING.md).

## License

MIT OR Apache-2.0 for **source code** only. Branding remains proprietary.
