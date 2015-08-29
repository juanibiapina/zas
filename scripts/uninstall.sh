#!/usr/bin/env bash

set -e

ZAS_ROOT="$HOME/Library/Application Support/Zas"

ZASD_PLIST_PATH="$HOME/Library/LaunchAgents/com.zas.zasd.plist"

if [ ! -d "$ZAS_ROOT" ]; then
  echo "error: zas is not installed"
  exit 1
fi

rm -rf "$ZAS_ROOT"

launchctl unload "$ZASD_PLIST_PATH" 2>/dev/null || true
rm -f "$ZASD_PLIST_PATH"
