#!/usr/bin/env bats

SERVER_PORT=12043

setup() {
  target/debug/zas 1>&2 &
  pid="$!"
}

teardown() {
  kill -9 "$pid" 1>&2 || true
}

@test "resolves a .dev domain to 127.0.0.1" {
  response="$(dig app.dev @127.0.0.1 -p $SERVER_PORT +short +retry=0)"

  [ "$response" = "127.0.0.1" ]
}
