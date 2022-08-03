default:
    @just -l

docker-build:
    @docker build -t hexarc-tracker .

docker-run:
    @docker run \
        --publish 127.0.0.1:80:8080 \
        --env PORT=8080 \
        -it hexarc-tracker

docker-init-mongo:
    @docker run -d --name local-mongo \
        -p 27017:27017 \
        -e MONGO_INITDB_ROOT_USERNAME=admin \
        -e MONGO_INITDB_ROOT_PASSWORD=pass \
        mongo:latest

docker-restart-mongo:
    @docker restart local-mongo