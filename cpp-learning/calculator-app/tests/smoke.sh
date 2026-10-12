#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

"${CXX:-g++}" -std=c++17 -Wall -Wextra -Wpedantic \
  "$root/src/main.cpp" -o "$tmp/calculator"

output="$(printf '8 + 3\n12 / 4\n2 ^ 8\nsqrt 81\nquit\n' | "$tmp/calculator")"

grep -Fq '= 11.0000' <<< "$output"
grep -Fq '= 3.0000' <<< "$output"
grep -Fq '= 256.0000' <<< "$output"
grep -Fq '= 9.0000' <<< "$output"

echo "Smoke tests passed."
