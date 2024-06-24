FROM messense/rust-musl-cross:x86_64-musl as builder

WORKDIR /app

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
LABEL maintainer="Noah Dunbar <noah@noahdunbar.com>"

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/minebot /minebot

ENTRYPOINT [ "minebot" ]
