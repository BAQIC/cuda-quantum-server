ARG base_image=ghcr.io/nvidia/cuda-quantum-devdeps:ext-gcc12-main
FROM $base_image

ENV CUDAQ_REPO_ROOT=/workspaces/cuda-quantum
ENV CUDAQ_INSTALL_PREFIX=/usr/local/cudaq
ENV PATH="$CUDAQ_INSTALL_PREFIX/bin:${PATH}"
ENV PYTHONPATH="$CUDAQ_INSTALL_PREFIX:${PYTHONPATH}"
ENV PATH="${HOME}/.cargo/bin:${PATH}"
ENV RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
ENV NINJA_THREAD=10

# Change the source of crates.io to TUNA
RUN mkdir ~/.cargo/ && touch ~/.cargo/config \
    && echo '[source.crates-io]' > ~/.cargo/config \
    && echo 'registry = "https://github.com/rust-lang/crates.io-index"'  >> ~/.cargo/config \
    && echo "replace-with = 'tuna'"  >> ~/.cargo/config \
    && echo '[source.tuna]'   >> ~/.cargo/config \
    && echo 'registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"'  >> ~/.cargo/config \
    && echo '[net]'   >> ~/.cargo/config \
    && echo 'git-fetch-with-cli = true'   >> ~/.cargo/config \
    && echo '' >> ~/.cargo/config

WORKDIR /workspace
RUN apt update && apt install curl -y && curl https://mirrors.ustc.edu.cn/misc/rustup-install.sh -sSf | sh -s -- -y

RUN git clone https://github.com/lucky9-cyou/cuda-quantum.git
WORKDIR /workspace/cuda-quantum
RUN git checkout feat/emulate-server
RUN bash scripts/build_cudaq.sh

WORKDIR /workspace
RUN git clone https://github.com/BAQIC/cuda-quantum-server.git
WORKDIR /workspace/cuda-quantum-server
RUN cargo fetch

ENTRYPOINT [ "cargo", "run" ]