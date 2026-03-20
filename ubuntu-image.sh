#/bin/bash

docker run -it --rm   -v .   --workdir / ubuntu/rust:1.75-24.04_edge exec 
##\n  cargo build --release
