export BUILDKIT_PROGRESS=plain

build:
	DOCKER_DEFAULT_PLATFORM=linux/arm64 docker build --rm -t vmod-builder -f package/Dockerfile .
	DOCKER_DEFAULT_PLATFORM=linux/arm64 docker run --rm --mount type=bind,source=$$PWD,target=/mountpoint vmod-builder

dev:
	DOCKER_DEFAULT_PLATFORM=linux/arm64 docker run --rm --mount type=bind,source=$$PWD,target=/mountpoint --entrypoint=/bin/bash -it vmod-builder
