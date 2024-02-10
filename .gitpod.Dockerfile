FROM gitpod/workspace-full

USER gitpod

RUN sudo apt-get -q update \
    && sudo apt-get install -yq \
        libpython3.6 \
        rust-lldb \
    && sudo rm -rf /var/lib/apt/lists/*
RUN cargo install --locked trunk
RUN cargo install -f wasm-bindgen-cli
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
ENV RUST_LLDB=/usr/bin/lldb-8
