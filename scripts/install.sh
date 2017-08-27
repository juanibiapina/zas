#!/usr/bin/env bash

set -e

# Setup variables

FIREWALL_PLIST_PATH="/Library/LaunchDaemons/com.zas.firewall.plist"
ARCHIVE_URL="https://github.com/juanibiapina/zas/releases/download/v${VERSION}/zas-v${VERSION}-x86_64-apple-darwin.tar.gz"

# Install DNS resolver

echo "Installing DNS resolver"
sudo mkdir -p /etc/resolver/
sudo cp -f "resources/dev-resolver" /etc/resolver/dev

# Install port forwarding

echo "Installing port forward"
sudo cp "resources/com.zas.firewall.plist" "${FIREWALL_PLIST_PATH}"

sudo launchctl bootstrap system "${FIREWALL_PLIST_PATH}" 2>/dev/null
sudo launchctl enable system/com.zas.firewall 2>/dev/null
sudo launchctl kickstart -k system/com.zas.firewall 2>/dev/null
