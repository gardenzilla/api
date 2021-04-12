FROM fedora:33
RUN dnf update -y && dnf clean all -y
WORKDIR /usr/local/bin
COPY ./target/release/api /usr/local/bin/api
# Set server timezone to Budapest timezone
# RUN sudo timedatectl set-timezone Europe/Budapest
STOPSIGNAL SIGINT
ENTRYPOINT ["api"]
