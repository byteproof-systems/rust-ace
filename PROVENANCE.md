# Provenance and verification claim

## What this is

A Rust translation of ACE 6.5.24 (every `ace/*.cpp` translation unit that
compiles standalone on Linux x86_64 — 394 library TUs — plus one test TU),
produced mechanically by an automated translation system and validated
against the original by a byte-identical differential.

## The claim, precisely

For each of the 145 test scenarios in `tests/driver.c` + `tests/ops.cpp`:
running the natively-compiled ACE binary and the Rust-translated binary
with the same argument in fresh working directories produces **identical
stdout bytes, identical exit status, and identical post-run directory
contents**. The scenarios functionally exercise 321 of 394 components with
real API workloads; the remaining 73 contain no executable regions on this
platform.

## Verified surface

- OS/arch: Linux x86_64 (Ubuntu 24.04.4, kernel 6.8)
- Native toolchain: clang/clang++ 18.1.3, flags as encoded in
  `verify/verify.sh`
- Rust toolchain: the nightly pinned in `crate/rust-toolchain.toml`
- Build root: `/build` (embedded `__FILE__` strings are part of the
  compared surface, so both sides must build from the same root)
- Upstream source: `https://github.com/DOCGroup/ACE_TAO` at tag
  `ACE+TAO-6_5_24`, `ACE/` subdirectory, with the one-line
  `ace/config.h` written by the verify script

Environments outside this surface may pass as well but are not covered by
the claim. Run `verify/verify.sh` (or the Dockerfile, which reproduces the
reference environment) to establish the claim on your own hardware.

## What is not claimed

- No claim is made for other platforms, other compilers, other ACE
  versions, or ACE subsystems with no executable code on Linux.
- This is not a supported product and carries no warranty; see `LICENSE`.

## Regeneration policy

The Rust sources under `crate/src/` are generated. Hand edits to them will
be superseded by the next regeneration; behavioural issues are best filed
as issues referencing a failing scenario or a new differential test case.
