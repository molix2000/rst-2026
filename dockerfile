FROM rust:1.67

WORKDIR /usr/src/molix2000
COPY . .

RUN cargo install --path .

CMD ["bash"]
