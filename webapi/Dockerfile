# Web APIサーバのビルド
FROM rust:1.81.0-slim as server

WORKDIR /usr/src/myapp
# Web APIサーバもwait-for-itが使えるようにする
RUN apt-get update && \
  apt-get install -y libpq-dev &&\
  apt-get install -y wait-for-it &&\
  cargo install diesel_cli --no-default-features --features postgres

COPY . .

# E2Eテストを実行するクライアントのビルド
FROM rust:1.81.0-slim as client

WORKDIR /usr/src/myapp

RUN apt-get update && \
  apt-get install -y wait-for-it

COPY e2e/ .