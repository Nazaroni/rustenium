#!/usr/bin/env bash
# Downloads latest chromedriver (MacOS arm64, Apple Silicon) for local testing

set -e

if ! command -v wget &> /dev/null; then
  echo "wget not found. Install wget first."
  exit 1
fi

CHROME_DRIVER_VERSION=$(wget -qO- https://chromedriver.storage.googleapis.com/LATEST_RELEASE)
DRIVER_URL="https://edgedl.me.gvt1.com/edgedl/chrome/chrome-for-testing/${CHROME_DRIVER_VERSION}/mac-arm64/chromedriver-mac-arm64.zip"

wget -O chromedriver.zip "$DRIVER_URL"
unzip -o chromedriver.zip -d .
chmod +x chromedriver-mac-arm64/chromedriver-mac-arm64
mv chromedriver-mac-arm64/chromedriver-mac-arm64 ./chromedriver
rm -rf chromedriver.zip chromedriver-mac-arm64

echo "chromedriver for MacOS arm64 downloaded as ./chromedriver."
