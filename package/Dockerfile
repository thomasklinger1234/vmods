FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y --no-install-recommends \
      curl \
      wget \
      tar \
      gzip \
      jq \
      ca-certificates \
      rpm

RUN curl -s https://packagecloud.io/install/repositories/varnishcache/varnish73/script.deb.sh | bash
RUN apt-get install -y varnish-dev varnish clang libclang-dev

RUN set -e; curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

WORKDIR /mountpoint
ENTRYPOINT ["/mountpoint/package/package.sh"]
