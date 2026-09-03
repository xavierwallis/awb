from rust:1-bookworm as builder
workdir /app

# cache dependency compilation separately from source changes
copy Cargo.toml Cargo.lock ./
copy vendor/ ./vendor/
run mkdir -p src && echo 'fn main() {}' > src/main.rs
run cargo build --release
run rm -f target/release/awb target/release/deps/awb-*

# build the real binary (deps layer stays cached)
copy src/ ./src/
run touch src/main.rs && cargo build --release


from debian:bookworm-slim
workdir /app

run apt-get update && apt-get install -y \
  ca-certificates \
  chromium \
  fonts-liberation \
  libnss3 \
  libatk-bridge2.0-0 \
  libgtk-3-0 \
  libx11-xcb1 \
  libxcomposite1 \
  libxrandr2 \
  libxdamage1 \
  libgbm1 \
  libasound2 \
  libxshmfence1 \
  libdrm2 \
  && rm -rf /var/lib/apt/lists/*

copy --from=builder /app/target/release/awb /usr/local/bin/awb

# chrome profile is mounted as a volume at runtime — create empty dir as mount point
run mkdir -p /app/chrome/profile

env ROCKET_ADDRESS=0.0.0.0
env ROCKET_PORT=8000

expose 8000
cmd [ "awb" ]
