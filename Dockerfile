from rust:1.60 as timbrend

RUN USER=root cargo new --bin scieldas
WORKDIR ./scieldas
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/scieldas*
RUN cargo build --release


FROM debian:bookworm-slim
ARG APP=/usr/src/scieldas

RUN apt-get update \
    && apt-get install -y ca-certificates fonts-inconsolata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

RUN groupadd scieldas && useradd -g scieldas scieldas && mkdir -p ${APP}

COPY --from=timbrend /scieldas/target/release/scieldas ${APP}/scieldas

RUN chown -R scieldas:scieldas ${APP}

USER scieldas
WORKDIR ${APP}

CMD ["./scieldas"]
