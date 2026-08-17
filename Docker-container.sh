#!/bin/bash
docker run -it --name rust-dev \
  -v ${Rust-path-folder}/rst-2026:/workspace \
  --entrypoint /bin/sh \
  ubuntu/rust:1.84-25.04_edge

  ## Note: this image does not include bash, so we start /bin/sh instead.
  ## To start only:

  ## docker start -ai rust-dev
  ## Use this to remove old rust images that have the same name rust-dev
  ## docker image prune -a ## This is the cleaning command every restart.
  ## For the newer Rust image I do this as it will not have bash
  ## ONLY uncomment this if you wish to work with the 1.93, be warned there is no bash there
  # docker run -it --name rust-dev \
  # -v ${Rust-path-folder}/rst-2026:/workspace \
  # --entrypoint /bin/sh \
  # ubuntu/rust:1.93-26.04_edge