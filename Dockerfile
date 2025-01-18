FROM rust:latest as build
RUN USER=root cargo new --bin library-server
WORKDIR /library-server

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/library-server*
RUN cargo build --release

FROM rust:latest

COPY --from=build /library-server/target/release/library-server .

CMD ["./library-server""]
