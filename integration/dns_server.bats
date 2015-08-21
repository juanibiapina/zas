#!/usr/bin/env bats

load support/test_helper
load support/config

setup() {
  export ZAS_HOME="${FIXTURES_DIR}/zas_home"
  target/debug/zas 1>&2 &
  pid="$!"
}

teardown() {
  pkill -P "$pid" 1>&2 || true
  kill -9 "$pid" 1>&2 || true
  wait "$pid" 2>/dev/null || true
}

@test "dns: resolves app.dev domain to 127.0.0.1" {
  response="$(dig app.dev @127.0.0.1 -p $DNS_PORT +short +retry=0)"

  [ "$response" = "127.0.0.1" ]
}

@test "dns: resolves anything_asdfasdf.dev to 127.0.0.1" {
  response="$(dig anything_asdfasdf.dev @127.0.0.1 -p $DNS_PORT +nocomment +retry=0)"

  domain="$(echo "$response" | grep -v "^;" | grep "\.dev" | cut -d "	" -f 1)"
  domain="${domain%.}"
  ip="$(echo "$response" | grep -v "^;" | grep "\.dev" | cut -d "	" -f 5)"

  [ "$domain" = "anything_asdfasdf.dev" ]
  [ "$ip" = "127.0.0.1" ]
}

@test "dns: accepts more than one request" {
  dig app.dev @127.0.0.1 -p $DNS_PORT +nocomment +retry=0
  dig app.dev @127.0.0.1 -p $DNS_PORT +nocomment +retry=0
  dig app.dev @127.0.0.1 -p $DNS_PORT +nocomment +retry=0
  dig app.dev @127.0.0.1 -p $DNS_PORT +nocomment +retry=0
}

@test "dns: does not know any other domains" {
  response="$(dig google.com @127.0.0.1 -p $DNS_PORT +short +retry=0)"

  [ "$response" = "" ]
}
