FROM ubuntu:20.04

MAINTAINER Mahesh Vangala <vangalamaheshh@gmail.com>

WORKDIR /usr/local/bin

RUN set -ex \
  && apt-get update -y \
  && apt-get upgrade -y \
  && apt-get install -y build-essential \
  && apt-get install -y curl vim less git \
  # following are the deps for cargo-edit
  && DEBIAN_FRONTEND=noninteractive apt-get install -y pkg-config libssl-dev

RUN set -ex \
  && apt-get update -y 

RUN set -ex \
  && curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH "/root/.cargo/bin:${PATH}"

# install cargo-edit (easy to cargo add <dependencies>)
RUN set -ex \
  && cargo install cargo-edit

WORKDIR /mnt

CMD ["tail", "-f", "/dev/null"]
