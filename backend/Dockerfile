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
FROM develop as build
RUN cargo build --release

# production
FROM rust
COPY --from=build /app/target/release/api .
EXPOSE 4000
CMD ["/usr/local/bin/api"]