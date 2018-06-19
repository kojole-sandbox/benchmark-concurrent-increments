#!/usr/bin/env bash
set -eux

OUT_DIR="$(dirname $0)/out"
N_INCS=7207200

rm -f $OUT_DIR/*.txt

declare -a kinds=(
  Atomic
  Mutex
  RwLock
  PlMutex
  PlRwLock
  Channel
  ChannelLock
  CChannel
  CChannelLock
)

for kind in "${kinds[@]}"; do
  for i in $(seq 1 16); do
    cargo run --release $kind $i $N_INCS | tee -a "$OUT_DIR/$kind.txt"
  done
done
