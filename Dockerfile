FROM ubuntu:24.04
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    python3 \
    python3-dev \
    python3-pip \
    g++ \
    cmake \
    pkg-config \
    curl \
    libssl-dev

ENV CARGO_HOME=/root/.cargo
ENV RUSTUP_HOME=/root/.rustup
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

COPY --from=ghcr.io/astral-sh/uv:latest /uv /uv/bin/uv
ENV PATH="/uv/bin:${PATH}"

RUN uv python install 3.12
RUN uv venv /venv --python 3.12

ENV VIRTUAL_ENV=/venv
ENV PATH="/venv/bin:${PATH}"
RUN uv pip install maturin
