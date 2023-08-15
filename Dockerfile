FROM rust:1.71.1 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/rsquery-restapi-server
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/rsquery-restapi-server /usr/local/bin/rsquery-restapi-server

CMD ["rsquery-restapi-server"]