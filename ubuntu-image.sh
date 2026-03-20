#/bin/bash

docker run -it --rm   -v .   --workdir / ubuntu/rust:1.84-25.04_edge exec 
##\n  cargo build --release
