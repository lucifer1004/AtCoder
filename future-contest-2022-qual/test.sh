#!/bin/bash

rm -rf tmp.txt score.txt
cargo build --release --bin a
for i in $(seq -f "%04g" 0 999)
do
  ./tester ./target/release/a < ./in/${i}.txt > tmp.txt 2>>score.txt
done
awk '{sum += $3} END {print sum / 1000}' score.txt
