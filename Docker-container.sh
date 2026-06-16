#!/bin/bash
docker run -it --name rust-dev \
  -v ${Rust-path-folder}/rst-2026:/workspace \
  --entrypoint /bin/sh \
  ubuntu/rust:1.93-26.04_edge

  ## Note: this image does not include bash, so we start /bin/sh instead.
  ## To start only:

  ## docker start -ai rust-dev