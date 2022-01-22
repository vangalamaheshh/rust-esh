FROM ubuntu:20.04

MAINTAINER Mahesh Vangala <vangalamaheshh@gmail.com>

WORKDIR /usr/local/bin

RUN set -ex \
  && apt-get update -y \
  && apt-get upgrade -y \
  && apt-get install -y build-essential \
  && apt-get install -y curl vim less git

RUN set -ex \
  && apt-get update -y 

RUN set -ex \
  && curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH "/root/.cargo/bin:${PATH}"

WORKDIR /mnt

CMD ["tail", "-f", "/dev/null"]
