---
title: Getting started
order: 2
---

## Add a document

Create `content/my-topic.md`:

```markdown
---
title: My topic
order: 10
---

Your content here.
```

Rebuild or redeploy to publish. The page appears at `/doc/my-topic`.

## Front matter

| Field | Purpose |
|-------|---------|
| `title` | Page heading and navigation label (defaults to slug) |
| `order` | Sort key on the index (lower first) |
