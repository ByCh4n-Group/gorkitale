#!/bin/bash

# Check if ffmpeg is installed
if ! command -v ffmpeg &> /dev/null; then
    echo "ffmpeg could not be found, please install it."
    exit 1
fi

# Create assets directory if it doesn't exist
mkdir -p assets

# Convert Turkish intro
if [ -f "assets/intro_tr.mp4" ]; then
    echo "Converting assets/intro_tr.mp4 to assets/intro_tr.gif..."
    ffmpeg -y -i assets/intro_tr.mp4 -vf "fps=30,scale=480:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" assets/intro_tr.gif
else
    echo "assets/intro_tr.mp4 not found."
fi

# Convert English intro
if [ -f "assets/intro_en.mp4" ]; then
    echo "Converting assets/intro_en.mp4 to assets/intro_en.gif..."
    ffmpeg -y -i assets/intro_en.mp4 -vf "fps=30,scale=480:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" assets/intro_en.gif
else
    echo "assets/intro_en.mp4 not found."
fi

echo "Conversion complete."
