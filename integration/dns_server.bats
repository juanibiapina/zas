#!/usr/bin/env bats

DNS_PORT=12043

setup() {
  target/debug/zas 1>&2 &
  pid="$!"
}

teardown() {
  kill -9 "$pid" 1>&2 || true
  wait "$pid" 2>/dev/null || true
}

@test "resolves app.dev domain to 127.0.0.1" {
  response="$(dig app.dev @127.0.0.1 -p $DNS_PORT +short +retry=0)"

  [ "$response" = "127.0.0.1" ]
}

@test "resolves anything_asdfasdf.dev to 127.0.0.1" {
  response="$(dig anything_asdfasdf.dev @127.0.0.1 -p $DNS_PORT +nocomment +retry=0)"

  domain="$(echo "$response" | grep -v "^;" | grep "\.dev" | cut -d "	" -f 1)"
  domain="${domain%.}"
  ip="$(echo "$response" | grep -v "^;" | grep "\.dev" | cut -d "	" -f 5)"

  [ "$domain" = "anything_asdfasdf.dev" ]
  [ "$ip" = "127.0.0.1" ]
}

@test "accepts more than one request" {
  dig app.dev @127.0.0.1 -p $DNS_PORT +nocomment +retry=0
  dig app.dev @127.0.0.1 -p $DNS_PORT +nocomment +retry=0
  dig app.dev @127.0.0.1 -p $DNS_PORT +nocomment +retry=0
  dig app.dev @127.0.0.1 -p $DNS_PORT +nocomment +retry=0
}

@test "does not know any other domains" {
  response="$(dig google.com @127.0.0.1 -p $DNS_PORT +short +retry=0)"

  [ "$response" = "" ]
}
