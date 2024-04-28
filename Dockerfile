FROM rust:latest

WORKDIR /usr/src/crabinfo
COPY . .

RUN cargo install --path .

CMD ["crabinfo"]
