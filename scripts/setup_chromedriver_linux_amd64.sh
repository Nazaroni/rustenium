#!/usr/bin/env bash
# Downloads latest chromedriver (Linux x64) for local testing

set -e

if ! command -v wget &> /dev/null; then
  echo "wget not found. Install wget first."
  exit 1
fi

CHROME_DRIVER_VERSION=$(wget -qO- https://chromedriver.storage.googleapis.com/LATEST_RELEASE)
DRIVER_URL="https://chromedriver.storage.googleapis.com/${CHROME_DRIVER_VERSION}/chromedriver_linux64.zip"

wget -O chromedriver.zip "$DRIVER_URL"
unzip -o chromedriver.zip -d .
chmod +x chromedriver
rm chromedriver.zip

echo "chromedriver downloaded."
