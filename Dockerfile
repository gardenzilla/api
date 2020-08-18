FROM debian:buster-slim
WORKDIR /usr/local/bin
COPY ./target/release/api /usr/local/bin/api
RUN apt-get update && apt-get install -y
# RUN apt-get install curl -y
CMD ["api"]
# EXPOSE 4200/tcp