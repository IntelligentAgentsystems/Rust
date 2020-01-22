FROM rust:1.40-slim-buster as builder

WORKDIR /src
# Add rustfmt as tonic uses it to pretty-print source files
RUN rustup component add rustfmt
COPY protos/ protos/
COPY factory_functional_units/ factory_functional_units/
RUN cd factory_functional_units && cargo install --color never --path . --root /app

FROM debian:buster-slim
COPY --from=builder /app/bin/* /usr/local/bin
ENTRYPOINT [ "factory_functional_units" ]