#!/bin/bash

# Check if libs directory exists
if [ ! -d "libs" ]; then
  echo "Error: libs directory not found."
  exit 1
fi

# Iterate over each subdirectory in libs
for dir in libs/*/; do
  if [ -d "$dir" ]; then
    echo "Building in directory: $dir"
    cd "$dir" || continue

    # Run cargo build --release
    cargo build --release

    # Return to the root directory
    cd - > /dev/null || exit 1
  fi
done

# Here is where the chron job setups will go for other code #
