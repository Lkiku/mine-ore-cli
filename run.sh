#!/bin/bash

for i in {1..6}; do
  tmux new-session -d -s "s-$i" "bash supervision.sh; read"  # 'read' 用于保持会话开启
done
