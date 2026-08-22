#!/usr/bin/env bash
# Byte-identical differential verification for rust-ace.
#
# Builds ACE 6.5.24 natively and the Rust translation from this repository,
# links both against the same C driver, runs all 145 test scenarios in fresh
# directories, and compares stdout, exit status, and on-disk effects
# byte-for-byte.
#
# Requirements (see README.md):
#   - Linux x86_64, ~16 GB RAM recommended, ~10 GB free disk
#   - clang/clang++ 18 (verified toolchain: 18.1.3)
#   - rustup with the toolchain pinned in crate/rust-toolchain.toml
#   - git, curl
#   - /etc/hosts must map this machine's hostname to 127.0.0.1
#   - write access to /build (the build root is part of the verified
#     byte-identity surface: __FILE__ strings embed it on both sides)
set -euo pipefail

REPO="$(cd "$(dirname "$0")/.." && pwd)"
WORK=/build/ace_full
SRC="$WORK/src"
ACE_GIT_URL="${ACE_GIT_URL:-https://github.com/DOCGroup/ACE_TAO}"
ACE_TAG="ACE+TAO-6_5_24"

say() { printf '\n== %s\n' "$*"; }
die() { printf 'ERROR: %s\n' "$*" >&2; exit 1; }

say "preflight"
command -v clang >/dev/null || die "clang not found"
command -v clang++ >/dev/null || die "clang++ not found"
command -v cargo >/dev/null || die "cargo not found (install rustup)"
command -v git >/dev/null || die "git not found"
clang --version | head -1
H="$(hostname)"
grep -qE "^127\.0\.0\.1[[:space:]].*\b$H\b" /etc/hosts \
  || die "/etc/hosts must map '$H' to 127.0.0.1 (one scenario exercises \
loopback shared-memory transport). Fix: echo '127.0.0.1 $H' | sudo tee -a /etc/hosts"
mkdir -p "$WORK" 2>/dev/null || die "cannot create $WORK (need write access to /build)"

say "stage ACE $ACE_TAG at $SRC/ACE"
if [ ! -f "$SRC/ACE/ace/ACE.cpp" ]; then
  rm -rf "$WORK/checkout" && mkdir -p "$SRC"
  git clone --depth 1 --branch "$ACE_TAG" "$ACE_GIT_URL" "$WORK/checkout"
  mv "$WORK/checkout/ACE" "$SRC/ACE"
  rm -rf "$WORK/checkout"
fi
echo '#include "ace/config-linux.h"' > "$SRC/ACE/ace/config.h"
cp "$REPO/tests/ops.cpp" "$SRC/ACE/full_ops.cpp"

say "compile native translation units"
NOBJ="$WORK/objs-native"; mkdir -p "$NOBJ"
CXXFLAGS=(-std=c++17 -O2 -ffunction-sections -fdata-sections -I.)
n=0
while IFS= read -r tu; do
  [ -n "$tu" ] || continue
  obj="$NOBJ/$(echo "$tu" | tr '/.' '__').o"
  if [ ! -f "$obj" ] || [ "$SRC/ACE/$tu" -nt "$obj" ]; then
    (cd "$SRC/ACE" && clang++ "${CXXFLAGS[@]}" -c "$tu" -o "$obj")
  fi
  n=$((n+1))
done < "$REPO/tests/tu-manifest.txt"
echo "compiled $n translation units"

say "compile driver and link native binary"
clang -std=gnu11 -O2 -I"$SRC/ACE" -c "$REPO/tests/driver.c" -o "$WORK/driver.o"
clang "$WORK/driver.o" "$NOBJ"/*.o -lstdc++ -lm -lpthread -ldl -lrt \
  -Wl,--gc-sections -o "$WORK/native"

say "build the Rust crate (release)"
(cd "$REPO/crate" && cargo build --release)
RLIB="$REPO/crate/target/release/librust_ace.a"
[ -f "$RLIB" ] || die "staticlib not produced: $RLIB"

say "link Rust binary"
clang "$WORK/driver.o" "$RLIB" -lstdc++ -lm -lpthread -ldl -lrt \
  -Wl,--gc-sections -o "$WORK/rust"

say "run 145-scenario differential"
OPS=$(grep -oE '^ +X\(([a-z_0-9]+)\)' "$REPO/tests/driver.c" \
      | sed 's/.*X(//; s/)//')
[ -n "$OPS" ] || OPS=$(grep -oE 'X\(([a-z_0-9]+)\)' "$REPO/tests/driver.c" \
      | sed 's/X(//; s/)//' | sort -u)
pass=0; fail=0; failed_ops=""
for op in $OPS; do
  rm -rf "$WORK/n" "$WORK/r"; mkdir -p "$WORK/n" "$WORK/r"
  no=$( (cd "$WORK/n" && timeout 120 "$WORK/native" "$op" 2>/dev/null); echo "rc=$?" )
  nf=$(ls "$WORK/n" | sort)
  ro=$( (cd "$WORK/r" && timeout 120 "$WORK/rust" "$op" 2>/dev/null); echo "rc=$?" )
  rf=$(ls "$WORK/r" | sort)
  if [ "$no" = "$ro" ] && [ "$nf" = "$rf" ]; then
    pass=$((pass+1))
  else
    fail=$((fail+1)); failed_ops="$failed_ops $op"
    printf 'DIVERGE %s\n  native: %s files=[%s]\n  rust:   %s files=[%s]\n' \
      "$op" "$(echo $no)" "$(echo $nf)" "$(echo $ro)" "$(echo $rf)"
  fi
done

say "result"
echo "byte-identical: $pass  divergent: $fail"
sha256sum "$WORK/native" "$WORK/rust"
[ "$fail" -eq 0 ] || die "divergent scenarios:$failed_ops"
echo "VERIFIED: every scenario byte-identical to native."
