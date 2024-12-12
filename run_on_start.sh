#!/bin/bash

# Path to the configuration file
CONFIG_FILE="$(pwd)/configs/config.yml"

# Check if the configuration file exists
if [ ! -f "$CONFIG_FILE" ]; then
    echo "Configuration file $CONFIG_FILE not found! Exiting."
    exit 1
fi

# Read configuration values from the YAML file
LIVE_CAMERA=$(grep '^live_camera:' "$CONFIG_FILE" | awk '{print $2}')
LIVE_AUDIO=$(grep '^live_audio:' "$CONFIG_FILE" | awk '{print $2}')
ADDITIONAL_PROGRAM=$(grep '^additional_program:' "$CONFIG_FILE" | awk '{print $2}')

# Detect system architecture and set paths dynamically
PROJECT_ROOT=$(dirname $(realpath $0))/..
DETECT_MOVEMENT_BINARY="$PROJECT_ROOT/libs/detect_movement/target/release/detect_movement"
AUDIO_DETECTION_SCRIPT="$PROJECT_ROOT/libs/audio_detection/audio_detection.py"

# Function to run live camera service
run_live_camera() {
    echo "Starting live camera service..."
    # Replace with the actual command to start the Rust program for live camera
    ./path/to/rust_camera_program &
}

# Function to run live audio service
run_live_audio() {
    echo "Starting audio detection service..."
    if [ -f "$AUDIO_DETECTION_SCRIPT" ]; then
        python3 "$AUDIO_DETECTION_SCRIPT" &
    else
        echo "Audio detection script not found at $AUDIO_DETECTION_SCRIPT."
    fi
}

# Function to run movement detection
run_detect_movement() {
    echo "Starting movement detection service..."
    if [ -f "$DETECT_MOVEMENT_BINARY" ]; then
        "$DETECT_MOVEMENT_BINARY" &
    else
        echo "Movement detection binary not found at $DETECT_MOVEMENT_BINARY."
    fi
}

# Function to run additional program
run_additional_program() {
    echo "Starting additional program..."
    # Placeholder for user-defined additional program
    # Add your commands here
    echo "(User to define the commands)"
}

# Run services based on configuration values
if [ "$LIVE_CAMERA" = "true" ]; then
    run_detect_movement
else
    echo "Movement detection service is disabled."
fi

if [ "$LIVE_AUDIO" = "true" ]; then
    run_live_audio
else
    echo "Audio detection service is disabled."
fi

if [ "$ADDITIONAL_PROGRAM" = "true" ]; then
    run_additional_program
else
    echo "Additional program is disabled."
fi

# Wait for all background processes to finish
wait

echo "All configured services have been started."
