FROM fedora:33
WORKDIR /usr/local/bin
COPY ./target/release/api /usr/local/bin/api
STOPSIGNAL SIGINT
ENTRYPOINT ["api"]
