FROM docker.io/library/rust:1.84.1-bullseye AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src \
  && echo "fn main() {}" > src/main \
  && cargo build --release

COPY src/ src/

RUN touch src/main.rs \
  && cargo build --release

FROM gcr.io/distroless/cc-debian12@sha256:3b75fdd33932d16e53a461277becf57c4f815c6cee5f6bc8f52457c095e004c8 AS deploy

COPY --from=build /app/target/release/tistimg /tristimg

ENTRYPOINT ["/tristimg"]
