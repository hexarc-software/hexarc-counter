default:
    @just -l

docker-build:
    @docker build -t my-rust-app .

docker-run:
    @docker run \
        --publish 127.0.0.1:80:8080 \
        --env PORT=8080 \
        -it my-rust-app