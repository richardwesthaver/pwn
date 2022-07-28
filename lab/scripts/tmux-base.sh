#!/usr/bin/env sh
tmux new-session -d -s $1 -n 0
tmux attach -t $1:0
