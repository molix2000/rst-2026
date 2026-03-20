##FROM rust:1.67
From ubuntu/rust:1.84-25.04_edge
WORKDIR /usr/src/molix2000
COPY . .

RUN cargo install --path ./chapter-3-5/

CMD ["bash"]
