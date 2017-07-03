#!/bin/bash
for fname in $(
    find . \
      -maxdepth 1 \
      -type d \
      -name "e*"
);
do
    echo "*** $fname ***"
    cd $fname
    cargo run -q
    cd ..
done