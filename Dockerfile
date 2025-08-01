FROM rust:latest

WORKDIR /code
COPY . .
CMD ["cargo", "test"]