#!/usr/bin/env bash
set -e

# Get rustc version info
rustc_info=$(rustc -vV)

# Extract host target triple using a regex match
if [[ $rustc_info =~ host:\ ([^[:space:]]+) ]]; then
  targetTriple="${BASH_REMATCH[1]}"
else
  echo "Failed to determine platform target triple" >&2
  exit 1
fi

echo "Target triple determined: $targetTriple"

# Create the destination folder if it doesn't exist
mkdir -p ./binaries

# Check if we're targeting Windows by matching "windows" in the triple,
# if so copy the .exe, otherwise copy the Linux binary.
if [[ $targetTriple == *"windows"* ]]; then
  cp midway/burrito_out/midway_windows.exe "./binaries/midway-${targetTriple}.exe"
  echo "Copied Windows binary."
else
  cp midway/burrito_out/midway_linux "./binaries/midway-${targetTriple}"
  echo "Copied Linux binary."
fi

echo "File has been copied and renamed successfully."
