# Multistage docker build, requires docker 17.05

# builder stage
FROM rust:1.31 as builder

RUN set -ex && \
    apt-get update && \
    apt-get --no-install-recommends --yes install \
    clang \
    libclang-dev \
    llvm-dev \
    libncurses5 \
    libncursesw5 \
    cmake \
    git

WORKDIR /usr/src/kepler

# Copying Kepler
COPY . .

# Building Kepler
RUN cargo build --release

# runtime stage
FROM debian:9.4

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y locales openssl

RUN sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen && \
    dpkg-reconfigure --frontend=noninteractive locales && \
    update-locale LANG=en_US.UTF-8

ENV LANG en_US.UTF-8

COPY --from=builder /usr/src/kepler/target/release/kepler /usr/local/bin/kepler

WORKDIR /root/.kepler

RUN kepler server config && \
    sed -i -e 's/run_tui = true/run_tui = false/' kepler-server.toml

VOLUME ["/root/.kepler"]

EXPOSE 7413 7414 7415 7416

ENTRYPOINT ["kepler"]

CMD ["server", "run"]
