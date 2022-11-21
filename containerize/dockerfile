FROM rust:1.64.0 as builder

# Build the external dependencies
RUN cargo new --bin ccyaa-exporter-build
WORKDIR /ccyaa-exporter-build
COPY ./Cargo.toml ./
RUN cargo build --release
RUN rm src/*.rs

# Build src
ADD . ./
RUN rm ./target/release/deps/ccyaa_order_exporter*
RUN cargo build --release

FROM debian:bullseye-slim as final

RUN mkdir -p /ccyaa-exporter/secrets
WORKDIR /ccyaa-exporter

RUN mkdir ./ejson
ADD https://github.com/Shopify/ejson/releases/download/v1.3.3/ejson_1.3.3_linux_amd64.tar.gz ./
RUN tar -xvzf ejson_1.3.3_linux_amd64.tar.gz -C ./ejson
RUN rm ejson_1.3.3_linux_amd64.tar.gz

COPY ./config/secrets.ejson ./secrets/
COPY --from=builder /ccyaa-exporter-build/target/release/ccyaa_order_exporter ./
COPY ./startup.sh ./startup.sh
RUN chmod +x startup.sh

# Let's Encrypt CA update
RUN apt update && apt install -y ca-certificates

RUN useradd -M appuser
RUN chown -R appuser /ccyaa-exporter

USER appuser
ENTRYPOINT ["./startup.sh"]
