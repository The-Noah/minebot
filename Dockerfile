FROM messense/rust-musl-cross:x86_64-musl as chef

RUN cargo install cargo-chef

WORKDIR /app

FROM chef as planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
LABEL maintainer="Noah Dunbar <noah@noahdunbar.com>"


COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/minebot /minebot

ENTRYPOINT [ "/minebot" ]
