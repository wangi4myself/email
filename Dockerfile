FROM rust:1.84 AS builder

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release

FROM rust:1.84 AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/email email

COPY configuration configuration 

ENV APP_ENVIRONMENT production

ENTRYPOINT [ "./email" ]