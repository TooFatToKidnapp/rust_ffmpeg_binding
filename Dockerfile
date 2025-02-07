FROM debian:latest

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y \
    curl \
    vim \
    libavformat-dev \
    libavcodec-dev \
    libavutil-dev \
    pkg-config \
    libavfilter-dev \
    libavdevice-dev && \
    rm -rf /var/lib/apt/lists/*

ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:$PKG_CONFIG_PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="${HOME}/.cargo/bin:${PATH}"

WORKDIR /app

CMD ["tail", "-f", "/dev/null"]
