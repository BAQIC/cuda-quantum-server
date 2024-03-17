FROM ghcr.io/nvidia/cuda-quantum-devdeps:ext-gcc12-main

ENV CUDAQ_REPO_ROOT=/workspaces/cuda-quantum
ENV CUDAQ_INSTALL_PREFIX=/usr/local/cudaq
ENV PATH="$CUDAQ_INSTALL_PREFIX/bin:${PATH}"
ENV PYTHONPATH="$CUDAQ_INSTALL_PREFIX:${PYTHONPATH}"
ENV PATH="${HOME}/.cargo/bin:${PATH}"
ENV RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
ENV NINJA_THREAD=10
ENV EMULATE_ADDR="http://127.0.0.1:3000"

WORKDIR /workspace
RUN git clone https://github.com/lucky9-cyou/cuda-quantum.git && cd cuda-quantum && git checkout feat/emulate-server && bash scripts/build_cudaq.sh

WORKDIR /workspace
RUN apt update && apt install curl -y && curl https://mirrors.ustc.edu.cn/misc/rustup-install.sh -sSf | sh -s -- -y

WORKDIR /workspace/cudaq-agent
COPY . .
RUN cargo build --release && mv target/release/cudaq-agent /bin/cudaq-agent && cargo clean

ENTRYPOINT [ "/bin/cudaq-agent" ]