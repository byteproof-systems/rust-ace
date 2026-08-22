# Reference verification environment for rust-ace.
#
# The byte-identity claim is validated on exactly this toolchain
# (Ubuntu 24.04, clang 18, the nightly pinned in crate/rust-toolchain.toml).
# Other environments may well pass, but this one is the verified surface.
#
#   docker build -t rust-ace-verify -f verify/Dockerfile .
#   docker run --rm rust-ace-verify
FROM ubuntu:24.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends \
      clang-18 lld-18 binutils git curl ca-certificates make libc6-dev \
      libstdc++-13-dev \
    && rm -rf /var/lib/apt/lists/* \
    && ln -s /usr/bin/clang-18 /usr/local/bin/clang \
    && ln -s /usr/bin/clang++-18 /usr/local/bin/clang++ \
    && ln -s /usr/bin/clang-18 /usr/local/bin/cc
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
      | sh -s -- -y --default-toolchain none
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /repo
COPY . .
# rustup installs the pinned toolchain from crate/rust-toolchain.toml on
# first cargo invocation inside crate/.
RUN echo "127.0.0.1 $(hostname)" >> /etc/hosts || true
CMD ["bash", "-c", "echo \"127.0.0.1 $(hostname)\" >> /etc/hosts; bash verify/verify.sh"]
