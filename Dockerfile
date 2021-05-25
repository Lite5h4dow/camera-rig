FROM ubuntu:latest

RUN apt-get update && \
    apt-get install \
    --yes \
    binutils \
    curl \
    zip \
    build-essential
    
ARG RASPBERRY_PI_TOOLS_COMMIT_ID=5caa7046982f0539cf5380f94da04b31129ed521
RUN curl -sL https://github.com/raspberrypi/tools/archive/$RASPBERRY_PI_TOOLS_COMMIT_ID.tar.gz \
    | tar xzf - -C /usr/local --strip-components=1 tools-${RASPBERRY_PI_TOOLS_COMMIT_ID}/arm-bcm2708

ENV PATH=/usr/local/arm-bcm2708/arm-linux-gnueabihf/bin:$PATH
ENV PATH=/usr/local/arm-bcm2708/arm-linux-gnueabihf/libexec/gcc/arm-linux-gnueabihf/4.8.3:$PATH

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --verbose
ENV PATH=/root/.cargo/bin:$PATH

RUN rustup target add arm-unknown-linux-gnueabihf
RUN echo '[target.arm-unknown-linux-gnueabihf]\nlinker = "arm-linux-gnueabihf-gcc"' >> /root/.cargo/config

ENV USER=root
WORKDIR /usr/src