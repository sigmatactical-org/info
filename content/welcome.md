---
title: Welcome
order: 1
---

Sigma Info publishes Markdown from a content repository. Edit files under `content/` in this repo (or point `INFO_CONTENT_REPO` at a dedicated one).

## How it works

1. Authors commit `.md` files with optional YAML front matter.
2. The service fetches documents from GitHub at request time and caches them for an hour.
3. Pages are served with shared Sigma theme assets.
