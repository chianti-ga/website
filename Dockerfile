FROM rust:latest as builder

WORKDIR /srv
COPY . .

RUN apt-get update && apt-get install -y build-essential gcc libssl-dev pkg-config

RUN rustup target add wasm32-unknown-unknown x86_64-unknown-linux-gnu

RUN cargo binstall trunk
RUN build.sh


FROM alpine

RUN apk add --no-cache ca-certificates
COPY --from=builder /srv /srv

CMD ["backend"]