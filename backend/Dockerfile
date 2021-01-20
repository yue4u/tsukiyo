# dev
FROM rust as develop
WORKDIR /app
RUN apt-get update \
  && apt-get install -y -q \
     ca-certificates \
     locales \
     libpq-dev \
     gnupg \
     apt-transport-https\
     libssl-dev \
     pkg-config \
     curl \
     build-essential \
     git \
     wget \
  && echo "install rust tools" \
  && cargo install cargo-watch cargo-make diesel_cli \
  && cargo install diesel_cli --no-default-features --features postgres
COPY . .

# build
# see https://github.com/emk/rust-musl-builder/tree/master/examples/using-diesel
FROM ekidd/rust-musl-builder:latest AS builder
ADD --chown=rust:rust . ./
RUN cargo build --release

# production
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/backend \
    /usr/local/bin/
CMD /usr/local/bin/backend