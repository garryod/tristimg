FROM docker.io/library/rust:1.82.0-bullseye AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src \
  && echo "fn main() {}" > src/main \
  && cargo build --release

COPY src/ src/

RUN touch src/main.rs \
  && cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:f913198471738d9eedcd00c0ca812bf663e8959eebff3a3cbadb027ed9da0c38 AS deploy

COPY --from=build /app/target/release/tistimg /tristimg

ENTRYPOINT ["/tristimg"]
