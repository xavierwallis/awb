from rust:1-bookworm as builder

workdir /app
copy . .

run cargo build --release


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

copy chrome/profile /app/chrome/profile

env ROCKET_ADDRESS=0.0.0.0
env ROCKET_PORT=8000

expose 8000
cmd [ "awb" ]
