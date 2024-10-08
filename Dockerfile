FROM rust:1.80.1-slim-bookworm as build

WORKDIR /srv

RUN apt-get update && apt-get install -y build-essential gcc libssl-dev pkg-config

RUN rustup target add wasm32-unknown-unknown x86_64-unknown-linux-gnu

RUN cargo install trunk

RUN cargo new --bin backend
RUN cargo new --bin frontend
RUN cargo new --bin shared

# copy manifests for caching
COPY Cargo.toml ./Cargo.toml
COPY backend/Cargo.toml ./backend/Cargo.toml
COPY frontend/Cargo.toml ./frontend/Cargo.toml
COPY backend/build.rs ./backend/build.rs
COPY frontend/build.rs ./frontend/build.rs

COPY frontend/index.html ./frontend/index.html
COPY frontend/assets ./frontend/assets
COPY shared/Cargo.toml ./shared/Cargo.toml

# build steps will cache your dependencies
RUN cargo build --release --package=backend
RUN trunk build --release frontend/index.html

# Remove sample file from cargo new
RUN rm -r ./backend/ ./frontend/ ./shared/

# Copy actual code and ressources
COPY . .

RUN cargo build --release --package=backend
RUN trunk build --release frontend/index.html

#FINAL
FROM gcr.io/distroless/cc-debian12

WORKDIR /srv

# copy the build artifact from the build stage
COPY --from=build /srv/target/release/backend /srv/
COPY --from=build /srv/frontend/dist /srv/dist
COPY config_exemple.json /srv/data/config.json

CMD ["./backend"]
