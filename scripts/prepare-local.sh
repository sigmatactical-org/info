#!/usr/bin/env bash
# Link theme, sync Markdown content repo, and patch the git dependency for local/CI builds.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

THEME_PATH="theme"
if [[ -d ../theme/ts ]]; then
  THEME_PATH="../theme"
elif [[ ! -d theme/ts ]]; then
  git clone --depth 1 https://github.com/sigmatactical-org/sigma-theme.git theme
fi

CONTENT_DIR="content"
if [[ -n "${INFO_CONTENT_REPO:-}" ]]; then
  if [[ ! -d content-repo ]]; then
    git clone --depth 1 "$INFO_CONTENT_REPO" content-repo
  else
    git -C content-repo pull --ff-only
  fi
  rsync -a --delete content-repo/ "$CONTENT_DIR/"
elif [[ -d ../sigma-info-content ]]; then
  rsync -a --delete ../sigma-info-content/ "$CONTENT_DIR/"
fi

mkdir -p .cargo
cat > .cargo/config.toml <<EOF
[patch."https://github.com/sigmatactical-org/sigma-theme.git"]
sigma-theme = { path = "$THEME_PATH" }
EOF

cat > askama.toml <<EOF
[general]
dirs = ["templates", "$THEME_PATH/assets/templates"]
EOF

(cd "$THEME_PATH/ts" && npm ci && npm run check && npm run build)

echo "sigma-theme ($THEME_PATH) and content ($CONTENT_DIR) ready for cargo build."
