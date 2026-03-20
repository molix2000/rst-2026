##FROM rust:1.67
From ubuntu/rust:1.75-24.04_edge
WORKDIR /usr/src/molix2000
COPY . .

RUN cargo install --path ./test-3-9/

CMD ["bash"]
