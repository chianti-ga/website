FROM rust:latest as build

WORKDIR /srv

RUN apt-get update && apt-get install -y build-essential gcc libssl-dev pkg-config

RUN rustup target add wasm32-unknown-unknown x86_64-unknown-linux-gnu

RUN cargo install trunk

RUN cargo new --bin backend
RUN cargo new --bin frontend
RUN cargo new --bin shared



# copy over your manifests
COPY Cargo.toml /srv/Cargo.toml

COPY backend/Cargo.toml /srv/backend/Cargo.toml

COPY frontend/Cargo.toml /srv/frontend/Cargo.toml
COPY frontend/index.html /srv/frontend/index.html

COPY shared/Cargo.toml /srv/shared/Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release --package=frontend
RUN trunk build --release frontend/index.html

RUN rm -r /srv/backend/src /srv/frontend/src /srv/shared/src

COPY . .

RUN cargo build --release --package=frontend
RUN trunk build frontend/index.html

COPY


# our final base
FROM rust:slim-bookworm

# copy the build artifact from the build stage
COPY --from=build /srv/out_release .

# set the startup command to run your binary
CMD ["./backend"]