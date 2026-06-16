#!/bin/bash
docker run -it --name rust-dev \
  -v /Users/morawi/Documents/Rust/rst-2026:/workspace \
  ubuntu/rust:1.93-26.04_edge \
  bash

  ## To start only:

  ## docker start -ai rust-dev