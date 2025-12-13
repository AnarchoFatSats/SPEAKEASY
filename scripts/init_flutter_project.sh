#!/usr/bin/env bash
set -euo pipefail

if ! command -v flutter >/dev/null 2>&1; then
  echo "flutter not found. Install Flutter SDK first."
  exit 1
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

if [ -d mobile ]; then
  echo "mobile/ already exists. Remove it or copy templates manually."
  exit 1
fi

flutter create mobile

echo "Copying templates..."
rsync -av --delete mobile_template/ mobile/

echo "Done. Open mobile/ in your IDE."
