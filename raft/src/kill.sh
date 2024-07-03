#!/bin/bash
pids=$(ps aux | grep -i "raft" | grep -v grep | awk '{print $2}')

# Iterate over each PID and kill the process forcefully
for pid in $pids; do
    echo "Killing process with PID: $pid"
    kill -9 $pid
done

echo "All processes containing 'raft' have been killed."

