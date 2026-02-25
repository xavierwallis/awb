from rust:latest as builder

workdir /app
copy . .

run cargo build --release


from debian:trixie-slim
workdir /app

# (optional but common) TLS certs for HTTP clients
run apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

copy --from=builder /app/target/release/awb /usr/local/bin/awb

env ROCKET_ADDRESS=0.0.0.0
env ROCKET_PORT=8000

expose 8000
cmd [ "awb" ]
