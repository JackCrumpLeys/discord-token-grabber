#!/bin/sh

if ! [ -x "$(command -v cargo)" ]; then
  echo "Cargo is not installed. Please install it :'(">&2
  echo Guide: https://www.rust-lang.org/tools/install
  exit 1
fi

echo Enter your webhook url: 

read WEBHOOK_URL

sed -Ei "s/WEBHOOK_URL_HERE/$WEBHOOK_URL/g" ./src/main.rs

cargo build --release