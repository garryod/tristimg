FROM docker.io/library/rust:1.82.0-bullseye AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src \
  && echo "fn main() {}" > src/main \
  && cargo build --release

COPY src/ src/

RUN touch src/main.rs \
  && cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:b7550f0b15838de14c564337eef2b804ba593ae55d81ca855421bd52f19bb480 AS deploy

COPY --from=build /app/target/release/tistimg /tristimg

ENTRYPOINT ["/tristimg"]
