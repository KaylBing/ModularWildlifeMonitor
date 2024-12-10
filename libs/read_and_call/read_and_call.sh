#!/bin/bash

# Path to the configuration file
CONFIG_FILE="/path/to/config.yml"

# Function to read a value from the YAML config
read_config_value() {
    local key=$1
    grep "^$key:" "$CONFIG_FILE" | awk '{print $2}'
}

# Read the configuration values
LIVE_CAMERA=$(read_config_value "live_camera")
LIVE_AUDIO=$(read_config_value "live_audio")
ADDITIONAL_PROGRAM=$(read_config_value "additional_program")

# Start the live camera if enabled
if [ "$LIVE_CAMERA" == "true" ]; then
    echo "Starting live camera..."
    # Add your command to start the live camera here
    # Example: ffmpeg -f v4l2 -i /dev/video0 ...
fi

# Start the live audio if enabled
if [ "$LIVE_AUDIO" == "true" ]; then
    echo "Starting live audio..."
    # Add your command to start the live audio here
    # Example: arecord -D hw:1,0 ...
fi

# Start the additional program if enabled
if [ "$ADDITIONAL_PROGRAM" == "true" ]; then
    echo "Running additional program..."
    # Add the command to run your program here
    # Example: /path/to/additional_program.sh &
fi

