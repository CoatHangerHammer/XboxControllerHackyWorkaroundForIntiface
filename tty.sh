#!/bin/bash

# Change this to match main
FIFO_PATH=$HOME"/Documents/fifo/myfifo"
# This will delete whatever tty you set
# Probably don't use ttyS0 as that seems to be in use sometimes by the system
TTY_PATH="/dev/ttyS31"

# Ensure FIFO exists
if [[ ! -p "$FIFO_PATH" ]]; then
    echo "Creating FIFO at $FIFO_PATH..."
    mkfifo "$FIFO_PATH"
fi

# Start a dummy reader in the background to prevent SIGPIPE
# This keeps the FIFO open even if no other readers exist
cat < "$FIFO_PATH" >/dev/null &
DUMMY_PID=$!
echo "Started dummy FIFO reader (PID $DUMMY_PID)"

# Start socat to mirror PTY → FIFO
echo "Starting socat: $TTY_PATH → $FIFO_PATH"
sudo socat -d -d PTY,link="$TTY_PATH",rawer,mode=666,echo=0 FIFO:"$FIFO_PATH",wronly

# Clean up dummy reader when script exits
echo "Stopping dummy reader..."
kill $DUMMY_PID 2>/dev/null
wait $DUMMY_PID 2>/dev/null

echo "tty_to_fifo.sh stopped."
