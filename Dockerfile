FROM docker.io/library/rust:1.80.1-bullseye AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src \
  && echo "fn main() {}" > src/main \
  && cargo build --release

COPY src/ src/

RUN touch src/main.rs \
  && cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:3310655aac0d85eb9d579792387af1ff3eb7a1667823478be58020ab0e0d97a8 AS deploy

COPY --from=build /app/target/release/tistimg /tristimg

ENTRYPOINT ["/tristimg"]
