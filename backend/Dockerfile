ARG APP_NAME=d42x-server

# Build Rust

FROM hub.aiursoft.cn/rust:slim AS chef
# FROM hub.aiursoft.cn/rust:latest AS chef
LABEL author="DvorakChen"
LABEL email="dvorakchen@outlook.com"

ARG APP_NAME

RUN  echo "[source.crates-io]\n\ 
    replace-with = 'rsproxy-sparse'\n\
    [source.rsproxy]\n\
    registry = \"https://rsproxy.cn/crates.io-index\"\n\
    [source.rsproxy-sparse]\n\ 
    registry = \"sparse+https://rsproxy.cn/index/\"\n\ 
    [registries.rsproxy]\n\ 
    index = \"https://rsproxy.cn/crates.io-index\"\n\ 
    [net]\n\
    git-fetch-with-cli = true\n" >> $CARGO_HOME/config.toml

RUN cargo install cargo-chef
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS rust-build
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

# RUN

FROM hub.aiursoft.cn/debian:stable-slim AS final

ARG APP_NAME

USER root

COPY --from=rust-build /app/target/release/${APP_NAME} /bin/server

EXPOSE 9876

WORKDIR /bin

CMD ["server", "-m"]