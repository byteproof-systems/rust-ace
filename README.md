# rust-ace

ACE 6.5.24, translated to Rust — **byte-identical to native, and you can
verify it yourself**.

This repository contains a Rust translation of the ADAPTIVE Communication
Environment (ACE) 6.5.24: 394 library translation units (~764,000
functions, ~99,000 record types) built as a single Rust static library.
It was produced mechanically by an automated translation system and is
validated by a differential test: the same C driver linked once against
natively-compiled ACE and once against the Rust translation, with 145
scenarios compared **byte-for-byte** on stdout, exit status, and on-disk
effects. See `PROVENANCE.md` for the precise claim and its boundaries.

Don't trust us — run it:

## Quick start (Docker, reference environment)

```
git clone https://github.com/byteproof-systems/rust-ace
cd rust-ace
docker build -t rust-ace-verify -f verify/Dockerfile .
docker run --rm rust-ace-verify
```

Expected final output: `VERIFIED: every scenario byte-identical to native.`

## Native verification (Linux x86_64)

Requirements:
- clang/clang++ 18 (the verified toolchain is 18.1.3 / Ubuntu 24.04)
- rustup (the pinned nightly in `crate/rust-toolchain.toml` installs
  automatically on first build)
- git, ~16 GB RAM recommended, ~10 GB free disk
- write access to `/build` (the build root is part of the byte-compared
  surface — embedded `__FILE__` strings must match on both sides)
- `/etc/hosts` must map your hostname to `127.0.0.1` (one scenario uses
  loopback shared-memory transport):
  `echo "127.0.0.1 $(hostname)" | sudo tee -a /etc/hosts`

Then:

```
bash verify/verify.sh
```

The script clones upstream ACE at tag `ACE+TAO-6_5_24`, builds both
sides, and runs the differential. On the reference machine (4 cores,
14 GB) the full run takes under an hour.

## Using the library

`crate/` builds a C-ABI static library (`librust_ace.a`) exposing ACE's
mangled C++ symbol surface — it links anywhere native ACE objects would,
as `tests/driver.c` demonstrates. It is nightly-Rust, Linux x86_64, and
faithful to ACE semantics including its unsafe idioms; treat it as a
translation of C++ (which it is), not as idiomatic safe Rust.

## Scope and honesty

- Verified surface: Linux x86_64 with the toolchains above. Other
  platforms/compilers are untested and unclaimed.
- 321 of ACE 6.5.24's 394 components execute real API workloads in the
  scenarios; the other 73 contain no executable code on Linux.
- The translation preserves behaviour, not style: the Rust is generated,
  heavily `unsafe`, and regenerated as a unit (see `PROVENANCE.md` —
  hand edits to `crate/src/` will be superseded).

## Contributing

Issues — especially failing scenarios or proposed new differential test
cases — are welcome. Pull requests against generated files under
`crate/src/` cannot be merged meaningfully; PRs to the driver, tests,
verification tooling, and docs are.

## License

DOC License, as inherited from ACE — see `LICENSE` and `NOTICE.md`.
