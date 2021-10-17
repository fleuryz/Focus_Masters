FROM rust:1.55

WORKDIR /focus
COPY . .

RUN cargo install --path .

CMD ["focus"]