# Build packages

This directory contains the infrastructure to compile new VMOD release packages. It compiles three artifacts:

- An RPM package for Amazon Linux, RHEL, CentOS, etc.
- A DEB package for Debian, Ubuntu, etc.
- A .tar.gz package for other Linux distributions

## Building in Docker

It's recommended to perform the build in a Docker container using the image provided in the `Dockerfile`, to ensure the necessary dependency versions are in place.

First, build the image:

```shell
docker build -t vmod-builder .
```

Now run the container. It expects the root of this Git repository to be mounted at `/mountpoint` in the container, 
so fill in the source of the bind mount appropriately. 

```shell
docker run --rm --mount type=bind,source=$PWD,target=/mountpoint vmod-builder
```

The container will create an `release` folder in the root of the Git repository containing the build artifacts.



