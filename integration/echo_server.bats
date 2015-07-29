#!/usr/bin/env bats

SERVER_PORT=12043

setup() {
  target/debug/bam 1>&2 &
  pid="$!"
}

teardown() {
  kill -9 "$pid" 1>&2 || true
}

@test "echoes a udp message back" {
  response="$(echo -n "some_data" | nc -4u -w1 localhost $SERVER_PORT)"

  [ "$response" = "some_data" ]
}
