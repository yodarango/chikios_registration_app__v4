FROM rust:1.56-slim-buster AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:20.04 
WORKDIR /app
COPY --from=build /app/target/release/app .
EXPOSE 3030 
CMD ["./app"]