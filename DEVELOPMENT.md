# Integration Tests

Some tests need to be run against an OpenFGA server. You can start one inside a container and execute tests with:

```bash
docker rm --force openfga-client && docker run -d --name openfga-client -p 36080:8080 -p 36081:8081 -p 36300:3000 openfga/openfga:v1.8 run

export TEST_OPENFGA_CLIENT_GRPC_URL="http://localhost:36081"
just test
```

