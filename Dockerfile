FROM rust:1.71.1 as build

RUN apt-get update && apt-get install -y libssl-dev openssl zlib1g zlib1g-dev libpq-dev cmake protobuf-compiler netcat curl unzip
WORKDIR /app
COPY . /app
RUN cargo build -r

FROM ubuntu:22.04
ARG RELEASE_FLAG=release

ENV RELEASE_FLAG=${RELEASE_FLAG}
ENV RUST_LOG=info
ENV RUST_BACKTRACE=full
COPY --from=build /app/target/release/rsquery-restapi-server /usr/local/bin/rsquery-restapi-server
CMD ["rsquery-restapi-server"]
