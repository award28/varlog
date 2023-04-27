# Docker Setup

The docker setup is much slower than bare metal, but so long as you have docker
installed and running, it _should_ (🫰) startup with relative ease. Since all of
the workspaces share all the same `Cargo.lock` file, some effort has been put in
to optimizing the projects build structure. Because of this, the docker compose
setup needs two make commands to be run first, and in a specific order.

> ℹ Please be patient while running these commands, as they can take several minutes

1. `make base` ← Creates the base Varlog docker image and tags it as `varlog/base`.
2. `make build` ← Builds the `varlog/registry`, `varlog/server`, and `varlog/app`
containers.

Once these containers have been built, you can run the `make up` command, which will
start the network of compose containers. After docker has finished starting these
containers, then run `make ps` to see the running containers. If there are 5 Varlog 
containers, then congratulations 🎉 you can now visit the UI at
[localhost:8000](http://localhost:8000).
