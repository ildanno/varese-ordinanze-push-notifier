FROM ubuntu:focal

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    lsb-release curl build-essential libssl-dev pkg-config && \
    apt-get clean all

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
