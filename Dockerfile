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
    && apt-get install -y ca-certificates fonts-inconsolata curl \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

RUN groupadd scieldas && useradd -g scieldas scieldas && mkdir -p ${APP}

COPY --from=timbrend /scieldas/target/release/scieldas ${APP}/scieldas

ENV ROCKET_CONFIG=${APP}/rocket.toml
ADD ./rocket.toml ${APP}/rocket.toml

RUN chown -R scieldas:scieldas ${APP}

USER scieldas
WORKDIR ${APP}

HEALTHCHECK CMD curl --fail http://0.0.0.0:8000/health || exit 1

CMD ["./scieldas"]
