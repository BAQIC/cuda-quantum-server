# cuda-quantum-server

## How to run
We provide a Dockerfile to run the server. To build the image, please use the following command:
```bash
docker build -t nvidia/cuda-quantum-rust:latest -f Dockerfile .
```

To run the emulate-server, please use the following command:
```bash
docker run -d --network=host --name=cudaq-rust --restart=always nvidia/cuda-quantum-rust:latest
```

Then, you can use `emulate-client` to submit jobs to the server.
