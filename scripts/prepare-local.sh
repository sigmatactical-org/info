#!/usr/bin/env bash
# Link theme, sync Markdown content repo, and patch the git dependency for local/CI builds.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

dir="$ROOT"
THEME_HELPER=""
while [[ "$dir" != "/" ]]; do
  if [[ -f "$dir/scripts/resolve-theme.sh" ]]; then
    THEME_HELPER="$dir/scripts/resolve-theme.sh"
    break
  fi
  dir="$(dirname "$dir")"
done

if [[ -n "$THEME_HELPER" ]]; then
  # shellcheck source=/dev/null
  source "$THEME_HELPER"
  prepare_sigma_theme "$ROOT"
else
  THEME_PATH="theme"
  if [[ -d ../theme/ts ]]; then
    THEME_PATH="../theme"
  elif [[ ! -d theme/ts ]]; then
    git clone --depth 1 https://github.com/sigmatactical-org/sigma-theme.git theme
  fi
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

if [[ -n "$THEME_HELPER" ]]; then
  write_theme_patch_files "$ROOT"
  write_askama_config "$ROOT"
  build_theme_ts "$ROOT"
else
  mkdir -p .cargo
  cat >.cargo/config.toml <<EOF
[patch."https://github.com/sigmatactical-org/sigma-theme.git"]
sigma-theme = { path = "$THEME_PATH" }
EOF
  cat >askama.toml <<EOF
[general]
dirs = ["templates", "$THEME_PATH/assets/templates"]
EOF
  (cd "$THEME_PATH/ts" && npm ci && npm run check && npm run build)
fi

echo "sigma-theme ($ROOT/${THEME_PATH:-theme}) and content ($CONTENT_DIR) ready for cargo build."
