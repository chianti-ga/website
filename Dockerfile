FROM rust:1.70 as builder

WORKDIR /srv
COPY . .

RUN apt-get update && apt-get install -y \
    build-essential \
    gcc \
    libssl-dev \
    pkg-config \

RUN rustup target add wasm32-unknown-unknown
RUN rustup target add x86_64-unknown-linux-gnu

RUN cargo install trunk
RUN build.sh


FROM alpine

RUN apk add --no-cache ca-certificates
COPY --from=builder /srv /srv

CMD ["backend"]