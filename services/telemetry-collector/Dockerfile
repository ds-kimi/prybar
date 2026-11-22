# Build from the repo root: docker build -f services/telemetry-collector/Dockerfile .
FROM rust:1-bookworm AS build
WORKDIR /src
COPY . .
RUN cargo build --release -p telemetry-collector

FROM debian:bookworm-slim
COPY --from=build /src/target/release/telemetry-collector /usr/local/bin/
ENV COLLECTOR_ADDR=0.0.0.0:8080 COLLECTOR_DB=/data/telemetry.db
VOLUME /data
EXPOSE 8080
USER 65534
CMD ["telemetry-collector"]
