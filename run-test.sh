#!/bin/bash
script_dir=$(dirname "$0")

export ARCH="${ARCH:-riscv64}"

rm /tmp/output.txt || true

echo "start---riscv64" >>/tmp/output.txt
echo "start---aarch64" >>/tmp/output.txt
echo "start---loongarch64" >>/tmp/output.txt
echo "start---x86_64" >>/tmp/output.txt
make oscomp_run | tee -a /tmp/output.txt
echo "end---riscv64" >>/tmp/output.txt
echo "end---aarch64" >>/tmp/output.txt
echo "end---loongarch64" >>/tmp/output.txt
echo "end---x86_64" >>/tmp/output.txt

"$script_dir/judge.js" /tmp/output.txt
