#!/usr/bin/env bash
# Long-winded file name because this is only really designed for use in tasks.json in VSCode :)

# Launch in a brand new session
# If we launch background with "&", it's terminated when this script is terminated.
setsid bootimage run -- -s -S >&- 2>&- &

# Sleep for a second so VSCode doesn't wipe out qemu
sleep 1