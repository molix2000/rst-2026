#/bin/bash

docker run -it --rm \n  -v . \n  --workdir / \n  ubuntu/rust:1.75-24.04_edge exec 
##\n  cargo build --release
