#!/usr/bin/env just --justfile

client:
  cargo run --bin client

server:
  cargo run --bin server
