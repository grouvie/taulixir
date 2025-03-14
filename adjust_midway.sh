#!/usr/bin/env bash
set -e

#------------------------------------------------------------------------------
# Step 1: Specify the source file to adjust.
#------------------------------------------------------------------------------
SOURCE_FILE="./midway/_build/prod/rel/midway/bin/midway"

BATCH_FILE="./midway/_build/prod/rel/midway/bin/midway.bat"

if [ ! -f "$SOURCE_FILE" ]; then
  echo "Error: File $SOURCE_FILE not found."
  exit 1
fi

if [ ! -f "$BATCH_FILE" ]; then
  echo "Error: File $BATCH_FILE not found."
  exit 1
fi

#------------------------------------------------------------------------------
# Step 2: Determine the target triple using rustc.
#------------------------------------------------------------------------------
rustc_info=$(rustc -vV)

if [[ $rustc_info =~ host:\ ([^[:space:]]+) ]]; then
  targetTriple="${BASH_REMATCH[1]}"
else
  echo "Failed to determine platform target triple" >&2
  exit 1
fi

echo "Target triple determined: $targetTriple"

#------------------------------------------------------------------------------
# Step 3: Decide what file name to use for the new file.
#------------------------------------------------------------------------------
if [[ "$targetTriple" =~ (windows|mingw) ]]; then
  newFilename="midway-${targetTriple}.exe"
else
  newFilename="midway-${targetTriple}"
fi

TARGET_DIR=$(dirname "$SOURCE_FILE")
NEW_FILEPATH="${TARGET_DIR%/}/$newFilename"

#------------------------------------------------------------------------------
# Step 4: Replace the RELEASE_ROOT assignment.
#------------------------------------------------------------------------------

# New replacement line
newAssignment='RELEASE_ROOT="$(CDPATH='' cd "$(dirname "$SELF")/../debug/_root_/rel/midway/" && pwd -P)"'

# Process the file:
# 1. Remove any line that starts with RELEASE_ROOT=
# 2. Insert the new line in the same place
awk -v newLine="$newAssignment" '
    /^RELEASE_ROOT=/ { print newLine; next }
    { print }
' "$SOURCE_FILE" > "$NEW_FILEPATH"

# Preserve the original file permissions
chmod --reference="$SOURCE_FILE" "$NEW_FILEPATH"

echo "New file created: $NEW_FILEPATH"

#------------------------------------------------------------------------------
# Step 5: Replace the RELEASE_ROOT assignment in `midway.bat`
#------------------------------------------------------------------------------
newBatchAssignment='set RELEASE_ROOT=%cd%/debug/_root_/rel/midway/'

awk -v newLine="$newBatchAssignment" '
    /^set RELEASE_ROOT=/ { print newLine; next }
    { print }
' "$BATCH_FILE" > "$BATCH_FILE.tmp"

# Preserve the permissions from the original batch file
chmod --reference="$BATCH_FILE" "$BATCH_FILE.tmp"

# Replace the original batch file with the updated one
mv "$BATCH_FILE.tmp" "$BATCH_FILE"

echo "Updated $BATCH_FILE"
