# Running tests

Some tests need to be run against an OpenFGA server. You can start one inside a container with:

```shell
# Cleanup previous instances.
docker rm --force openfga

# Start a new instance and publish ports.
docker run --name openfga -p 35080:8080 -p 35081:8081 -p 35300:3000 openfga/openfga run
```

Then make tests aware of this server by setting the following environment variable.

```shell
export TEST_OPENFGA_CLIENT_GRPC_URL="http://127.0.0.1:35081"
```
