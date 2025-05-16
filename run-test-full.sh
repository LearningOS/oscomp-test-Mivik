#!/bin/bash
script_dir=$(dirname "$0")

export ACCEL=n

rm /tmp/output.txt || true

echo 'Running riscv64'
echo 'start---riscv64' >>/tmp/output.txt
make ARCH=riscv64 oscomp_run >>/tmp/output.txt || true
echo 'end---riscv64' >>/tmp/output.txt

echo 'Running aarch64'
echo 'start---aarch64' >> /tmp/output.txt
make ARCH=aarch64 oscomp_run >>/tmp/output.txt || true
echo 'end---aarch64' >> /tmp/output.txt

echo 'Running loongarch64'
echo 'start---loongarch64' >> /tmp/output.txt
make ARCH=loongarch64 oscomp_run >>/tmp/output.txt || true
echo 'end---loongarch64' >> /tmp/output.txt

echo 'Running x86_64'
echo 'start---x86_64' >> /tmp/output.txt
make ARCH=x86_64 oscomp_run >>/tmp/output.txt || true
echo 'end---x86_64' >> /tmp/output.txt

"$script_dir/judge.js" /tmp/output.txt
