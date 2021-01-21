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
