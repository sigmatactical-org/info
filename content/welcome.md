---
title: Welcome
order: 1
---

Sigma Info publishes Markdown from a content repository. Edit files under `content/` (or sync from a dedicated repo before `cargo build`).

## How it works

1. Authors commit `.md` files with optional YAML front matter.
2. CI checks out the content repo and builds the site.
3. Pages are embedded in the binary and served with shared Sigma theme assets.
