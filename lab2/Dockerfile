FROM ubuntu
FROM gcc:9.2

WORKDIR /usr/app

# COPY . .

RUN apt-get update && apt-get install make

RUN sh scripts/build.sh

ENTRYPOINT [ "./build/bin/lab2.out" ]
