from rust:latest as builder

workdir /app
copy . .

run cargo build

cmd [ "cargo", "run" ]
