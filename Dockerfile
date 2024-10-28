FROM docker.io/library/rust:1.82.0-bullseye AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src \
  && echo "fn main() {}" > src/main \
  && cargo build --release

COPY src/ src/

RUN touch src/main.rs \
  && cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:6f05aba4de16e89f8d879bf2a1364de3e41aba04f1dcbba8c75494f6134b4b13 AS deploy

COPY --from=build /app/target/release/tistimg /tristimg

ENTRYPOINT ["/tristimg"]
